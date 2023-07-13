use crate::{Scalar, Vector2};

pub trait AnchorT {
    fn anchor(&self, anchor: Anchor) -> Vector2;
    fn origin(&self) -> Vector2 {
        self.anchor(Anchor::Origin)
    }
    fn above(&self, xshift: Scalar) -> Vector2 {
        self.north() + Vector2::new(xshift, 0.0)
    }
    fn below(&self, xshift: Scalar) -> Vector2 {
        self.north() - Vector2::new(xshift, 0.0)
    }
    fn left(&self, yshift: Scalar) -> Vector2 {
        self.west() - Vector2::new(0.0, yshift)
    }
    fn right(&self, yshift: Scalar) -> Vector2 {
        self.east() + Vector2::new(0.0, yshift)
    }
    fn north(&self) -> Vector2 {
        self.anchor(Anchor::North)
    }
    fn above_left(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.northwest() + Vector2::new(xshift, -yshift)
    }
    fn above_right(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.northeast() + Vector2::new(xshift, yshift)
    }
    fn below_left(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.west() + Vector2::new(-xshift, -yshift)
    }
    fn below_right(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.east() + Vector2::new(-xshift, yshift)
    }
    fn northeast(&self) -> Vector2 {
        self.anchor(Anchor::NorthEast)
    }
    fn east(&self) -> Vector2 {
        self.anchor(Anchor::East)
    }
    fn southeast(&self) -> Vector2 {
        self.anchor(Anchor::SouthEast)
    }
    fn south(&self) -> Vector2 {
        self.anchor(Anchor::South)
    }
    fn southwest(&self) -> Vector2 {
        self.anchor(Anchor::SouthWest)
    }
    fn west(&self) -> Vector2 {
        self.anchor(Anchor::West)
    }
    fn northwest(&self) -> Vector2 {
        self.anchor(Anchor::NorthWest)
    }
}

pub enum Anchor {
    Origin,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Polar { radius: Scalar, angle: Scalar },
}

