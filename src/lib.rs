pub mod anchor;
pub mod rectangle;
pub mod style;

use svg::node::element::Rectangle as SvgRect;
use svg::node::Node as SvgNode;

type Scalar = f32;
type Vector2 = nalgebra::Vector2<Scalar>;

pub trait IntoSvgNode {
    fn into_svg_node<T: Sized + SvgNode>(self, style: style::Style) -> T;
}

#[macro_export]
macro_rules! px {
    ($p:literal) => {
        $p as Scalar
    }
}

#[macro_export]
macro_rules! mm {
    ($p:literal) => {
        $p as Scalar * 3.78
    }
}

#[macro_export]
macro_rules! cm {
    ($p:literal) => {
        $p as Scalar * 37.8
    }
}

#[macro_export]
macro_rules! inch {
    ($p:literal) => {
        $p as Scalar * 96.0
    }
}

#[test]
fn pixels() {
    assert_eq!(px!(3.78), mm!(1));
    assert_eq!(cm!(10), mm!(100.0));
    assert_eq!(inch!(2), px!(192));
}
