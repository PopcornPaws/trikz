use crate::{Scalar, Vector2};

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
        self.southwest() + Vector2::new(-xshift, -yshift)
    }
    fn below_right(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.southeast() + Vector2::new(xshift, -yshift)
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

impl AnchorT for Vector2 {
    fn anchor(&self, anchor: Anchor) -> Self {
        let default_radius = 5.0;
        let (radius, angle) = match anchor {
            Anchor::Origin => return *self,
            Anchor::North => (default_radius, 90.0),
            Anchor::East => (default_radius, 0.0),
            Anchor::South => (default_radius, -90.0),
            Anchor::West => (default_radius, 180.0),
            Anchor::NorthEast => (default_radius, 45.0),
            Anchor::SouthEast => (default_radius, -45.0),
            Anchor::SouthWest => (default_radius, -135.0),
            Anchor::NorthWest => (default_radius, 135.0),
            Anchor::Polar { radius, angle } => (radius, angle),
        };

        self + crate::polar_coordinates(radius, angle)
    }
}

fn polar_coordinates(radius: Scalar, angle: Scalar) -> Vector2 {
    let radians = angle * crate::PI / 180.0;
    let (s, c) = radians.sin_cos();
    Vector2::new(radius * c, radius * s)
}

#[cfg(test)]
mod test {
    #[test]
    fn polar_coordinates() {
        assert!((super::polar_coordinates(2.0, 0.0) - 2.0 * Vector2::x()).norm() < 1e-6);
        assert!((super::polar_coordinates(3.0, 90.0) - 3.0 * Vector2::y()).norm() < 1e-6);
        assert!((super::polar_coordinates(2.0, 180.0) + 2.0 * Vector2::x()).norm() < 1e-6);
        assert!((super::polar_coordinates(3.0, -90.0) + 3.0 * Vector2::y()).norm() < 1e-6);
    }
}
