mod section;

use crate::style::{Stroke, Style};
use crate::svg::{keys, IntoElem, Path as SvgPath, Value, WriteAttributes};
use crate::{Scalar, Vector2};
use section::Section;

#[derive(Clone, Debug)]
pub struct Path {
    sequence: Vec<Section>,
}

pub struct PathBuilder {
    sequence: Vec<Section>,
}

impl PathBuilder {
    pub fn start(start: Vector2) -> Self {
        Self {
            sequence: vec![Section::MoveTo(start)],
        }
    }

    pub fn mv_to(&mut self, xy: Vector2) -> &mut Self {
        self.sequence.push(Section::MoveTo(xy));
        self
    }

    pub fn mv(&mut self, dxdy: Vector2) -> &mut Self {
        self.sequence.push(Section::Move(dxdy));
        self
    }

    pub fn line_to(&mut self, xy: Vector2) -> &mut Self {
        self.sequence.push(Section::LineTo(xy));
        self
    }

    pub fn line(&mut self, dxdy: Vector2) -> &mut Self {
        self.sequence.push(Section::Line(dxdy));
        self
    }

    pub fn vline_to(&mut self, y: Scalar) -> &mut Self {
        self.sequence.push(Section::VerticalLineTo(y));
        self
    }

    pub fn vline(&mut self, dy: Scalar) -> &mut Self {
        self.sequence.push(Section::VerticalLine(dy));
        self
    }

    pub fn hline_to(&mut self, x: Scalar) -> &mut Self {
        self.sequence.push(Section::HorizontalLineTo(x));
        self
    }

    pub fn hline(&mut self, dx: Scalar) -> &mut Self {
        self.sequence.push(Section::HorizontalLine(dx));
        self
    }

    pub fn curve_to(&mut self, x1y1: Vector2, x2y2: Vector2, xy: Vector2) -> &mut Self {
        self.sequence.push(Section::CurveTo(x1y1, x2y2, xy));
        self
    }

    pub fn curve(&mut self, dx1dy1: Vector2, dx2dy2: Vector2, dxdy: Vector2) -> &mut Self {
        self.sequence.push(Section::Curve(dx1dy1, dx2dy2, dxdy));
        self
    }

    pub fn end(&mut self) -> Path {
        Path {
            sequence: std::mem::take(&mut self.sequence),
        }
    }

    pub fn close(&mut self) -> Path {
        self.sequence.push(Section::Close);
        Path {
            sequence: std::mem::take(&mut self.sequence),
        }
    }
}

impl Path {
    pub fn sequence(&self) -> &[Section] {
        self.sequence.as_ref()
    }

    pub fn into_sequence(self) -> Vec<Section> {
        self.sequence
    }

    pub fn cursor(&self, index: usize) -> Vector2 {
        // NOTE sequence is never empty because Path can only be initialized via the `start`
        // method. Thus, we can extract the starting point which might be needed if the last
        // element is a Close section. Unwrap is also fine because Section::cursor() cannot return
        // none when the Section is a MoveTo variant, which is always the case.
        let start = self.sequence[0].cursor(Vector2::zeros()).unwrap();
        self.sequence[1..]
            .iter()
            .take(index)
            .fold(start, |acc, x| x.cursor(acc).unwrap_or(start))
    }
}

impl From<Path> for Value {
    fn from(path: Path) -> Value {
        path.into_sequence()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
            .into()
    }
}

impl IntoElem for Path {
    type Output = SvgPath;
    type StyleType = Stroke;
    fn into_elem(self, style: &Style<Self::StyleType>) -> Self::Output {
        let mut path = SvgPath::new().set(keys::PATH, Value::from(self));

        style.write(path.get_attributes_mut());

        path
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ops::Deref;

    #[test]
    fn build() {
        let path = PathBuilder::start(Vector2::new(1.0, 2.0))
            .mv_to(Vector2::zeros())
            .mv(Vector2::zeros())
            .line_to(Vector2::zeros())
            .line(Vector2::zeros())
            .hline_to(0.0)
            .hline(0.0)
            .vline_to(0.0)
            .vline(0.0)
            .curve_to(Vector2::zeros(), Vector2::zeros(), Vector2::zeros())
            .curve(Vector2::zeros(), Vector2::zeros(), Vector2::zeros())
            .close();

        assert_eq!(
            path.sequence(),
            &[
                Section::MoveTo(Vector2::new(1.0, 2.0)),
                Section::MoveTo(Vector2::zeros()),
                Section::Move(Vector2::zeros()),
                Section::LineTo(Vector2::zeros()),
                Section::Line(Vector2::zeros()),
                Section::HorizontalLineTo(0.0),
                Section::HorizontalLine(0.0),
                Section::VerticalLineTo(0.0),
                Section::VerticalLine(0.0),
                Section::CurveTo(Vector2::zeros(), Vector2::zeros(), Vector2::zeros()),
                Section::Curve(Vector2::zeros(), Vector2::zeros(), Vector2::zeros()),
                Section::Close,
            ]
        );
    }

    #[test]
    fn cursor() {
        let origin = Vector2::new(1.0, -2.0);
        let path = PathBuilder::start(origin).end();
        assert_eq!(path.cursor(0), origin);
        assert_eq!(path.cursor(1), origin);
        assert_eq!(path.cursor(4), origin);

        let delta = Vector2::new(2.0, 5.0);
        let end = origin + delta;
        let path = PathBuilder::start(origin).line(delta).end();
        assert_eq!(path.cursor(0), origin);
        assert_eq!(path.cursor(1), end);
        assert_eq!(path.cursor(4), end);

        let x = 30.0;
        let y = 10.0;
        let path = PathBuilder::start(origin).vline(y).hline(x).end();
        assert_eq!(path.cursor(0), origin);
        assert_eq!(path.cursor(1), origin + y * Vector2::y());
        assert_eq!(path.cursor(2), origin + Vector2::new(x, y));
        assert_eq!(path.cursor(3), origin + Vector2::new(x, y));
        assert_eq!(path.cursor(4), origin + Vector2::new(x, y));

        let path = PathBuilder::start(origin)
            .vline(y)
            .hline(x)
            .line_to(Vector2::new(100.0, 200.0))
            .close();
        assert_eq!(path.cursor(path.sequence().len() - 1), origin);
    }

    #[test]
    fn convert_to_value() {
        let path = PathBuilder::start(Vector2::new(-1.75, -2.5))
            .vline(4.0)
            .hline(-12.34)
            .mv(Vector2::new(1.0, -1.0))
            .curve_to(
                100.0 * Vector2::x(),
                -200.0 * Vector2::y(),
                Vector2::zeros(),
            )
            .close();

        assert_eq!(
            Value::from(path).deref(),
            Value::from("M -1.75 -2.5 v 4 h -12.34 m 1 -1 C 100 0, -0 -200, 0 0 Z").deref(),
        );
    }
}
