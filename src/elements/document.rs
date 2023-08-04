use super::*;

#[derive(Default)]
pub struct Document {
    elements: Vec<ElemRef>,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    fn add<T, E: Into<raw::Element>>(&mut self, elem: E) -> Element<T> {
        self.elements.push(Rc::new(RefCell::new(elem.into())));
        let index = self.elements.len() - 1;
        Element::new(Rc::clone(&self.elements[index]))
    }

    pub fn circle(&mut self) -> Element<Circle> {
        self.add(raw::Circle::new())
    }

    pub fn line(&mut self) -> Element<Line> {
        self.add(raw::Line::new())
    }

    //pub fn arrow(&mut self) -> Element<Arrow> {
    //    self.add(raw::Path::new())
    //}

    //pub fn path(&mut self) -> Element<Path> {
    //    self.add(raw::Path::new())
    //}

    pub fn marker(&mut self) -> Element<Marker> {
        self.add(raw::Marker::new())
    }

    pub fn rectangle(&mut self) -> Element<Rectangle> {
        self.add(raw::Rectangle::new())
    }

    pub fn finalize(self) -> raw::Document {
        let mut document = raw::Document::new().set("viewBox", "-400 -50 850 100");
        self.elements.into_iter().for_each(|elem| {
            document.append(Rc::into_inner(elem).unwrap().into_inner());
        });
        document
    }

    pub fn save<P: AsRef<std::path::Path>>(self, path: P) {
        let document = self.finalize();
        svg::save(path, &document).expect("failed to save to svg");
    }

    #[cfg(feature = "pdf")]
    pub fn save_pdf<P: AsRef<std::path::Path>>(self, path: P) {
        let document = self.finalize();
        let pdf = svg2pdf::convert_str(&document.to_string(), svg2pdf::Options::default())
            .expect("invalid svg string");
        std::fs::write(path, pdf).expect("failed to save to svg");
    }
}
