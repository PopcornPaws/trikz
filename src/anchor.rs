use crate::{Scalar, Vector2};

const DEFAULT_RADIUS: Scalar = 5.0;

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
// positive Y is down (south)
pub trait AnchorT {
    fn anchor(&self, anchor: Anchor) -> Vector2;
    fn origin(&self) -> Vector2 {
        self.anchor(Anchor::Origin)
    }
    fn above(&self, yshift: Scalar) -> Vector2 {
        self.north() - Vector2::new(0.0, yshift)
    }
    fn below(&self, yshift: Scalar) -> Vector2 {
        self.south() + Vector2::new(0.0, yshift)
    }
    fn left(&self, xshift: Scalar) -> Vector2 {
        self.west() - Vector2::new(xshift, 0.0)
    }
    fn right(&self, xshift: Scalar) -> Vector2 {
        self.east() + Vector2::new(xshift, 0.0)
    }
    fn above_left(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.northwest() + Vector2::new(-xshift, -yshift)
    }
    fn above_right(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.northeast() + Vector2::new(xshift, -yshift)
    }
    fn below_left(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.southwest() + Vector2::new(-xshift, yshift)
    }
    fn below_right(&self, xshift: Scalar, yshift: Scalar) -> Vector2 {
        self.southeast() + Vector2::new(xshift, yshift)
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
        anchor_circle(anchor, *self, DEFAULT_RADIUS)
    }
}

fn polar_coordinates(radius: Scalar, angle: Scalar) -> Vector2 {
    let radians = angle * crate::PI / 180.0;
    let (s, c) = radians.sin_cos();
    Vector2::new(radius * c, radius * s)
}

pub fn anchor_circle(anchor: Anchor, origin: Vector2, radius: Scalar) -> Vector2 {
    // positive X is right (east)
    // positive Y is down (south)
    let (radius, angle) = match anchor {
        Anchor::Origin => return origin,
        Anchor::North => (radius, -90.0),
        Anchor::East => (radius, 0.0),
        Anchor::South => (radius, 90.0),
        Anchor::West => (radius, 180.0),
        Anchor::NorthEast => (radius, -45.0),
        Anchor::SouthEast => (radius, 45.0),
        Anchor::SouthWest => (radius, 135.0),
        Anchor::NorthWest => (radius, -135.0),
        Anchor::Polar { radius, angle } => (radius, angle),
    };

    origin + polar_coordinates(radius, angle)
}

