#![allow(dead_code)]

use rand::Rng;
use rdxsort::*;
use std::time::Instant;

/// Sorts an array containing elements with three distinct values using the Dutch National Flag algorithm.
///
/// This function implements the Dutch National Flag algorithm, which sorts an array containing elements with three distinct values
/// into three sections: elements less than a pivot value, elements equal to the pivot value, and elements greater than the pivot value.
///
/// # Arguments
///
/// * `a` - Mutable reference to a slice of signed integers (`&mut [isize]`) representing the array to be sorted.
/// * `size` - Size of the slice `a`.
fn dutch_flag(a: &mut [isize], size: usize) {
    let mut left = 0;
    let mut mid = 0;
    let mut right = size - 1;
    let mut temp;

    while mid <= right {
        if a[mid] < 1 {
            temp = a[mid];
            a[mid] = a[left];
            a[left] = temp;
            left += 1;
            mid += 1;
        } else if a[mid] > 1 {
            temp = a[mid];
            a[mid] = a[right];
            a[right] = temp;
            right -= 1;
        } else {
            mid += 1;
        }
    }
}

/// Runs the Dutch National Flag algorithm to sort an array containing elements with three distinct values.
fn run_dutch_flag() {
    const ARR_LEN: usize = 10;

    let mut rng = rand::thread_rng();
    let mut array = [0; ARR_LEN];

    for i in 0..ARR_LEN {
        array[i] = rng.gen_range(0..=2);
    }

    println!("Randomly generated array: {:?}", array);
    dutch_flag(&mut array, ARR_LEN);
    println!("Sorted array: {:?}", array);
}

/// Performs an in-place partitioning of elements in a mutable slice based on a pivot element.
///
/// # Arguments
///
/// * `a` - Mutable reference to a slice of integers (`&mut [i32]`) representing the array to be partitioned.
/// * `left` - Index of the leftmost element of the slice.
/// * `right` - Index of the rightmost element of the slice.
///
/// # Returns
///
/// Returns the index of the pivot element after partitioning.
///
/// # Behavior
///
/// The function rearranges the elements of the slice `a` such that all elements less than or equal to
/// the pivot element are moved to its left, and all elements greater than the pivot are moved to its right.
/// It chooses the pivot element as the median of the first, middle, and last elements of the slice (`a[(left + right) / 2]`).
/// It utilizes a two-pointer approach (`l_spot` and `r_spot`) to traverse the slice from both ends towards the center,
/// swapping elements when necessary to maintain the partitioning condition.
/// The function modifies the slice `a` in place.
fn impr_partition(a: &mut [i32], left: usize, right: usize) -> usize {
    let pivot_index = (left + right) / 2;
    let mut temp = a[left];
    a[left] = a[pivot_index];
    a[pivot_index] = temp;

    let pivot = a[left];
    let mut l_spot = left + 1;
    let mut r_spot = right;

    while l_spot <= r_spot {
        while (l_spot <= r_spot) && (a[r_spot] >= pivot) {
            r_spot -= 1;
        }
        while (l_spot <= r_spot) && (a[l_spot] <= pivot) {
            l_spot += 1;
        }

        if l_spot < r_spot {
            temp = a[l_spot];
            a[l_spot] = a[r_spot];
            a[r_spot] = temp;
            l_spot += 1;
            r_spot -= 1;
        }
    }

    temp = a[left];
    a[left] = a[r_spot];
    a[r_spot] = temp;
    r_spot
}

// There is no performance impact between these two functions (that I could find)

// fn improved_quicksort_helper(a: &mut [i32], size: usize, left: isize, right: isize) {
//     if left > right {
//         return;
//     }
//     let mid = impr_partition(a, left as usize, right as usize) as isize;
//     improved_quicksort_helper(a, size, left, mid - 1);
//     improved_quicksort_helper(a, size, mid + 1, right);
// }

// fn improved_quicksort(a: &mut [i32], size: usize) {
//     improved_quicksort_helper(a, size, 0, (size - 1) as isize);
// }


