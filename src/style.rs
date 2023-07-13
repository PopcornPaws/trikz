use crate::Scalar;
use std::fmt::{Display, Formatter, Result as FmtResult};

const DASH: char = '4';
const DOT: char = '1';

#[derive(Clone, Debug, Default)]
pub struct Style {
    pub fill: Option<Color>,
    pub stroke: Option<Stroke>,
}

#[derive(Clone, Copy, Debug)]
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

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::Black => write!(f, "black"),
            Self::Blue => write!(f, "blue"),
            Self::Cyan => write!(f, "cyan"),
            Self::Gray => write!(f, "gray"),
            Self::Green => write!(f, "green"),
            Self::Magenta => write!(f, "magenta"),
            Self::Red => write!(f, "red"),
            Self::White => write!(f, "white"),
            Self::Rgb(r, g, b) => write!(f, "#{:#03x}{:#03x}{:#03x}", r, g, b),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Stroke {
    color: Option<Color>,
    opacity: Option<Scalar>,
    width: Option<Scalar>,
    style: Option<StrokeStyle>,
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

    pub fn opacity(self, opacity: Scalar) -> Self {
        Self {
            color: self.color,
            opacity: Some(opacity),
            width: self.width,
            style: self.style,
        }
    }

    pub fn width(self, width: Scalar) -> Self {
        Self {
            color: self.color,
            opacity: self.opacity,
            width: Some(width),
            style: self.style,
        }
    }

    pub fn style(self, style: StrokeStyle) -> Self {
        Self {
            color: self.color,
            opacity: self.opacity,
            width: self.width,
            style: Some(style),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StrokeStyle {
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
