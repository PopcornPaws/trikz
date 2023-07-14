use crate::into_svg::{node, IntoSvg, CORNER, HEIGHT, WIDTH, X, Y, STYLE};
use crate::style::Style;
use crate::{Scalar, Vector2};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rectangle {
    pub origin: Vector2,
    pub width: Scalar,
    pub height: Scalar,
    pub corner_radius: Scalar,
}

impl Rectangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn at(self, origin: Vector2) -> Self {
        Self {
            origin,
            width: self.width,
            height: self.height,
            corner_radius: self.corner_radius,
        }
    }

    pub fn width(self, width: Scalar) -> Self {
        Self {
            origin: self.origin,
            width,
            height: self.height,
            corner_radius: self.corner_radius,
        }
    }

    pub fn height(self, height: Scalar) -> Self {
        Self {
            origin: self.origin,
            width: self.width,
            height,
            corner_radius: self.corner_radius,
        }
    }

    pub fn rounded_corners(self, corner_radius: Scalar) -> Self {
        Self {
            origin: self.origin,
            width: self.width,
            height: self.height,
            corner_radius,
        }
    }
}

impl IntoSvg for Rectangle {
    type Output = node::Rectangle;
    fn into_svg(self, style: &Style) -> Self::Output {
        node::Rectangle::new()
            .set(X, self.origin[0])
            .set(Y, self.origin[1])
            .set(WIDTH, self.width)
            .set(HEIGHT, self.height)
            .set(CORNER, self.corner_radius)
            .set(STYLE, style.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::style::{Color, Stroke};

    use std::ops::Deref;

    #[test]
    fn into_svg() {
        let rectangle = Rectangle::new()
            .at(Vector2::new(10.0, 20.0))
            .width(10.0)
            .height(100.0)
            .rounded_corners(4.0);

        let style = Style::default();

        let svg = rectangle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(X).unwrap().clone().deref(), "10");
        assert_eq!(svg_attributes.get(Y).unwrap().clone().deref(), "20");
        assert_eq!(svg_attributes.get(WIDTH).unwrap().clone().deref(), "10");
        assert_eq!(svg_attributes.get(HEIGHT).unwrap().clone().deref(), "100");
        assert_eq!(svg_attributes.get(CORNER).unwrap().clone().deref(), "4");
        assert_eq!(
            svg_attributes.get(STYLE).unwrap().clone().deref(),
            format!("fill: none; {}", Stroke::default())
        );

        let rectangle = Rectangle::new();

        let stroke = Stroke::new().dashed().color(Color::Magenta);
        let style = Style::new().fill(Color::Green).stroke(stroke);

        let svg = rectangle.into_svg(&style);
        let svg_attributes = svg.get_attributes();

        assert_eq!(svg_attributes.get(X).unwrap().clone().deref(), "0");
        assert_eq!(svg_attributes.get(Y).unwrap().clone().deref(), "0");
        assert_eq!(svg_attributes.get(WIDTH).unwrap().clone().deref(), "0");
        assert_eq!(svg_attributes.get(HEIGHT).unwrap().clone().deref(), "0");
        assert_eq!(svg_attributes.get(CORNER).unwrap().clone().deref(), "0");
        assert_eq!(
            svg_attributes.get(STYLE).unwrap().clone().deref(),
            format!("fill: green; {}", stroke)
        );
    }
}

// TODO
// - impl AnchorT
