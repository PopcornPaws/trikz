use crate::elements::{Marker, Path};
use crate::Vector2;

pub struct Arrow<'a> {
    path: Path,
    marker: &'a Marker<Path>,
}

impl<'a> Arrow<'a> {
    pub fn new(path: Path, marker: &'a Marker<Path>) -> Self {
        Self { path, marker }
    }
}

impl ToAttributes for Arrow<'_> {
    fn to_attributes(&self, attributes: &mut Attributes) {
        attributes.insert(keys::PATH.into(), Value::from(self.path));
        attributes.insert(
    }
}

pub struct ArrowHead(Marker<Path>);

impl AsRef<Marker<Path>> for ArrowHead {
    fn as_ref(&self) -> &Marker<Path> {
        &self.0
    }
}

impl Default for ArrowHead {
    fn default() -> Self {
        let elem = PathBuilder::start(Vector2::zeros())
            .line_to(Vector2::new(10.0, 5.0))
            .line_to(Vector2::new(0.0, 10.0))
            .close();

        Self(Marker::new("arrow".to_string(), elem))
    }
}
