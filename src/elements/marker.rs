use super::{Element, PathBuilder};
use crate::svgutils::{keys, raw};
use crate::{Scalar, Vector2};

pub struct Marker;

const DEFAULT_HEIGHT: usize = 3;
const DEFAULT_WIDTH: usize = 3;
const DEFAULT_ORIENT: &str = "auto-start-reverse";
const DEFAULT_REF_X: usize = 5;
const DEFAULT_REF_Y: usize = 5;
const DEFAULT_VIEW_BOX: &str = "0 0 10 10";

impl Element<Marker> {
    fn with_child(self, id: usize, elem: raw::Element) -> Self {
        self.insert_multi(
            [
                keys::MARKER_ID,
                keys::MARKER_HEIGHT,
                keys::MARKER_WIDTH,
                keys::MARKER_ORIENT,
                keys::VIEW_BOX,
                keys::REF_X,
                keys::REF_Y,
            ]
            .into_iter()
            .zip([
                raw::Value::from(id),
                raw::Value::from(DEFAULT_HEIGHT),
                raw::Value::from(DEFAULT_WIDTH),
                raw::Value::from(DEFAULT_ORIENT),
                raw::Value::from(DEFAULT_VIEW_BOX),
                raw::Value::from(DEFAULT_REF_X),
                raw::Value::from(DEFAULT_REF_Y),
            ]),
        );
        self.add_child(elem);
        self
    }

    pub fn height(self, height: Scalar) -> Self {
        self.insert(keys::MARKER_HEIGHT, height);
        self
    }

    pub fn width(self, width: Scalar) -> Self {
        self.insert(keys::MARKER_WIDTH, width);
        self
    }

    pub fn arrow(self) -> Self {
        let path = PathBuilder::start(Vector2::zeros())
            .line_to(Vector2::new(10.0, 5.0))
            .line_to(Vector2::new(0.0, 10.0))
            .close()
            .into_raw();

        self.with_child(0, raw::Element::from(path))
    }

    pub fn circle(self) -> Self {
        todo!() //circle markers on a line
    }

    pub fn square(self) -> Self {
        todo!() // square markers on a line
    }

    pub fn id(&self) -> usize {
        self.get(keys::MARKER_ID)
    }
}

#[cfg(test)]
mod test {
    // TODO
}
