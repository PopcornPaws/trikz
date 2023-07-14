use super::color::Color;
use crate::transform::{keys, svg, WriteAttribute};
use crate::Scalar;

const DASH: char = '4';
const DOT: char = '1';

#[derive(Clone, Copy, Debug)]
pub struct Stroke {
    color: Option<Color>,
    opacity: u8,
    width: Scalar,
    style: StrokeStyle,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: None,
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
            color: Some(color),
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

impl WriteAttribute for Stroke {
    fn write(&self, attributes: &mut svg::Attributes) {
        if let Some(color) = self.color {
            attributes.insert(keys::STROKE.into(), color.into());
            attributes.insert(
                keys::STROKE_OPACITY.into(),
                format!("{}%", self.opacity).into(),
            );
            attributes.insert(keys::STROKE_WIDTH.into(), self.width.into());
            attributes.insert(keys::STROKE_STYLE.into(), self.style.into());
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

impl Into<svg::Value> for StrokeStyle {
    fn into(self) -> svg::Value {
        match self {
            Self::Dashed => format!("{} {}", DASH, DOT).into(),
            Self::Dashdotted => format!("{} {} {} {}", DASH, DOT, DASH, DOT).into(),
            Self::Dotted => format!("{}", DOT).into(),
            Self::Solid => "none".into(),
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
    use std::ops::Deref;

    #[test]
    fn build() {
        let stroke = Stroke::default();

        assert_eq!(stroke.color, None);
        assert_eq!(stroke.opacity, 100);
        assert_eq!(stroke.width, 1.0);
        assert_eq!(stroke.style, StrokeStyle::Solid);

        let stroke = Stroke::new().dotted().width(3.5).color(Color::Green);

        assert_eq!(stroke.color, Some(Color::Green));
        assert_eq!(stroke.opacity, 100);
        assert_eq!(stroke.width, 3.5);
        assert_eq!(stroke.style, StrokeStyle::Dotted);

        let stroke = Stroke::new().dashdotted().opacity(30);

        assert_eq!(stroke.color, None);
        assert_eq!(stroke.opacity, 30);
        assert_eq!(stroke.width, 1.0);
        assert_eq!(stroke.style, StrokeStyle::Dashdotted);

        let stroke = Stroke::new()
            .dashed()
            .opacity(124)
            .color(Color::Rgb(10, 20, 30));

        assert_eq!(stroke.color, Some(Color::Rgb(10, 20, 30)));
        assert_eq!(stroke.opacity, 100);
        assert_eq!(stroke.width, 1.0);
        assert_eq!(stroke.style, StrokeStyle::Dashed);
    }

    #[test]
    fn display() {
        let mut attributes = svg::Attributes::new();
        let stroke = Stroke::default();
        stroke.write(&mut attributes);

        assert!(attributes.is_empty());

        let stroke = Stroke::new().dotted().width(3.5).color(Color::Green);
        stroke.write(&mut attributes);

        assert_eq!(
            attributes.get(keys::STROKE).unwrap().clone().deref(),
            "green"
        );
        assert_eq!(
            attributes
                .get(keys::STROKE_OPACITY)
                .unwrap()
                .clone()
                .deref(),
            "100%"
        );
        assert_eq!(
            attributes.get(keys::STROKE_WIDTH).unwrap().clone().deref(),
            "3.5"
        );
        assert_eq!(
            attributes.get(keys::STROKE_STYLE).unwrap().clone().deref(),
            DOT.to_string()
        );

        let stroke = Stroke::new().color(Color::Red).dashdotted().opacity(30);
        stroke.write(&mut attributes);

        assert_eq!(attributes.get(keys::STROKE).unwrap().clone().deref(), "red");
        assert_eq!(
            attributes
                .get(keys::STROKE_OPACITY)
                .unwrap()
                .clone()
                .deref(),
            "30%"
        );
        assert_eq!(
            attributes.get(keys::STROKE_WIDTH).unwrap().clone().deref(),
            "1"
        );
        assert_eq!(
            attributes.get(keys::STROKE_STYLE).unwrap().clone().deref(),
            format!("{} {} {} {}", DASH, DOT, DASH, DOT)
        );

        let stroke = Stroke::new()
            .dashed()
            .opacity(124)
            .color(Color::Rgb(10, 20, 30));

        stroke.write(&mut attributes);

        assert_eq!(
            attributes.get(keys::STROKE).unwrap().clone().deref(),
            "#0A141E"
        );
        assert_eq!(
            attributes
                .get(keys::STROKE_OPACITY)
                .unwrap()
                .clone()
                .deref(),
            "100%"
        );
        assert_eq!(
            attributes.get(keys::STROKE_WIDTH).unwrap().clone().deref(),
            "1"
        );
        assert_eq!(
            attributes.get(keys::STROKE_STYLE).unwrap().clone().deref(),
            format!("{} {}", DASH, DOT)
        );
    }
}
