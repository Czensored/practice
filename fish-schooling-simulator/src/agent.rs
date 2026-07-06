use crate::vector::{Vec2, Vec3};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Fish2d {
    pub(crate) position: Vec2,
    pub(crate) velocity: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Shark2d {
    pub(crate) position: Vec2,
    pub(crate) velocity: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SharkTarget2d {
    pub(crate) position: Vec2,
    pub(crate) crowding: usize,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Fish3d {
    pub(crate) position: Vec3,
    pub(crate) velocity: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Shark3d {
    pub(crate) position: Vec3,
    pub(crate) velocity: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SharkTarget3d {
    pub(crate) position: Vec3,
    pub(crate) crowding: usize,
}
