use super::color::Color;
use crate::svgutils::{
    keys,
    raw::{Attributes, Value},
    ToAttributes,
};
use crate::Scalar;

const DASH: char = '4';
const DOT: char = '1';

#[derive(Clone, Debug)]
pub struct Stroke {
    color: Option<Color>,
    markers: [Option<String>; 3],
    opacity: u8,
    width: Scalar,
    style: StrokeStyle,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: None,
            markers: [None, None, None],
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
            markers: self.markers,
            opacity: self.opacity,
            width: self.width,
            style: self.style,
        }
    }

    pub fn opacity(self, opacity: u8) -> Self {
        Self {
            color: self.color,
            markers: self.markers,
            opacity: opacity.min(100),
            width: self.width,
            style: self.style,
        }
    }

    pub fn width(self, width: Scalar) -> Self {
        Self {
            color: self.color,
            markers: self.markers,
            opacity: self.opacity,
            width,
            style: self.style,
        }
    }

    pub fn dashed(self) -> Self {
        Self {
            color: self.color,
            markers: self.markers,
            opacity: self.opacity,
            width: self.width,
            style: StrokeStyle::Dashed,
        }
    }

    pub fn dashdotted(self) -> Self {
        Self {
            color: self.color,
            markers: self.markers,
            opacity: self.opacity,
            width: self.width,
            style: StrokeStyle::Dashdotted,
        }
    }

    pub fn dotted(self) -> Self {
        Self {
            color: self.color,
            markers: self.markers,
            opacity: self.opacity,
            width: self.width,
            style: StrokeStyle::Dotted,
        }
    }

    pub fn marker_start(mut self, marker_id: String) -> Self {
        self.markers[0] = Some(marker_id);
        Self {
            color: self.color,
            markers: self.markers,
            opacity: self.opacity,
            width: self.width,
            style: self.style,
        }
    }

    pub fn marker_mid(mut self, marker_id: String) -> Self {
        self.markers[1] = Some(marker_id);
        Self {
            color: self.color,
            markers: self.markers,
            opacity: self.opacity,
            width: self.width,
            style: self.style,
        }
    }

    pub fn marker_end(mut self, marker_id: String) -> Self {
        self.markers[2] = Some(marker_id);
        Self {
            color: self.color,
            markers: self.markers,
            opacity: self.opacity,
            width: self.width,
            style: self.style,
        }
    }
}

impl ToAttributes for Stroke {
    fn to_attributes(&self, attributes: &mut Attributes) {
        if let Some(color) = self.color {
            attributes.insert(keys::STROKE.into(), color.into());
            attributes.insert(
                keys::STROKE_OPACITY.into(),
                format!("{}%", self.opacity).into(),
            );
            attributes.insert(keys::STROKE_WIDTH.into(), self.width.into());
            attributes.insert(keys::STROKE_STYLE.into(), self.style.into());

            self.markers
                .iter()
                .zip(keys::MARKERS)
                .filter_map(|(maybe_marker, key)| maybe_marker.as_ref().map(|marker| (marker, key)))
                .for_each(|(marker, &key)| {
                    attributes.insert(key.into(), format!("url(#{})", marker).into());
                });
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

impl From<StrokeStyle> for Value {
    fn from(stroke_style: StrokeStyle) -> Value {
        match stroke_style {
            StrokeStyle::Dashed => format!("{} {}", DASH, DOT).into(),
            StrokeStyle::Dashdotted => format!("{} {} {} {}", DASH, DOT, DASH, DOT).into(),
            StrokeStyle::Dotted => format!("{}", DOT).into(),
            StrokeStyle::Solid => "none".into(),
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
        assert_eq!(stroke.markers, [None, None, None]);

        let sm = "start-marker".to_string();
        let mm = "mid-marker".to_string();
        let em = "end-marker".to_string();
        let stroke = Stroke::new()
            .dashed()
            .opacity(124)
            .marker_mid(mm.clone())
            .marker_end(em.clone())
            .marker_start(sm.clone())
            .color(Color::Rgb(10, 20, 30));

        assert_eq!(stroke.color, Some(Color::Rgb(10, 20, 30)));
        assert_eq!(stroke.opacity, 100);
        assert_eq!(stroke.width, 1.0);
        assert_eq!(stroke.style, StrokeStyle::Dashed);
        assert_eq!(stroke.markers, [Some(sm), Some(mm), Some(em)]);
    }

    #[test]
    fn display() {
        let mut attributes = Attributes::new();
        let stroke = Stroke::default();
        stroke.to_attributes(&mut attributes);

        assert!(attributes.is_empty());

        let stroke = Stroke::new().dotted().width(3.5).color(Color::Green);
        stroke.to_attributes(&mut attributes);

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
        stroke.to_attributes(&mut attributes);

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

        let marker_id = "arrow".to_string();
        let stroke = Stroke::new()
            .dashed()
            .opacity(124)
            .marker_end(marker_id.clone())
            .color(Color::Rgb(10, 20, 30));

        stroke.to_attributes(&mut attributes);

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
        assert_eq!(
            attributes.get(keys::MARKERS[2]).unwrap().clone().deref(),
            format!("url(#{})", marker_id)
        );
    }
}
