mod eventmodel;
mod svg;
mod utils;

// use eventmodel::EventModel;
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
    doc.cards.push(Card::new(
        "form1".to_string(),
        CardType::Form,
        vec!["Test Form".to_string()],
    ));
    doc.cards.push(Card::new(
        "cmd1".to_string(),
        CardType::Command,
        vec!["Test Cmd".to_string()],
    ));
    doc.arrows.push(Arrow::new(
        doc.cards.iter().find(|c| c.id == "form1").unwrap().clone(),
        doc.cards.iter().find(|c| c.id == "cmd1").unwrap().clone(),
    ));
    doc.cards.push(Card::new(
        "evt1".to_string(),
        CardType::Event,
        vec![
            "Test Event".to_string(),
            "+ foo: str".to_string(),
            "+ bar: str".to_string(),
        ],
    ));
    doc.arrows.push(Arrow::new(
        doc.cards.iter().find(|c| c.id == "cmd1").unwrap().clone(),
        doc.cards.iter().find(|c| c.id == "evt1").unwrap().clone(),
    ));
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
