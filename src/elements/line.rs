use crate::style::Stroke;
use crate::svg::{keys, Attributes, IntoElem, Line as SvgLine, ToAttributes};
use crate::{into_elem, Vector2};

#[derive(Clone, Copy, Debug)]
pub struct Line {
    start: Vector2,
    end: Vector2,
}

impl Line {
    pub fn start(start: Vector2) -> Self {
        Self {
            start,
            end: Vector2::zeros(),
        }
    }
    pub fn end(self, end: Vector2) -> Self {
        Self {
            start: self.start,
            end,
        }
    }
}

into_elem!(Line, SvgLine, Stroke);

impl ToAttributes for Line {
    fn to_attributes(&self, attributes: &mut Attributes) {
        attributes.insert(keys::X1.into(), self.start[0].into());
        attributes.insert(keys::Y1.into(), self.start[1].into());
        attributes.insert(keys::X2.into(), self.end[0].into());
        attributes.insert(keys::Y2.into(), self.end[1].into());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn into_elem() {
        let mut attributes = Attributes::new();
        let line = Line::start(Vector2::y()).end(Vector2::x());
        line.to_attributes(&mut attributes);

        assert_eq!(attributes.get(keys::X1).unwrap().deref(), "0");
        assert_eq!(attributes.get(keys::Y1).unwrap().deref(), "1");
        assert_eq!(attributes.get(keys::X2).unwrap().deref(), "1");
        assert_eq!(attributes.get(keys::Y2).unwrap().deref(), "0");
    }
}
