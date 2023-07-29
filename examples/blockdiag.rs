use trikz::anchor::AnchorT;
use trikz::elements::Document;
use trikz::style::{Color, Stroke, Style};
use trikz::*; // TODO should be prelude (AnchorT as well)

fn main() {
    let width = cm!(2);
    let height = cm!(1.25);

    let mut document = Document::new();

    {
        let arrow_marker = document.marker().arrow();

        let stroke = Stroke::new().color(Color::Black).width(mm!(0.57));
        let arrow = Stroke::new()
            .color(Color::Black)
            .width(mm!(1.5))
            .marker_end(arrow_marker.id());

        let rect_style = Style::new().fill(Color::Green).stroke(stroke.clone());
        let circ_style = Style::new().fill(Color::Blue).stroke(stroke.clone());
        let arrow_style = Style::new().stroke(arrow);

        let controller = document
            .rectangle()
            .width(width)
            .height(height)
            .with_style(&rect_style);

        let plant = document
            .rectangle()
            .like(controller.clone()) // rc clone
            .at(controller.right(2.5 * width));

        let sum = document
            .circle()
            .at(controller.left(2.5 * width))
            .radius(0.5 * height)
            .with_style(&circ_style);

        // arrows
        let _a_ref = document
            .line()
            .start(sum.left(2.5 * width))
            .end(sum.west())
            .with_style(&arrow_style);

        let _a_err = document
            .line()
            .start(sum.east())
            .end(controller.west())
            .with_style(&arrow_style);

        let _a_inp = document
            .line()
            .start(controller.east())
            .end(plant.west())
            .with_style(&arrow_style);

        let _a_out = document
            .line()
            .start(plant.east())
            .end(plant.right(2.5 * width))
            .with_style(&arrow_style);
    }

    document.save("examples/blockdiag.svg");
}
