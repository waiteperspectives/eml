use super::eventmodel::*;
use super::svg::{Arrow, Card, CardType, SvgDocument};

fn _ingest_fields_card(card_type: CardType, id: ExpressionId, fields: Vec<Field>) -> Card {
    Card::new(
        id.0.clone(),
        card_type,
        fields
            .iter()
            .map(|f| match f {
                Field::Text(ff) => {
                    format!("{}: {}", ff.name, ff.data)
                }
                Field::Integer(ff) => {
                    format!("{}: {}", ff.name, ff.data.to_string(),)
                }
            })
            .collect::<Vec<String>>(),
    )
}

fn _ingest_table_card(card_type: CardType, id: ExpressionId, tbl: Vec<String>) -> Card {
    Card::new(id.0.clone(), card_type, tbl)
}

pub fn ingest_form(id: ExpressionId, fields: Vec<Field>) -> Card {
    _ingest_fields_card(CardType::Form, id, fields)
}

pub fn ingest_job(id: ExpressionId, fields: Vec<Field>) -> Card {
    _ingest_fields_card(CardType::Job, id, fields)
}

pub fn ingest_command(id: ExpressionId, fields: Vec<Field>) -> Card {
    _ingest_fields_card(CardType::Command, id, fields)
}

pub fn ingest_event(id: ExpressionId, fields: Vec<Field>) -> Card {
    _ingest_fields_card(CardType::Event, id, fields)
}

pub fn ingest_view(id: ExpressionId, tbl: Vec<String>) -> Card {
    _ingest_table_card(CardType::View, id, tbl)
}

// impl this here because it relies on eventmodel stuff
impl SvgDocument {
    pub fn get_card(&self, id: ExpressionId) -> Card {
        self.cards.iter().find(|c| c.id == id.0).unwrap().clone()
    }

    pub fn ingest_expressions(&mut self, expressions: Vec<Expression>) {
        for expr in expressions {
            match expr {
                Expression::Form(id, fields) => {
                    let card = ingest_form(id, fields);
                    self.cards.push(card);
                }
                Expression::Job(id, fields) => {
                    let card = ingest_job(id, fields);
                    self.cards.push(card);
                }
                Expression::Command(id, fields) => {
                    let card = ingest_command(id, fields);
                    self.cards.push(card);
                }
                Expression::Event(id, fields) => {
                    let card = ingest_event(id, fields);
                    self.cards.push(card);
                }
                Expression::View(id, body) => match body {
                    Body::TableBody(tbl) => {
                        let card = ingest_view(id, tbl);
                        self.cards.push(card);
                    }
                    Body::UseBody(exprid) => {
                        let card = self.get_card(exprid);
                        self.cards.push(card);
                    }
                },
                Expression::Flow(_, expr_ids) => {
                    let (_, from_ids) = expr_ids.split_last().unwrap();
                    let (_, to_ids) = expr_ids.split_first().unwrap();
                    for (from, to) in from_ids.iter().zip(to_ids.iter()) {
                        let arrow = Arrow::new(
                            self.cards.iter().find(|c| c.id == from.0).unwrap().clone(),
                            self.cards.iter().find(|c| c.id == to.0).unwrap().clone(),
                        );
                        self.arrows.push(arrow);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg::*;
    use crate::utils::newid;

    #[test]
    fn test_ingest() {
        let expected = "<svg xmlns='http://www.w3.org/2000/svg' width='1650' height='1350'><defs><marker id='triangle' viewBox='0 0 10 10' refX='0' refY='5' markerUnits='strokeWidth' markerWidth='10' markerHeight='8' orient='auto'><path d='M 0 0 L 10 5 L 0 10 z' /></marker></defs><g id='swimlane' stroke='black' stroke-width='3' ><line id='swimlane_top' x1='0' y1='450' x2='1650' y2='450' /><line id='swimlane_bottom' x1='0' y1='900' x2='1650' y2='900' /></g><path d='M 450 225 Q 750 225 750 585' stroke='black' stroke-width='2' fill='none' marker-end='url(#triangle)' /><path d='M 750 750 Q 750 1275 1035 1275' stroke='black' stroke-width='2' fill='none' marker-end='url(#triangle)' /><rect id='AddTodoForm' stroke='black' stroke-width='2' x='150' y='150' height='150' width='300' fill='#ffffff' /><text transform='translate(162.5 162.5)' x='0' y='0' style='white-space: pre;' font-family='monospace' ><tspan x='0' dy='1rem' font-size='larger' font-weight='bold'>AddTodoForm</tspan><tspan x='0' dy='1rem'>==========</tspan><tspan x='0' dy='1rem'>key: todo1</tspan><tspan x='0' dy='1rem'>description: Wake up</tspan></text><rect id='AddTodo' stroke='black' stroke-width='2' x='600' y='600' height='150' width='300' fill='#60b3f7' /><text transform='translate(612.5 612.5)' x='0' y='0' style='white-space: pre;' font-family='monospace' ><tspan x='0' dy='1rem' font-size='larger' font-weight='bold'>AddTodo</tspan><tspan x='0' dy='1rem'>==========</tspan><tspan x='0' dy='1rem'>key: todo1</tspan><tspan x='0' dy='1rem'>description: Wake up</tspan></text><rect id='TodoAdded' stroke='black' stroke-width='2' x='1050' y='1200' height='150' width='300' fill='#f7a660' /><text transform='translate(1062.5 1212.5)' x='0' y='0' style='white-space: pre;' font-family='monospace' ><tspan x='0' dy='1rem' font-size='larger' font-weight='bold'>TodoAdded</tspan><tspan x='0' dy='1rem'>==========</tspan><tspan x='0' dy='1rem'>key: todo1</tspan><tspan x='0' dy='1rem'>description: Wake up</tspan></text></svg>".to_string();
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
                            data: "todo1".to_string(),
                        }),
                        Field::Text(TextField {
                            name: "description".to_string(),
                            data: "Wake up".to_string(),
                        }),
                    ],
                ),
                Expression::Command(
                    ExpressionId("AddTodo".to_string()),
                    vec![
                        Field::Text(TextField {
                            name: "key".to_string(),
                            data: "todo1".to_string(),
                        }),
                        Field::Text(TextField {
                            name: "description".to_string(),
                            data: "Wake up".to_string(),
                        }),
                    ],
                ),
                Expression::Event(
                    ExpressionId("TodoAdded".to_string()),
                    vec![
                        Field::Text(TextField {
                            name: "key".to_string(),
                            data: "todo1".to_string(),
                        }),
                        Field::Text(TextField {
                            name: "description".to_string(),
                            data: "Wake up".to_string(),
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
        let observed = doc.render();
        assert_eq!(observed, expected);
    }
}
