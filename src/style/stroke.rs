use super::color::Color;
use crate::Scalar;

use std::fmt::{Display, Formatter, Result as FmtResult};

const DASH: char = '4';
const DOT: char = '1';

#[derive(Clone, Copy, Debug)]
pub struct Stroke {
    color: Color,
    opacity: u8,
    width: Scalar,
    style: StrokeStyle,
}

impl Display for Stroke {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "stroke: {}; stroke-opacity: {}%; stroke-width: {}; stroke-dasharray: {};",
            self.color, self.opacity, self.width, self.style,
        )
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::None,
            opacity: 100,
            width: 1.0,
            style: StrokeStyle::Solid,
        }
    }
}

impl Stroke {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn color(self, color: Color) -> Self {
        Self {
            color,
            opacity: self.opacity,
            width: self.width,
            style: self.style,
        }
    }

    pub fn opacity(self, opacity: u8) -> Self {
        Self {
            color: self.color,
            opacity: opacity.min(100),
            width: self.width,
            style: self.style,
        }
    }

    pub fn width(self, width: Scalar) -> Self {
        Self {
            color: self.color,
            opacity: self.opacity,
            width,
            style: self.style,
        }
    }

    pub fn dashed(self) -> Self {
        Self {
            color: self.color,
            opacity: self.opacity,
            width: self.width,
            style: StrokeStyle::Dashed,
        }
    }

    pub fn dashdotted(self) -> Self {
        Self {
            color: self.color,
            opacity: self.opacity,
            width: self.width,
            style: StrokeStyle::Dashdotted,
        }
    }

    pub fn dotted(self) -> Self {
        Self {
            color: self.color,
            opacity: self.opacity,
            width: self.width,
            style: StrokeStyle::Dotted,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum StrokeStyle {
    Dashed,
    Dashdotted,
    Dotted,
    Solid,
}

impl Display for StrokeStyle {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Dashed => write!(f, "{} {}", DASH, DOT),
            Self::Dashdotted => write!(f, "{} {} {} {}", DASH, DOT, DASH, DOT),
            Self::Dotted => write!(f, "{}", DOT),
            Self::Solid => write!(f, "none"),
        }
    }
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self::Solid
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_stroke() {
        let stroke = Stroke::default();

        assert_eq!(stroke.color, Color::None);
        assert_eq!(stroke.opacity, 100);
        assert_eq!(stroke.width, 1.0);
        assert_eq!(stroke.style, StrokeStyle::Solid);

        let stroke = Stroke::new().dotted().width(3.5).color(Color::Green);

        assert_eq!(stroke.color, Color::Green);
        assert_eq!(stroke.opacity, 100);
        assert_eq!(stroke.width, 3.5);
        assert_eq!(stroke.style, StrokeStyle::Dotted);

        let stroke = Stroke::new().dashdotted().opacity(30);

        assert_eq!(stroke.color, Color::None);
        assert_eq!(stroke.opacity, 30);
        assert_eq!(stroke.width, 1.0);
        assert_eq!(stroke.style, StrokeStyle::Dashdotted);

        let stroke = Stroke::new()
            .dashed()
            .opacity(124)
            .color(Color::Rgb(10, 20, 30));

        assert_eq!(stroke.color, Color::Rgb(10, 20, 30));
        assert_eq!(stroke.opacity, 100);
        assert_eq!(stroke.width, 1.0);
        assert_eq!(stroke.style, StrokeStyle::Dashed);
    }

    #[test]
    fn display_stroke() {
        let stroke = Stroke::default();

        assert_eq!(
            stroke.to_string(),
            "stroke: none; stroke-opacity: 100%; stroke-width: 1; stroke-dasharray: none;"
        );

        let stroke = Stroke::new().dotted().width(3.5).color(Color::Green);

        assert_eq!(
            stroke.to_string(),
            format!(
                "stroke: green; stroke-opacity: 100%; stroke-width: 3.5; stroke-dasharray: {};",
                DOT
            ),
        );

        let stroke = Stroke::new().dashdotted().opacity(30);

        assert_eq!(
            stroke.to_string(),
            format!("stroke: none; stroke-opacity: 30%; stroke-width: 1; stroke-dasharray: {} {} {} {};", DASH, DOT, DASH, DOT),
        );

        let stroke = Stroke::new()
            .dashed()
            .opacity(124)
            .color(Color::Rgb(10, 20, 30));

        assert_eq!(
            stroke.to_string(),
            format!(
                "stroke: #0A141E; stroke-opacity: 100%; stroke-width: 1; stroke-dasharray: {} {};",
                DASH, DOT
            ),
        );
    }
}
