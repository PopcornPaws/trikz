use super::{PathBuilder, Element};
use crate::svgutils::{keys, raw};
use crate::{Scalar, Vector2};

pub struct Marker;

const DEFAULT_HEIGHT: usize = 10;
const DEFAULT_WIDTH: usize = 10;
const DEFAULT_ORIENT: &str = "auto-start-reverse";

impl Element<Marker> {
    fn with_child(self, id: usize, elem: raw::Element) -> Self {
        self.insert_multi(
            [
                keys::MARKER_ID.into(),
                keys::MARKER_HEIGHT.into(),
                keys::MARKER_WIDTH.into(),
                keys::MARKER_ORIENT.into(),
            ]
            .into_iter()
            .zip([
                raw::Value::from(id),
                raw::Value::from(DEFAULT_HEIGHT),
                raw::Value::from(DEFAULT_WIDTH),
                raw::Value::from(DEFAULT_ORIENT),
            ]),
        );
        self.add_child(elem);
        self
    }

    pub fn height(self, height: Scalar) -> Self {
        self.insert(keys::MARKER_HEIGHT.into(), raw::Value::from(height));
        self
    }

    pub fn width(self, width: Scalar) -> Self {
        self.insert(keys::MARKER_WIDTH.into(), raw::Value::from(width));
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
