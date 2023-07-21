/*
use trikz::elements::{Circle, Marker, Rectangle};
use trikz::{px, cm};
use trikz::style::{Color, Stroke, Style};

fn main() {
    // define styles
    let stroke = Stroke::new().color(Color::Black).width(px!(5));
    let rect_style = Style::fill(Color::Green).stroke(stroke.clone());
    let circ_style = Style::fill(Color::Red).stroke(stroke);
    let arrow = Marker::arrow();

    // define elements
    let plant = Rectangle::new().width(cm!(2)).height(cm!(1.25));
    let controller = plant.at(plant.left(...));
    let sum = Circle::new().at(controller.left(...));

    // define arrows
    let reference = Arrow::from(sum.left(xshift...)).to(sum.west()); // label
    let error = Arrow::from(sum.east()).to(controller.west()); // label
    let input = Arrow::from(controller.east()).to(plant.west()); // label
    let output = Arrow::from(controller.east()).to(plant.west()); // label
    let feedback = Arrow::from(...).down_left(...).left_up(...); // |__|

    let document = SvgDocument::new();
    // add elements with dedicated style to document
    // export to svg
}
*/

fn main() {
    todo!()
    // TODO something like this would be better
    /*
    let width = cm!(2);
    let height = cm!(1.25);

    let mut document = Document::new();

    let rect_style = Style::new();
    let circ_style = Style::new();

    let arrow = document.marker().arrow();

    let controller = document
        .rectangle()
        .width(width)
        .height(height)
        .with_style(rect_style)
        .label(...);

    let plant = document
        .rectangle()
        .like(controller)
        .at(controller.right(2.5 * width));

    let sum = document
        .circle()
        .at(controller.left(2.5 * width))
        .radius(0.5 * height)
        .with_style(circ_style);

    let reference = document
        .line()
        .start(sum.left(2.5 * width))
        .end(sum.west())
        .with_style()
        .marker(arrow);
    */
}
