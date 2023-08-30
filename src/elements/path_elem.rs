use super::marker::ARROW_X;
use super::path::{Path, PathBuilder};
use super::Element;
use crate::svgutils::{keys, raw};
use crate::{Scalar, Vector2};

const ARROW_SHIFT: Scalar = ARROW_X * 0.75;

impl Element<Path> {
    pub fn raw(self, path: Path) -> Self {
        self.insert(keys::PATH, raw::Value::from(&path));
        self
    }

    pub fn arrow(self, start: Vector2, end: Vector2) -> Self {
        let end_corrected = arrow_end_correction(start, end);
        let path = PathBuilder::start(start).line_to(end_corrected).end();
        self.raw(path)
    }

    // horizontal, then vertical elbow: _|
    pub fn arrow_hv(start: Vector2, end: Vector2) -> Self {
        todo!();
    }

    // vertical, then horizontal elbow: |_
    pub fn arrow_vh(start: Vector2, end: Vector2) -> Self {
        todo!();
    }

    // double elbow: |_| or |‾|
    pub fn arrow_hvh(self, start: Vector2, end: Vector2, y_shift: Scalar) -> Self {
        let end_corner = Vector2::new(end[0], start[1] + y_shift);
        let end_corrected = arrow_end_correction(end_corner, end);
        let path = PathBuilder::start(start)
            .vline(y_shift)
            .line_to(end_corner)
            .line_to(end_corrected)
            .end();
        self.raw(path)
    }

    // double elbow: ‾|_
    pub fn arrow_vhv(start: Vector2, end: Vector2, shift: Scalar) -> Self {
        todo!();
    }
}

fn arrow_end_correction(start: Vector2, end: Vector2) -> Vector2 {
    let diff = end - start;
    let length = diff.norm();
    let length_ratio = (length - ARROW_SHIFT) / length;
    start + diff * length_ratio
}

#[test]
fn arrow_end() {
    use crate::assert_relative_eq;

    let len: Scalar = 100.0;
    let start = Vector2::zeros();

    let end = len * Vector2::x();
    assert_relative_eq!(
        arrow_end_correction(start, end),
        (len - ARROW_SHIFT) * Vector2::x()
    );

    let end = -len * Vector2::x();
    assert_relative_eq!(
        arrow_end_correction(start, end),
        -(len - ARROW_SHIFT) * Vector2::x()
    );

    let end = len * Vector2::y();
    assert_relative_eq!(
        arrow_end_correction(start, end),
        (len - ARROW_SHIFT) * Vector2::y()
    );

    let end = -len * Vector2::y();
    assert_relative_eq!(
        arrow_end_correction(start, end),
        -(len - ARROW_SHIFT) * Vector2::y()
    );

    // -30 degrees
    let coeff = (3.0 as Scalar).sqrt() / 2.0;
    let len: Scalar = 2.0 * ARROW_SHIFT;
    let end = Vector2::new(coeff * len, -len / 2.0);
    assert_relative_eq!(arrow_end_correction(start, end), end / 2.0);

    let start = Vector2::new(-50.0, -50.0);
    let end = Vector2::new(50.0, 50.0);
    let coeff = (2.0 as Scalar).sqrt() / 2.0;
    assert_relative_eq!(
        arrow_end_correction(start, end),
        end - ARROW_SHIFT * coeff * Vector2::new(1.0, 1.0)
    );
}
