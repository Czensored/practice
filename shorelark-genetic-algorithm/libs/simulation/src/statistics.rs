use crate::*;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub generation: usize,
    pub ga: ga::Statistics,
}
