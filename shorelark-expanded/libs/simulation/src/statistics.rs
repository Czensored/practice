use crate::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub generation: usize,
    pub ga: ga::Statistics,
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "generation {}, min={:.2}, max={:.2}, avg={:.2}",
            self.generation,
            self.ga.min_fitness,
            self.ga.max_fitness,
            self.ga.avg_fitness,
        )
    }
}
