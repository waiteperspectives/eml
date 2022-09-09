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

        command AddTodo {
            key: "todo1"
            text: "Wake up"
        }

        event TodoAdded {
            key: "todo1"
            text: "Wake up"
        }

        view TodoList {
          |   key   |  text         |  state   |
          |---------|---------------|----------|
          | t1      | Wake up       | done     |
          | t2      | Eat breakfast | todo     |
          | t3      | Go to school  | todo     |
        }

        flow { AddTodoForm => AddTodo => TodoAdded => TodoList }

        view OtherList {
          |   key   |  text         |
          |---------|---------------|
          | t1      | Wake up       |
          | t2      | Eat breakfast |
          | t3      | Go to school  |
        }

        flow { TodoAdded => OtherList }
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
