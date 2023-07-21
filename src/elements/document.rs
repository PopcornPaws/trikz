use super::*;

pub struct Document {
    elements: Vec<ElemRef>,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    fn add<T, E: Deref<Target = raw::Element>>(&mut self, elem: E) -> Element<T> {
        // cloning should be cheap because the element is empty, but we need it as an Element, not
        // as a specific type
        self.elements
            .push(Rc::new(RefCell::new(elem.deref().clone())));
        let index = self.elements.len() - 1;
        Element::new(Rc::clone(&self.elements[index]))
    }

    pub fn circle(&mut self) -> Element<Circle> {
        self.add(raw::Circle::new())
    }

    pub fn rectangle(&mut self) -> Element<Rectangle> {
        self.add(raw::Rectangle::new())
    }

    pub fn line(&mut self) -> Element<Line> {
        self.add(raw::Line::new())
    }

    pub fn finalize(self) -> raw::Document {
        let mut document = raw::Document::new();
        self.elements.into_iter().for_each(|elem| {
            document.append(Rc::into_inner(elem).unwrap().into_inner());
        });
        document
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}