/// Sorts a mutable slice of integers using the QuickSort algorithm.
///
/// # Arguments
///
/// * `a` - Mutable reference to a slice of integers (`&mut [i32]`) representing the array to be sorted.
/// * `size` - Size of the slice `a`.
fn improved_quicksort(a: &mut [i32], size: usize) {
    if size <= 1 {
        return;
    }

    let pivot_index = impr_partition(a, 0, size - 1);

    improved_quicksort(&mut a[0..pivot_index], pivot_index);
    improved_quicksort(&mut a[pivot_index + 1..], size - pivot_index - 1);
}

/// Sorts a mutable slice of integers using the Insertion Sort algorithm.
///
/// # Arguments
///
/// * `a` - Mutable reference to a slice of integers (`&mut [i32]`) representing the array to be sorted.
/// * `size` - Size of the slice `a`.
fn insertion_sort(a: &mut [i32], size: usize) {
    for i in 1..size {
        let val = a[i];
        let mut j = i;

        while j > 0 && a[j - 1] > val {
            a[j] = a[j - 1];
            j -= 1;
        }

        a[j] = val;
    }
}

/// Sorts a mutable slice of integers using a hybrid of Insertion Sort and Improved QuickSort algorithms.
///
/// This function implements a hybrid sorting algorithm that switches between Insertion Sort and Improved QuickSort
/// based on the size of the array being sorted. For small arrays (size <= `max_arr_size`), it uses Insertion Sort,
/// while for larger arrays, it uses Improved QuickSort for improved performance.
///
/// # Arguments
///
/// * `a` - Mutable reference to a slice of integers (`&mut [i32]`) representing the array to be sorted.
/// * `size` - Size of the slice `a`.
/// * `max_arr_size` - Maximum size of the array for which Insertion Sort will be used.
///
/// # Behavior
///
/// The function sorts the elements of the slice `a` in non-decreasing order using a hybrid of Insertion Sort and Improved QuickSort algorithms.
/// - For arrays with size less than or equal to `max_arr_size`, it uses Insertion Sort for sorting.
/// - For arrays larger than `max_arr_size`, it partitions the array using the `impr_partition` function and then sorts the subarrays using Improved QuickSort.
/// - The function modifies the slice `a` in place.
///
/// # See Also
///
/// - [`insertion_sort`](fn.insertion_sort.html): Insertion Sort algorithm used for small arrays.
/// - [`improved_quicksort`](fn.improved_quicksort.html): Improved QuickSort algorithm.
fn hybrid_quicksort(a: &mut [i32], size: usize, max_arr_size: usize) {
    if size <= 1 {
        return;
    }

    if size <= max_arr_size {
        insertion_sort(a, size)
    } else {
        let pivot_index = impr_partition(a, 0, size - 1);
        improved_quicksort(&mut a[0..pivot_index], pivot_index);
        improved_quicksort(&mut a[pivot_index + 1..], size - pivot_index - 1);
    }
}

fn check_asc_values(a: &[i32], n: usize) -> &str {
    for i in 0..n {
        if a[i] != 2 * i as i32 {
            // println!("Mismatch at index {}: Expected {}, Actual {}", i, 2 * i, a[i]);
            return "NO";
        }
    }
    "ok"
}

fn check_ascending(a: &[i32], size: usize) -> &str {
    for i in 0..(size - 1) {
        if a[i] > a[i + 1] {
            return "NO";
        }
    }
    "ok"
}

