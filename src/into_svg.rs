use crate::style::Style;

pub mod node {
    pub use svg::node::element::*;
}

pub const X: &str = "x";
pub const Y: &str = "y";
pub const WIDTH: &str = "width";
pub const HEIGHT: &str = "height";
pub const CORNER: &str = "rx";

pub const CX: &str = "cx";
pub const CY: &str = "cy";
pub const RADIUS: &str = "r";
pub const STYLE: &str = "style";

pub trait IntoSvg {
    type Output: Sized + svg::node::Node;
    fn into_svg(self, style: &Style) -> Self::Output;
}
