use crate::{Scalar, Vector2};
use std::fmt::{Display, Formatter, Result as FmtResult};

// TODO add Arcs
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Segment {
    MoveTo(Vector2),
    Move(Vector2),
    LineTo(Vector2),
    Line(Vector2),
    VerticalLineTo(Scalar),
    VerticalLine(Scalar),
    HorizontalLineTo(Scalar),
    HorizontalLine(Scalar),
    CurveTo(Vector2, Vector2, Vector2), // Bézier curves
    Curve(Vector2, Vector2, Vector2),
    Close,
}

impl Segment {
    pub fn cursor(&self, previous: Vector2) -> Option<Vector2> {
        match self {
            Self::MoveTo(xy) | Self::LineTo(xy) | Self::CurveTo(_, _, xy) => Some(*xy),
            Self::Move(dxdy) | Self::Line(dxdy) | Self::Curve(_, _, dxdy) => Some(previous + dxdy),
            Self::VerticalLineTo(y) => Some(Vector2::new(previous[0], *y)),
            Self::VerticalLine(dy) => Some(previous + Vector2::new(0.0, *dy)),
            Self::HorizontalLineTo(x) => Some(Vector2::new(*x, previous[1])),
            Self::HorizontalLine(dx) => Some(previous + Vector2::new(*dx, 0.0)),
            Self::Close => None,
        }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::MoveTo(xy) => write!(f, "M {} {}", xy[0], xy[1]),
            Self::Move(dxdy) => write!(f, "m {} {}", dxdy[0], dxdy[1]),
            Self::LineTo(xy) => write!(f, "L {} {}", xy[0], xy[1]),
            Self::Line(dxdy) => write!(f, "l {} {}", dxdy[0], dxdy[1]),
            Self::VerticalLineTo(y) => write!(f, "V {}", y),
            Self::VerticalLine(dy) => write!(f, "v {}", dy),
            Self::HorizontalLineTo(x) => write!(f, "H {}", x),
            Self::HorizontalLine(dx) => write!(f, "h {}", dx),
            Self::CurveTo(x1y1, x2y2, xy) => write!(
                f,
                "C {} {}, {} {}, {} {}",
                x1y1[0], x1y1[1], x2y2[0], x2y2[1], xy[0], xy[1]
            ),
            Self::Curve(dx1dy1, dx2dy2, dxdy) => write!(
                f,
                "c {} {}, {} {}, {} {}",
                dx1dy1[0], dx1dy1[1], dx2dy2[0], dx2dy2[1], dxdy[0], dxdy[1]
            ),
            Self::Close => write!(f, "Z"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cursor() {
        assert_eq!(
            Segment::MoveTo(Vector2::new(10.0, 20.0)).cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(10.0, 20.0))
        );
        assert_eq!(
            Segment::Move(Vector2::new(10.0, 20.0)).cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(11.0, 21.0))
        );
        assert_eq!(
            Segment::LineTo(Vector2::new(10.0, 20.0)).cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(10.0, 20.0))
        );
        assert_eq!(
            Segment::Line(Vector2::new(10.0, 20.0)).cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(11.0, 21.0))
        );
        assert_eq!(
            Segment::CurveTo(Vector2::zeros(), Vector2::zeros(), Vector2::new(10.0, 20.0))
                .cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(10.0, 20.0))
        );
        assert_eq!(
            Segment::Curve(Vector2::zeros(), Vector2::zeros(), Vector2::new(10.0, 20.0))
                .cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(11.0, 21.0))
        );
        assert_eq!(
            Segment::VerticalLineTo(20.0).cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(1.0, 20.0))
        );
        assert_eq!(
            Segment::VerticalLine(20.0).cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(1.0, 21.0))
        );
        assert_eq!(
            Segment::HorizontalLineTo(20.0).cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(20.0, 1.0))
        );
        assert_eq!(
            Segment::HorizontalLine(20.0).cursor(Vector2::new(1.0, 1.0)),
            Some(Vector2::new(21.0, 1.0))
        );
        assert!(Segment::Close.cursor(Vector2::zeros()).is_none());
    }

    #[test]
    fn display() {
        let abs = Vector2::new(1.0, 2.0);
        let rel = Vector2::new(1.5, -2.5);
        let c1 = Vector2::new(-10.0, 20.0);
        let c2 = Vector2::new(1.0, 0.0);
        assert_eq!(Segment::MoveTo(abs).to_string(), "M 1 2");
        assert_eq!(Segment::Move(rel).to_string(), "m 1.5 -2.5");
        assert_eq!(Segment::LineTo(abs).to_string(), "L 1 2");
        assert_eq!(Segment::Line(rel).to_string(), "l 1.5 -2.5");
        assert_eq!(Segment::VerticalLineTo(abs[1]).to_string(), "V 2");
        assert_eq!(Segment::VerticalLine(rel[1]).to_string(), "v -2.5");
        assert_eq!(Segment::HorizontalLineTo(abs[0]).to_string(), "H 1");
        assert_eq!(Segment::HorizontalLine(rel[0]).to_string(), "h 1.5");
        assert_eq!(
            Segment::CurveTo(c1, c2, abs).to_string(),
            "C -10 20, 1 0, 1 2"
        );
        assert_eq!(
            Segment::Curve(c1, c2, rel).to_string(),
            "c -10 20, 1 0, 1.5 -2.5"
        );
        assert_eq!(Segment::Close.to_string(), "Z");
    }
}
