use crate::elements::{Path, PathBuilder};
use crate::svgutils::{keys, Attributes, IntoElem, Marker as SvgMarker, Node, ToAttributes};
use crate::{Scalar, Vector2};

pub struct Marker<T> {
    id: String,
    height: Scalar,
    width: Scalar,
    orient: Option<Scalar>, // optional angle, default: auto-start-reverse
    elem: T,
}

impl<T> Marker<T> {
    pub fn new(id: String, elem: T) -> Self {
        Self {
            id,
            height: 3.0, // default
            width: 3.0,  // default
            orient: None,
            elem,
        }
    }

    pub fn height(self, height: Scalar) -> Self {
        Self {
            id: self.id,
            height,
            width: self.width,
            orient: self.orient,
            elem: self.elem,
        }
    }

    pub fn width(self, width: Scalar) -> Self {
        Self {
            id: self.id,
            height: self.height,
            width,
            orient: self.orient,
            elem: self.elem,
        }
    }

    pub fn orient(self, angle: Scalar) -> Self {
        Self {
            id: self.id,
            height: self.height,
            width: self.width,
            orient: Some(angle),
            elem: self.elem,
        }
    }
}

impl Marker<Path> {
    pub fn arrow() -> Self {
        let elem = PathBuilder::start(Vector2::zeros())
            .line_to(Vector2::new(10.0, 5.0))
            .line_to(Vector2::new(0.0, 10.0))
            .close();

        Self::new("arrow".to_string(), elem)
    }
}

impl<T> ToAttributes for Marker<T> {
    fn to_attributes(&self, attributes: &mut Attributes) {
        attributes.insert(keys::MARKER_ID.into(), self.id.clone().into());
        attributes.insert(keys::MARKER_HEIGHT.into(), self.height.into());
        attributes.insert(keys::MARKER_WIDTH.into(), self.width.into());
        if let Some(angle) = self.orient {
            attributes.insert(keys::MARKER_ORIENT.into(), angle.into());
        } else {
            attributes.insert(keys::MARKER_ORIENT.into(), "auto-start-reverse".into());
        }
    }
}

impl<T> IntoElem for Marker<T>
where
    T: IntoElem,
    <T as IntoElem>::Elem: Node,
{
    type Elem = SvgMarker;
    type Repr = ();
    fn into_elem(self) -> Self::Elem {
        let mut elem = Self::Elem::new();
        self.to_attributes(elem.get_attributes_mut());
        // add the child element representing
        // the marker's shape
        elem.append(self.elem.into_elem());
        elem
    }
}

#[cfg(test)]
mod test {
    // TODO
}