pub fn anchor_rectangle(
    anchor: Anchor,
    origin: Vector2,
    half_width: Scalar,
    half_height: Scalar,
) -> Vector2 {
    // positive X is right (east)
    // positive Y is down (south)
    let shift = match anchor {
        Anchor::Origin => return origin,
        Anchor::North => -half_height * Vector2::y(),
        Anchor::NorthEast => Vector2::new(half_width, -half_height),
        Anchor::East => half_width * Vector2::x(),
        Anchor::SouthEast => Vector2::new(half_width, half_height),
        Anchor::South => half_height * Vector2::y(),
        Anchor::SouthWest => Vector2::new(-half_width, half_height),
        Anchor::West => -half_width * Vector2::x(),
        Anchor::NorthWest => Vector2::new(-half_width, -half_height),
        Anchor::Polar { radius, angle } => polar_coordinates(radius, angle),
    };
    origin + shift
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_relative_eq;

    #[test]
    fn polar() {
        assert_relative_eq!(polar_coordinates(2.0, 0.0), 2.0 * Vector2::x());
        assert_relative_eq!(polar_coordinates(3.0, 90.0), 3.0 * Vector2::y());
        assert_relative_eq!(polar_coordinates(2.0, 180.0), -2.0 * Vector2::x());
        assert_relative_eq!(polar_coordinates(3.0, -90.0), -3.0 * Vector2::y());
    }

    #[test]
    fn circle() {
        let radius = 10.0;
        let (s, c) = (crate::PI / 4.0).sin_cos(); //45 degrees
        let yr = s * radius;
        let xr = c * radius;
        let origin = Vector2::zeros();

        assert_relative_eq!(
            anchor_circle(Anchor::Origin, origin, radius),
            Vector2::zeros()
        );
        assert_relative_eq!(
            anchor_circle(Anchor::North, origin, radius),
            Vector2::new(0.0, -radius)
        );
        assert_relative_eq!(
            anchor_circle(Anchor::NorthEast, origin, radius),
            Vector2::new(xr, -yr)
        );
        assert_relative_eq!(
            anchor_circle(Anchor::East, origin, radius),
            Vector2::new(radius, 0.0)
        );
        assert_relative_eq!(
            anchor_circle(Anchor::SouthEast, origin, radius),
            Vector2::new(xr, yr)
        );
        assert_relative_eq!(
            anchor_circle(Anchor::South, origin, radius),
            Vector2::new(0.0, radius)
        );
        assert_relative_eq!(
            anchor_circle(Anchor::SouthWest, origin, radius),
            Vector2::new(-xr, yr)
        );
        assert_relative_eq!(
            anchor_circle(Anchor::West, origin, radius),
            Vector2::new(-radius, 0.0)
        );
        assert_relative_eq!(
            anchor_circle(Anchor::NorthWest, origin, radius),
            Vector2::new(-xr, -yr)
        );
    }

    #[test]
    fn rectangle() {
        let origin = Vector2::zeros();
        let half_width = 4.0;
        let half_height = 3.0;

        assert_relative_eq!(
            anchor_rectangle(Anchor::Origin, origin, half_width, half_height),
            Vector2::zeros()
        );
        assert_relative_eq!(
            anchor_rectangle(Anchor::North, origin, half_width, half_height),
            Vector2::new(0.0, -3.0)
        );
        assert_relative_eq!(
            anchor_rectangle(Anchor::NorthEast, origin, half_width, half_height),
            Vector2::new(4.0, -3.0)
        );
        assert_relative_eq!(
            anchor_rectangle(Anchor::East, origin, half_width, half_height),
            Vector2::new(4.0, 0.0)
        );
        assert_relative_eq!(
            anchor_rectangle(Anchor::SouthEast, origin, half_width, half_height),
            Vector2::new(4.0, 3.0)
        );
        assert_relative_eq!(
            anchor_rectangle(Anchor::South, origin, half_width, half_height),
            Vector2::new(0.0, 3.0)
        );
        assert_relative_eq!(
            anchor_rectangle(Anchor::SouthWest, origin, half_width, half_height),
            Vector2::new(-4.0, 3.0)
        );
        assert_relative_eq!(
            anchor_rectangle(Anchor::West, origin, half_width, half_height),
            Vector2::new(-4.0, 0.0)
        );
        assert_relative_eq!(
            anchor_rectangle(Anchor::NorthWest, origin, half_width, half_height),
            Vector2::new(-4.0, -3.0)
        );

        // sin(angle) = 3.0 / 5.0;
        // we need it in degrees
        // we should basically get northeast with these polar coordinates
        let angle = -(3.0 / 5.0 as Scalar).asin() * 180.0 / crate::PI;
        let anchor = Anchor::Polar { radius: 5.0, angle };
        assert_relative_eq!(
            anchor_rectangle(anchor, origin, half_width, half_height),
            anchor_rectangle(Anchor::NorthEast, origin, half_width, half_height)
        );
    }

    #[test]
    fn coordinate() {
        let coordinate = Vector2::zeros();
        let shift = 10.0;
        let shift_rad = shift + DEFAULT_RADIUS;
        assert_relative_eq!(coordinate.above(shift), -shift_rad * Vector2::y());
        assert_relative_eq!(coordinate.below(shift), shift_rad * Vector2::y());
        assert_relative_eq!(coordinate.left(shift), -shift_rad * Vector2::x());
        assert_relative_eq!(coordinate.right(shift), shift_rad * Vector2::x());

        let xshift = 5.0;
        let yshift = 2.0;
        assert_relative_eq!(
            coordinate.above_right(xshift, yshift),
            coordinate.northeast() + Vector2::new(xshift, -yshift)
        );
        let xshift = 2.0;
        let yshift = 1.0;
        assert_relative_eq!(
            coordinate.above_left(xshift, yshift),
            coordinate.northwest() + Vector2::new(-xshift, -yshift)
        );

        let xshift = 1.0;
        let yshift = 9.0;
        assert_relative_eq!(
            coordinate.below_left(xshift, yshift),
            coordinate.southwest() + Vector2::new(-xshift, yshift)
        );
        let xshift = -9.0;
        let yshift = 4.0;
        assert_relative_eq!(
            coordinate.below_right(xshift, yshift),
            coordinate.southeast() + Vector2::new(xshift, yshift)
        );

        let anchor = Anchor::Polar {
            radius: 2.0 * DEFAULT_RADIUS,
            angle: 135.0,
        };
        assert_relative_eq!(coordinate.anchor(anchor), 2.0 * coordinate.southwest());
    }
}
