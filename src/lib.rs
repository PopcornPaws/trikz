mod circle;
mod color;
mod rectangle;
mod stroke;

pub use color::Color;
pub use stroke::Stroke;

use svg::node::element::Rectangle as SvgRect;
use svg::node::Node as SvgNode;

type Scalar = f32;
type Vector2 = nalgebra::Vector2<Scalar>;

const WIDTH: &str = "width";
const HEIGHT: &str = "width";
const CORNER: &str = "rx";

pub trait IntoSvg {
    fn into_svg<T: Sized + SvgNode>(self, style: Style) -> T;
}

pub trait Node {
    fn anchor(&self, anchor: Anchor) -> Vector2;
    fn origin(&self) -> Vector2 {
        self.anchor(Anchor::Origin)
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

pub struct Pixel(Scalar);

impl Pixel {
    pub fn mm(mm: Scalar) -> Self {
        Self(3.78 * mm)
    }
    pub fn cm(cm: Scalar) -> Self {
        Self::mm(10.0 * cm)
    }
    pub fn in(inches: Scalar) -> Self {
        Self(96.0 * inches)
    }
}

pub struct Style {
    pub fill: Option<Color>,
    pub stroke: Option<Stroke>,
    pub opacity: Scalar,
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(2 + 2, 4)
    }
}
