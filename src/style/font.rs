use crate::svg::{self, keys, WriteAttributes};

#[derive(Clone, Copy, Debug, Default)]
pub struct Font {
    pub size: FontSize,
    // TODO
    // family
    // style
}

impl Font {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn xxs(self) -> Self {
        Self {
            size: FontSize::XXS,
        }
    }
    pub fn xs(self) -> Self {
        Self { size: FontSize::XS }
    }
    pub fn s(self) -> Self {
        Self { size: FontSize::S }
    }
    pub fn m(self) -> Self {
        Self { size: FontSize::M }
    }
    pub fn l(self) -> Self {
        Self { size: FontSize::L }
    }
    pub fn xl(self) -> Self {
        Self { size: FontSize::XL }
    }
    pub fn xxl(self) -> Self {
        Self {
            size: FontSize::XXL,
        }
    }
    pub fn xxxl(self) -> Self {
        Self {
            size: FontSize::XXXL,
        }
    }
}

impl Default for FontSize {
    fn default() -> Self {
        Self::M
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FontSize {
    XXS,
    XS,
    S,
    M,
    L,
    XL,
    XXL,
    XXXL,
}

impl WriteAttributes for Font {
    fn write(&self, attributes: &mut svg::Attributes) {
        attributes.insert(keys::FONT_SIZE.into(), self.size.into());
    }
}

impl Into<svg::Value> for FontSize {
    fn into(self) -> svg::Value {
        match self {
            Self::XXS => "xx-small".into(),
            Self::XS => "x-small".into(),
            Self::S => "small".into(),
            Self::M => "medium".into(),
            Self::L => "large".into(),
            Self::XL => "x-large".into(),
            Self::XXL => "xx-large".into(),
            Self::XXXL => "xxx-large".into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn font_size_into_value() {
        assert_eq!(<FontSize as Into<svg::Value>>::into(FontSize::XXS).deref(), svg::Value::from("xx-small").deref());
        assert_eq!(<FontSize as Into<svg::Value>>::into(FontSize::XS).deref(), svg::Value::from("x-small").deref());
        assert_eq!(<FontSize as Into<svg::Value>>::into(FontSize::S).deref(), svg::Value::from("small").deref());
        assert_eq!(<FontSize as Into<svg::Value>>::into(FontSize::M).deref(), svg::Value::from("medium").deref());
        assert_eq!(<FontSize as Into<svg::Value>>::into(FontSize::L).deref(), svg::Value::from("large").deref());
        assert_eq!(<FontSize as Into<svg::Value>>::into(FontSize::XL).deref(), svg::Value::from("x-large").deref());
        assert_eq!(<FontSize as Into<svg::Value>>::into(FontSize::XXL).deref(), svg::Value::from("xx-large").deref());
        assert_eq!(<FontSize as Into<svg::Value>>::into(FontSize::XXXL).deref(), svg::Value::from("xxx-large").deref());
    }

    #[test]
    fn font_write_attribute() {
        let mut attributes = svg::Attributes::new();
        let font = Font::default();
        font.write(&mut attributes);

        assert_eq!(attributes.get(keys::FONT_SIZE).unwrap().clone().deref(), "medium");
    }
}
