use crate::vector::Vec2;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Fish {
    pub(crate) position: Vec2,
    pub(crate) velocity: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Shark {
    pub(crate) position: Vec2,
    pub(crate) velocity: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SharkTarget {
    pub(crate) position: Vec2,
    pub(crate) crowding: usize,
}
