use crate::style::{Stroke, Style};
use crate::svg::{keys, IntoElem, Line as SvgLine, WriteAttributes};
use crate::Vector2;

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

impl IntoElem for Line {
    type Output = SvgLine;
    type StyleType = Stroke;
    fn into_elem(self, style: &Style<Self::StyleType>) -> Self::Output {
        let mut output = SvgLine::new()
            .set(keys::X1, self.start[0])
            .set(keys::Y1, self.start[1])
            .set(keys::X2, self.end[0])
            .set(keys::Y2, self.end[1]);

        style.write(output.get_attributes_mut());

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn into_elem() {
        let style = Style::default();
        let line = Line::start(Vector2::y()).end(Vector2::x());

        let elem = line.into_elem(&style);
        let attributes = elem.get_attributes();

        assert_eq!(attributes.get(keys::X1).unwrap().deref(), "0");
        assert_eq!(attributes.get(keys::Y1).unwrap().deref(), "1");
        assert_eq!(attributes.get(keys::X2).unwrap().deref(), "1");
        assert_eq!(attributes.get(keys::Y2).unwrap().deref(), "0");
    }
}
