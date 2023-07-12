type Scalar = f32;
type Vector2 = nalgebra::Vector2<Scalar>;

use svg::node::element::Rectangle as SvgRect;
use svg::node::Node as SvgNode;

const WIDTH: &str = "width";
const HEIGHT: &str = "width";
const CORNER: &str = "rx";

pub trait IntoSvg {
    fn into_svg<T: Sized + SvgNode>(self) -> T;
}

#[derive(Clone, Copy)]
pub struct Rectangle {
    attributes: Attributes,
    origin: Vector2,
}

#[derive(Clone, Copy)]
pub struct Attributes {
    pub width: Scalar,
    pub height: Scalar,
    pub corner_radius: Scalar,
}

pub struct Style {
    pub fill: Option<Color>,
    pub stroke: Option<Stroke>,
    pub opacity: Scalar,
}

pub enum Color {
    Black,
    Blue,
    Cyan,
    Gray,
    Green,
    Magenta,
    Red,
    White,
    Rgb(u8, u8, u8)
}


impl Rectangle {
    pub fn new(attributes: Attributes) -> Self {
        Self {
            attributes
            origin: Vector2::zeros(),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(2 + 2, 4)
    }
}
