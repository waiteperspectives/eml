mod eventmodel;
mod ingest;
mod parse;
mod svg;
mod utils;

use indoc::indoc;
use parse::parse;
use svg::*;
use utils::newid;

fn demo() {
    let mut doc = SvgDocument {
        id: newid(),
        width: 1000f64,
        height: 1000f64,
        cards: Vec::new(),
        arrows: Vec::new(),
        swimlane: Swimlane::new(),
    };
    let input = indoc! {r#"
        # eml: 0.0.1

        form AddTodoForm {
            key: "todo1"
            text: "Wake up"
        }
    "#};
    let model = parse(input).unwrap();
    doc.ingest_expressions(model.expressions);
    let config = SvgConfig {
        pad: 150f64,
        card_width: 300f64,
        card_height: 150f64,
    };
    doc.set_dimensions(&config);
    let svg_string = doc.render();
    println!("{}", svg_string);
}

fn main() {
    demo()
}
