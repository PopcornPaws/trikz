use crate::{Scalar, Vector2};

// positive X is right (east)
// positive Y is up (north)
pub trait AnchorT {
    fn anchor(&self, anchor: Anchor) -> Vector2;
    fn origin(&self) -> Vector2 {
        self.anchor(Anchor::Origin)
    }
    fn above(&self, yshift: Scalar) -> Vector2 {
        self.north() + Vector2::new(0.0, yshift)
    }
    fn below(&self, yshift: Scalar) -> Vector2 {
        self.south() - Vector2::new(0.0, yshift)
    }
    fn left(&self, xshift: Scalar) -> Vector2 {
        self.west() - Vector2::new(xshift, 0.0)
    }
    fn right(&self, xshift: Scalar) -> Vector2 {
        self.east() + Vector2::new(xshift, 0.0)
    }
    fn above_left(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.northwest() + Vector2::new(-xshift, yshift)
    }
    fn above_right(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.northeast() + Vector2::new(xshift, yshift)
    }
    fn below_left(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.west() + Vector2::new(-xshift, -yshift)
    }
    fn below_right(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.east() + Vector2::new(xshift, -yshift)
    }
    fn north(&self) -> Vector2 {
        self.anchor(Anchor::North)
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

