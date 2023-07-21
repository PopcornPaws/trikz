use crate::svgutils::{Attributes as SvgAttributes, Value};
use crate::Scalar;
use std::collections::HashMap;

pub struct Attributes(HashMap<&'static str, Attribute>);

impl Attributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: &'static str, attribute: Attribute) {
        self.0.insert(key, attribute);
    }

    pub fn get_scalar(&self, key: &'static str) -> Option<Scalar> {
        self.0.get(&key).map(|attr| attr.try_into().ok()).flatten()
    }

    pub fn into_inner(self) -> HashMap<&'static str, Attribute> {
        self.0
    }

    pub fn extend(&mut self, other: Attributes) {
        self.0.extend(other.into_inner().into_iter())
    }

    pub fn append_into(self, attributes: &mut SvgAttributes) {
        self.0.into_iter().for_each(|(key, value)| {
            attributes.insert(key.into(), Value::from(value));
        })
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

pub enum Attribute {
    Scalar(Scalar),
    String(String),
    Value(Value),
}

impl From<Attribute> for Value {
    fn from(attr: Attribute) -> Self {
        match attr {
            Attribute::Scalar(x) => x.into(),
            Attribute::String(x) => x.into(),
            Attribute::Value(x) => x.into(),
        }
    }
}

impl TryFrom<&Attribute> for Scalar {
    type Error = ();
    fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
        match attr {
            Attribute::Scalar(x) => Ok(*x),
            _ => Err(()),
        }
    }
}