fn print_results<F>(
    algorithm: &str,
    size: usize,
    asc: &[i32],
    ran: &[i32],
    des: &[i32],
    sort_fn: F
) where
    F: Fn(&mut Vec<i32>, usize),
{
    let mut temp_asc = asc.to_owned();
    let mut temp_ran = ran.to_owned();
    let mut temp_des = des.to_owned();

    print!("{:>20} {:>10}", algorithm, size);

    let start_time = Instant::now();
    sort_fn(&mut temp_asc, size);
    let elapsed_time = start_time.elapsed().as_secs_f64();
    print!("{:>10.2} {}", elapsed_time, check_asc_values(&temp_asc, size));

    let start_time = Instant::now();
    sort_fn(&mut temp_ran, size);
    let elapsed_time = start_time.elapsed().as_secs_f64();
    print!("{:>10.2} {}", elapsed_time, check_ascending(&temp_ran, size));

    let start_time = Instant::now();
    sort_fn(&mut temp_des, size);
    let elapsed_time = start_time.elapsed().as_secs_f64();
    print!("{:>10.2} {}", elapsed_time, check_asc_values(&temp_des, size));

    println!();
}

fn test_different_depth_limits() {
    println!("{:>20} {:>10} {:>16} {:>13} {:>16}", "depth_limit", "Size", "Ascending Order", "Random Order", "Descending Order");
    let mut size = 40000;
    while size <= 40960000 {
        let mut asc: Vec<i32> = Vec::with_capacity(size);
        let mut ran: Vec<i32> = Vec::with_capacity(size);
        let mut des: Vec<i32> = Vec::with_capacity(size);

        for i in 0..size {
            asc.push(2 * i as i32);
            ran.push(rand::thread_rng().gen::<i32>());
            des.push(2 * (size - i - 1) as i32);
        }

        for depth_limit in 3..=12 {
            let mut temp_asc = asc.clone();
            let mut temp_ran = ran.clone();
            let mut temp_des = des.clone();

            print!("{:>20} {:>10}", depth_limit, size);

            let start_time = Instant::now();
            hybrid_quicksort(&mut temp_asc, size, depth_limit);
            let elapsed_time = start_time.elapsed().as_secs_f64();
            print!("{:>10.2} {}", elapsed_time, check_asc_values(&temp_asc, size));

            let start_time = Instant::now();
            hybrid_quicksort(&mut temp_ran, size, depth_limit);
            let elapsed_time = start_time.elapsed().as_secs_f64();
            print!("{:>10.2} {}", elapsed_time, check_ascending(&temp_ran, size));

            let start_time = Instant::now();
            hybrid_quicksort(&mut temp_des, size, depth_limit);
            let elapsed_time = start_time.elapsed().as_secs_f64();
            print!("{:>10.2} {}", elapsed_time, check_asc_values(&temp_des, size));

            println!();
        }

        println!();
        size *= 2;
    }
}

fn test_vs_sorts() {
    const DEPTH_LIMIT: usize = 10;
    println!("{:>20} {:>10} {:>16} {:>13} {:>16}", "Algorithm", "Size", "Ascending Order", "Random Order", "Descending Order");

    let mut size = 640000;
    // 40960000
    // 5120000
    while size <= 40960000 {
        let mut asc: Vec<i32> = Vec::with_capacity(size);
        let mut ran: Vec<i32> = Vec::with_capacity(size);
        let mut des: Vec<i32> = Vec::with_capacity(size);

        for i in 0..size {
            asc.push(2 * i as i32);
            ran.push(rand::thread_rng().gen::<i32>());
            des.push(2 * (size - i - 1) as i32);
        }

        print_results("Improved QuickSort", size, &asc, &ran, &des, |arr, s| improved_quicksort(arr, s));
        print_results("Hybrid Quicksort", size, &asc, &ran, &des, |arr, s| hybrid_quicksort(arr, s, DEPTH_LIMIT));
        print_results("Built-In Sort", size, &asc, &ran, &des, |arr, _| arr.sort());
        print_results("BI Sort_unstable", size, &asc, &ran, &des, |arr, _| arr.sort_unstable());
        print_results("Radix Sort", size, &asc, &ran, &des, |arr, _| arr.rdxsort());

        println!();
        size *= 2;
    }
}

fn main() {
    // run_dutch_flag();
    // test_different_depth_limits();
    test_vs_sorts();
}
