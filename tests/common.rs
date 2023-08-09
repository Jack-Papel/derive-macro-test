use std::fmt::Debug;

#[derive(Debug)]
pub struct Transform(pub f32, pub f32);

pub trait Transformable {
    fn transform(&self) -> &Transform;
    fn transform_mut(&mut self) -> &mut Transform;
}

pub trait Entity: Transformable + Debug {}

impl<T> Entity for T where T: Transformable + Debug {}
