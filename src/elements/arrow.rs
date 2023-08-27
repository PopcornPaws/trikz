use super::{Element, ReprT};
use crate::svgutils::keys;
use crate::Vector2;
use super::path::PathBuilder;

pub struct Arrow;

impl ReprT for Arrow {
    type Repr = crate::style::Stroke;
}

impl Element<Arrow> {
    pub fn straight(start: Vector2, end: Vector2) -> Self {
        todo!();
    }

    pub fn elbow(start: Vector2, end: Vector2) -> Self {
        todo!();
    }

    pub fn double_elbow(start: Vector2, mid: Vector2, end: Vector2) -> Self {
        todo!();
    }
}
