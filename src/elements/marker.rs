use super::Element;
use crate::{Scalar, Vector2};

pub struct Marker;

impl Element<Marker> {
    pub fn add<T>(id: usize, elem: Element<T>) -> Self {
        todo!();
        // TODO Add elem as a child node of this node
        // and insert default attributes
        //Self {
        //    id,
        //    height: 3.0, // default
        //    width: 3.0,  // default
        //    elem,
        //}
        //attributes.insert(keys::MARKER_ID.into(), self.id.clone().into());
        //attributes.insert(keys::MARKER_HEIGHT.into(), self.height.into());
        //attributes.insert(keys::MARKER_WIDTH.into(), self.width.into());
        //attributes.insert(keys::MARKER_ORIENT.into(), "auto-start-reverse".into());
    }

    pub fn height(self, height: Scalar) -> Self {
        todo!();
    }

    pub fn width(self, width: Scalar) -> Self {
        todo!();
    }

    pub fn arrow(self) -> Self {
        todo!();
        //let elem = PathBuilder::start(Vector2::zeros())
        //    .line_to(Vector2::new(10.0, 5.0))
        //    .line_to(Vector2::new(0.0, 10.0))
        //    .close();

        // TODO Self::add(0, Element::<Path>::new(Rc::new(RefCell::new(elem))))
    }
    pub fn circle(self) -> Self {
        todo!() // circle markers on a line
        // TODO Self::add(0, Element::<Circle>::new(Rc::new(RefCell::new(elem))))
    }

    pub fn square(self) -> Self {
        todo!() // square markers on a line
        // TODO Self::add(0, Element::<Rectangle>::new(Rc::new(RefCell::new(elem))))
    }
}

#[cfg(test)]
mod test {
    // TODO
}
