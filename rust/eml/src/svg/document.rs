use super::arrow::Arrow;
use super::card::{Card, CardType};
use super::swimlane::Swimlane;

pub struct SvgConfig {
    pub pad: f64,
    pub card_height: f64,
    pub card_width: f64,
}

pub struct SvgDocument {
    pub id: String,
    pub width: f64,
    pub height: f64,
    pub cards: Vec<Card>,
    pub arrows: Vec<Arrow>,
    pub swimlane: Swimlane,
}

impl SvgDocument {
    pub fn add_cards(&mut self, cards: Vec<Card>) {
        for card in cards.into_iter() {
            self.cards.push(card);
        }
    }

    pub fn add_arrows(&mut self, arrows: Vec<Arrow>) {
        for arrow in arrows.into_iter() {
            self.arrows.push(arrow);
        }
    }

    pub fn set_dimensions(&mut self, config: &SvgConfig) {
        self.height = config.card_height * 3f64 + config.pad * 6f64;
        self.width =
            (config.pad * 2f64) + (config.pad + config.card_width) * self.cards.len() as f64;
        self.swimlane.top = config.card_height + config.pad * 2f64;
        self.swimlane.bottom = config.card_height * 2f64 + config.pad * 4f64;
        self.swimlane.width = self.width;
        let mut x_pos = config.pad;
        for card in self.cards.iter_mut() {
            match card.card_type {
                CardType::Form => {
                    card.set_origin(x_pos, config.pad);
                    card.set_dimensions(config.card_width, config.card_height, config.pad);
                    x_pos += config.card_width + config.pad;
                }
                CardType::Job => {
                    card.set_origin(x_pos, config.pad);
                    card.set_dimensions(config.card_width, config.card_height, config.pad);
                    x_pos += config.card_width + config.pad;
                }
                CardType::Command => {
                    card.set_origin(x_pos, config.card_height + config.pad * 3f64);
                    card.set_dimensions(config.card_width, config.card_height, config.pad);
                    x_pos += config.card_width + config.pad;
                }
                CardType::View => {
                    card.set_origin(x_pos, config.card_height + config.pad * 3f64);
                    card.set_dimensions(config.card_width, config.card_height, config.pad);
                    x_pos += config.card_width + config.pad;
                }
                CardType::Event => {
                    card.set_origin(x_pos, config.card_height * 3f64 + config.pad * 5f64);
                    card.set_dimensions(config.card_width, config.card_height, config.pad);
                    x_pos += config.card_width + config.pad;
                }
            };
        }
        for arrow in self.arrows.iter_mut() {
            arrow.set_dimensions(&self.cards);
        }
    }

    pub fn render(self) -> String {
        format!(
            "\
            <svg xmlns='http://www.w3.org/2000/svg' width='{width}' height='{height}'>\
              <defs>\
                <marker id='triangle' viewBox='0 0 10 10' refX='0' refY='5' markerUnits='strokeWidth' markerWidth='10' markerHeight='8' orient='auto'>\
                  <path d='M 0 0 L 10 5 L 0 10 z' />\
                </marker>\
              </defs>\
              <g id='swimlane' stroke='black' stroke-width='3' >\
                <line id='swimlane_top' x1='0' y1='{top}' x2='{width}' y2='{top}' />\
                <line id='swimlane_bottom' x1='0' y1='{bottom}' x2='{width}' y2='{bottom}' />\
              </g>\
              {arrows}\
              {cards}\
            </svg>\
            ",
            width = self.width,
            height = self.height,
            top = self.swimlane.top,
            bottom = self.swimlane.bottom,
            cards = self
                .cards
                .iter()
                .map(|el| { el.render() })
                .collect::<Vec<String>>()
                .join(""),
            arrows = self
                .arrows
                .iter()
                .map(|el| { el.render() })
                .collect::<Vec<String>>()
                .join(""),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::newid;

    #[test]
    fn test_document() {
        let expected = "<svg xmlns='http://www.w3.org/2000/svg' width='1650' height='1350'><defs><marker id='triangle' viewBox='0 0 10 10' refX='0' refY='5' markerUnits='strokeWidth' markerWidth='10' markerHeight='8' orient='auto'><path d='M 0 0 L 10 5 L 0 10 z' /></marker></defs><g id='swimlane' stroke='black' stroke-width='3' ><line id='swimlane_top' x1='0' y1='450' x2='1650' y2='450' /><line id='swimlane_bottom' x1='0' y1='900' x2='1650' y2='900' /></g><path d='M 450 225 Q 750 225 750 585' stroke='black' stroke-width='2' fill='none' marker-end='url(#triangle)' /><path d='M 750 750 Q 750 1275 1035 1275' stroke='black' stroke-width='2' fill='none' marker-end='url(#triangle)' /><rect id='form1' stroke='black' stroke-width='2' x='150' y='150' height='150' width='300' fill='#ffffff' /><text transform='translate(162.5 162.5)' x='0' y='0' style='white-space: pre;' font-family='monospace' ><tspan x='0' dy='1rem' font-size='larger' font-weight='bold'>form1</tspan><tspan x='0' dy='1rem'>==========</tspan><tspan x='0' dy='1rem'>Test Form</tspan></text><rect id='cmd1' stroke='black' stroke-width='2' x='600' y='600' height='150' width='300' fill='#60b3f7' /><text transform='translate(612.5 612.5)' x='0' y='0' style='white-space: pre;' font-family='monospace' ><tspan x='0' dy='1rem' font-size='larger' font-weight='bold'>cmd1</tspan><tspan x='0' dy='1rem'>==========</tspan><tspan x='0' dy='1rem'>Test Cmd</tspan></text><rect id='evt1' stroke='black' stroke-width='2' x='1050' y='1200' height='150' width='300' fill='#f7a660' /><text transform='translate(1062.5 1212.5)' x='0' y='0' style='white-space: pre;' font-family='monospace' ><tspan x='0' dy='1rem' font-size='larger' font-weight='bold'>evt1</tspan><tspan x='0' dy='1rem'>==========</tspan><tspan x='0' dy='1rem'>Test Event</tspan><tspan x='0' dy='1rem'>+ foo: str</tspan><tspan x='0' dy='1rem'>+ bar: str</tspan></text></svg>".to_string();
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
        let observed = doc.render();

        assert_eq!(expected, observed);
    }
}
