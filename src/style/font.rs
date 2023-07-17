use crate::svg::{keys, Attributes, Value, WriteAttributes};

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

#[allow(clippy::upper_case_acronyms)]
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
    fn write(&self, attributes: &mut Attributes) {
        attributes.insert(keys::FONT_SIZE.into(), self.size.into());
    }
}

impl From<FontSize> for Value {
    fn from(fontsize: FontSize) -> Value {
        match fontsize {
            FontSize::XXS => "xx-small".into(),
            FontSize::XS => "x-small".into(),
            FontSize::S => "small".into(),
            FontSize::M => "medium".into(),
            FontSize::L => "large".into(),
            FontSize::XL => "x-large".into(),
            FontSize::XXL => "xx-large".into(),
            FontSize::XXXL => "xxx-large".into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn into_value() {
        assert_eq!(
            Value::from(FontSize::XXS).deref(),
            Value::from("xx-small").deref()
        );
        assert_eq!(
            Value::from(FontSize::XS).deref(),
            Value::from("x-small").deref()
        );
        assert_eq!(
            Value::from(FontSize::S).deref(),
            Value::from("small").deref()
        );
        assert_eq!(
            Value::from(FontSize::M).deref(),
            Value::from("medium").deref()
        );
        assert_eq!(
            Value::from(FontSize::L).deref(),
            Value::from("large").deref()
        );
        assert_eq!(
            Value::from(FontSize::XL).deref(),
            Value::from("x-large").deref()
        );
        assert_eq!(
            Value::from(FontSize::XXL).deref(),
            Value::from("xx-large").deref()
        );
        assert_eq!(
            Value::from(FontSize::XXXL).deref(),
            Value::from("xxx-large").deref()
        );
    }

    #[test]
    fn write_attribute() {
        let mut attributes = Attributes::new();
        let font = Font::default();
        font.write(&mut attributes);

        assert_eq!(
            attributes.get(keys::FONT_SIZE).unwrap().clone().deref(),
            "medium"
        );
    }
}
