use num_integer::Roots;

fn create_sieve(n: usize) -> Vec<bool> {
    let mut sieve = vec![true; n + 1];

    sieve[0] = false;
    if n >= 1 {
        sieve[1] = false;
    }

    for i in 2..=n.sqrt() as usize {
        if sieve[i] {
            for j in (i * i..=n).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    sieve
}

fn get_primes_up_to(n: usize) -> Vec<usize> {
    let sieve = create_sieve(n);
    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &is_prime)| if is_prime { Some(i) } else { None })
        .collect()
}

pub trait SieveOfEratosthenes {
    /// Trait providing the Sieve of Eratosthenes algorithm for finding prime numbers.
    ///
    /// This trait defines a method for computing all prime numbers up to and including `self`,
    /// using the classic Sieve of Eratosthenes algorithm.
    ///
    /// # Returns
    ///
    /// A `Vec<usize>` containing all prime numbers less than or equal to the input value.
    ///
    /// # Examples
    ///
    /// ```
    /// use primality_utils::SieveOfEratosthenes;
    ///
    /// let primes = 29usize.sieve_of_eratosthenes();
    /// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    /// ```
    ///
    /// # Notes
    ///
    /// - The algorithm is efficient for moderate-sized inputs (`n < 10^7`).
    /// - For very large ranges or big integers, consider a segmented sieve or probabilistic tests.
    fn sieve_of_eratosthenes(&self) -> Vec<usize>;
}


impl SieveOfEratosthenes for usize {
    fn sieve_of_eratosthenes(&self) -> Vec<usize> {
        get_primes_up_to(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes_up_to_10() {
        let primes = 10.sieve_of_eratosthenes();
        assert_eq!(primes, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_primes_up_to_1() {
        let primes = 1.sieve_of_eratosthenes();
        assert_eq!(primes, vec![]);
    }

    #[test]
    fn test_primes_up_to_29() {
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let actual = 29.sieve_of_eratosthenes();
        assert_eq!(actual, expected);
    }
}
