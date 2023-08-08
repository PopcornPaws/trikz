use super::{Element, PathBuilder};
use crate::svgutils::{keys, raw};
use crate::{Scalar, Vector2};

pub struct Marker;

const DEFAULT_ORIENT: &str = "auto-start-reverse";
//const DEFAULT_REF_X: Scalar = 1.5;
//const DEFAULT_REF_Y: usize = 10.0;
const DEFAULT_VIEW_BOX: &str = "0 -5 10 10";

pub const ARROW_X: Scalar = 10.0;
pub const ARROW_Y: Scalar = 3.0;

pub const ARROW_ID: [u8; 4] = *b"arow";

impl Element<Marker> {
    fn with_child(self, id: [u8; 4], elem: raw::Element) -> Self {
        self.insert_multi(
            [
                keys::MARKER_ID,
                keys::MARKER_ORIENT,
                keys::VIEW_BOX,
                keys::REF_X,
            ]
            .into_iter()
            .zip([
                raw::Value::from(hex::encode(id)),
                raw::Value::from(DEFAULT_ORIENT),
                raw::Value::from(DEFAULT_VIEW_BOX),
                raw::Value::from(ARROW_X),
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
            .line_to(-ARROW_Y * Vector2::y())
            .line_to(ARROW_X * Vector2::x())
            .line_to(ARROW_Y * Vector2::y())
            .close()
            .into_raw();

        self.with_child(ARROW_ID, raw::Element::from(path))
    }

    pub fn circle(self) -> Self {
        todo!() //circle markers on a line
    }

    pub fn square(self) -> Self {
        todo!() // square markers on a line
    }

    pub fn id(&self) -> u32 {
        let id = self.get_raw(keys::MARKER_ID);
        let mut id_bytes = [0u8; 4];
        hex::decode_to_slice(id, &mut id_bytes).expect("invalid marker id");
        u32::from_le_bytes(id_bytes)
    }
}

#[cfg(test)]
mod test {
    // TODO
}
