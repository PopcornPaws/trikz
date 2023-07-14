use crate::rectangle::Rectangle;
use crate::style::Style;

use svg::node::element::Rectangle as SvgRectangle;
use svg::node::Node as SvgNode;

const X: &str = "x";
const Y: &str = "y";
const WIDTH: &str = "width";
const HEIGHT: &str = "height";
const CORNER: &str = "rx";
const STYLE: &str = "style";
const NONE: &str = "none";

pub trait IntoSvg {
    type Output: Sized + SvgNode;
    fn into_svg(self, style: &Style) -> Self::Output;
}

impl IntoSvg for Rectangle {
    type Output = SvgRectangle;
    fn into_svg(self, style: &Style) -> Self::Output {
        SvgRectangle::new()
            .set(X, self.origin[0])
            .set(Y, self.origin[1])
            .set(WIDTH, self.attributes.width)
            .set(HEIGHT, self.attributes.height)
            .set(CORNER, self.attributes.corner_radius)
            .set(STYLE, style.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Vector2;
    use crate::rectangle::Attributes as RectangleAttributes;
    use crate::style::{Stroke, Color};
    use svg::node::Value;

    use std::ops::Deref;

    #[test]
    fn rectangle_into_svg() {
        let attributes = RectangleAttributes::new()
            .width(10.0)
            .height(100.0)
            .radius(4.0);
        let rectangle = Rectangle::new(attributes).at(Vector2::new(10.0, 20.0));

        let style = Style::default();

        let svg = rectangle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(X).unwrap().clone().deref(), "10");
        assert_eq!(svg_attributes.get(Y).unwrap().clone().deref(), "20");
        assert_eq!(svg_attributes.get(WIDTH).unwrap().clone().deref(), "10");
        assert_eq!(svg_attributes.get(HEIGHT).unwrap().clone().deref(), "100");
        assert_eq!(svg_attributes.get(CORNER).unwrap().clone().deref(), "4");
        assert_eq!(svg_attributes.get(STYLE).unwrap().clone().deref(), format!("fill: none; {}", Stroke::default()));

        let rectangle = Rectangle::new(attributes);

        let stroke = Stroke::new().dashed().color(Color::Magenta);
        let style = Style::new().fill(Color::Green).stroke(stroke);

        let svg = rectangle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(STYLE).unwrap().clone().deref(), format!("fill: green; {}", stroke));
    }
}
