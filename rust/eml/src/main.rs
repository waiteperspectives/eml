mod eventmodel;
mod ingest;
mod svg;
mod utils;

use eventmodel::*;
use ingest::*;
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
    let model = EventModel {
        expressions: vec![
            Expression::Form(
                ExpressionId("AddTodoForm".to_string()),
                vec![
                    Field::Text(TextField {
                        name: "key".to_string(),
                        data: vec!["todo1".to_string()],
                    }),
                    Field::Text(TextField {
                        name: "description".to_string(),
                        data: vec!["Wake up".to_string()],
                    }),
                ],
            ),
            Expression::Command(
                ExpressionId("AddTodo".to_string()),
                vec![
                    Field::Text(TextField {
                        name: "key".to_string(),
                        data: vec!["todo1".to_string()],
                    }),
                    Field::Text(TextField {
                        name: "description".to_string(),
                        data: vec!["Wake up".to_string()],
                    }),
                ],
            ),
            Expression::Event(
                ExpressionId("TodoAdded".to_string()),
                vec![
                    Field::Text(TextField {
                        name: "key".to_string(),
                        data: vec!["todo1".to_string()],
                    }),
                    Field::Text(TextField {
                        name: "description".to_string(),
                        data: vec!["Wake up".to_string()],
                    }),
                ],
            ),
            Expression::Flow(
                ExpressionId(newid()),
                vec![
                    ExpressionId("AddTodoForm".to_string()),
                    ExpressionId("AddTodo".to_string()),
                    ExpressionId("TodoAdded".to_string()),
                ],
            ),
        ],
    };
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
