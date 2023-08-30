mod segment;

use crate::elements::ReprT;
use crate::svgutils::{keys, raw, ToAttributes};
use crate::{Scalar, Vector2};
use segment::Segment;

#[derive(Clone, Debug)]
pub struct Path(Vec<Segment>);

impl ReprT for Path {
    type Repr = crate::style::Stroke;
}

pub struct PathBuilder(Vec<Segment>);

impl PathBuilder {
    pub fn start(start: Vector2) -> Self {
        Self(vec![Segment::MoveTo(start)])
    }

    pub fn mv_to(&mut self, xy: Vector2) -> &mut Self {
        self.0.push(Segment::MoveTo(xy));
        self
    }

    pub fn mv(&mut self, dxdy: Vector2) -> &mut Self {
        self.0.push(Segment::Move(dxdy));
        self
    }

    pub fn line_to(&mut self, xy: Vector2) -> &mut Self {
        self.0.push(Segment::LineTo(xy));
        self
    }

    pub fn line(&mut self, dxdy: Vector2) -> &mut Self {
        self.0.push(Segment::Line(dxdy));
        self
    }

    pub fn vline_to(&mut self, y: Scalar) -> &mut Self {
        self.0.push(Segment::VerticalLineTo(y));
        self
    }

    pub fn vline(&mut self, dy: Scalar) -> &mut Self {
        self.0.push(Segment::VerticalLine(dy));
        self
    }

    pub fn hline_to(&mut self, x: Scalar) -> &mut Self {
        self.0.push(Segment::HorizontalLineTo(x));
        self
    }

    pub fn hline(&mut self, dx: Scalar) -> &mut Self {
        self.0.push(Segment::HorizontalLine(dx));
        self
    }

    pub fn curve_to(&mut self, x1y1: Vector2, x2y2: Vector2, xy: Vector2) -> &mut Self {
        self.0.push(Segment::CurveTo(x1y1, x2y2, xy));
        self
    }

    pub fn curve(&mut self, dx1dy1: Vector2, dx2dy2: Vector2, dxdy: Vector2) -> &mut Self {
        self.0.push(Segment::Curve(dx1dy1, dx2dy2, dxdy));
        self
    }

    pub fn end(&mut self) -> Path {
        Path(std::mem::take(&mut self.0))
    }

    pub fn close(&mut self) -> Path {
        self.0.push(Segment::Close);
        Path(std::mem::take(&mut self.0))
    }
}

impl Path {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn segments(&self) -> &[Segment] {
        self.0.as_ref()
    }

    pub fn into_segments(self) -> Vec<Segment> {
        self.0
    }

    pub fn cursor(&self, index: usize) -> Vector2 {
        // NOTE sequence is never empty because Path can only be initialized via the `start` method
        // of the PathBuilder. Thus, we can extract the starting point which might be needed if the
        // last element is a Close section. Unwrap is also fine because Segment::cursor() cannot
        // return none when the Segment is a MoveTo variant, which is always the case.
        let start = self.0[0].cursor(Vector2::zeros()).unwrap();
        self.0
            .iter()
            .skip(1)
            .take(index)
            .fold(start, |acc, x| x.cursor(acc).unwrap_or(start))
    }

    pub fn into_raw(self) -> raw::Path {
        let mut path = raw::Path::new();
        let attributes = path.get_attributes_mut();
        self.to_attributes(attributes);
        path
    }
}

impl From<&Path> for raw::Value {
    fn from(path: &Path) -> raw::Value {
        path.segments()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
            .into()
    }
}

impl ToAttributes for Path {
    fn to_attributes(&self, attributes: &mut raw::Attributes) {
        attributes.insert(keys::PATH.into(), raw::Value::from(self));
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
            path.segments(),
            &[
                Segment::MoveTo(Vector2::new(1.0, 2.0)),
                Segment::MoveTo(Vector2::zeros()),
                Segment::Move(Vector2::zeros()),
                Segment::LineTo(Vector2::zeros()),
                Segment::Line(Vector2::zeros()),
                Segment::HorizontalLineTo(0.0),
                Segment::HorizontalLine(0.0),
                Segment::VerticalLineTo(0.0),
                Segment::VerticalLine(0.0),
                Segment::CurveTo(Vector2::zeros(), Vector2::zeros(), Vector2::zeros()),
                Segment::Curve(Vector2::zeros(), Vector2::zeros(), Vector2::zeros()),
                Segment::Close,
            ]
        );
    }

    #[test]
    fn cursor() {
        let origin = Vector2::new(1.0, -2.0);
        let path = PathBuilder::start(origin).end();
        assert_eq!(path.len(), 1);
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
        assert_eq!(path.cursor(path.len() - 1), origin);
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
            raw::Value::from(&path).deref(),
            raw::Value::from("M -1.75 -2.5 v 4 h -12.34 m 1 -1 C 100 0, -0 -200, 0 0 Z").deref(),
        );
    }
}
