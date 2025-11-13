use std::ops::Index;

#[derive(Clone, Debug)]
pub struct Chromosome {
    genes: Vec<f32>,
}

impl Chromosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.genes.len() * 4);
        for &w in &self.genes {
            out.extend_from_slice(&w.to_le_bytes());
        }
        out
    }
}

// ---
// | this is the type of expression you expect inside the square brackets
// |
// | e.g. if you implemented `Index<&str>`, you could write:
// |   chromosome["yass"]
// ------- v---v
impl Index<usize> for Chromosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

// ---
// | this is the type of the item an iterator should provide in order to be compatible
// | with our chromosome
// |
// | (sometimes it's called the type an iterator *yields*)
// |
// | intuitively, since our chromosome is built of of floating-point numbers, we
// | expect floating-point numbers in here as well
// -------------- v-v
impl FromIterator<f32> for Chromosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for Chromosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}

#[cfg(test)]
impl PartialEq for Chromosome {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
    }
}
