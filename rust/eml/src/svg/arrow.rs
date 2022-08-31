use super::{Card, CardType, Point};
use crate::utils::newid;

fn det(a: (f64, f64), b: (f64, f64)) -> f64 {
    a.0 * b.1 - a.1 * b.0
}

fn line_intersection(
    line1: ((f64, f64), (f64, f64)),
    line2: ((f64, f64), (f64, f64)),
) -> (f64, f64) {
    let xdiff = (line1.0 .0 - line1.1 .0, line2.0 .0 - line2.1 .0);
    let ydiff = (line1.0 .1 - line1.1 .1, line2.0 .1 - line2.1 .1);
    let div = det(xdiff, ydiff);
    if div == 0f64 {
        panic!("Lines do not intersect");
    }
    let d = (det(line1.0, line1.1), det(line2.0, line2.1));
    let x = det(d, xdiff) / div;
    let y = det(d, ydiff) / div;
    (x, y)
}

pub struct Arrow {
    pub begin_at: Card,
    pub end_at: Card,
    pub id: String,
}

impl Arrow {
    pub fn new(begin_at: Card, end_at: Card) -> Self {
        Arrow {
            begin_at,
            end_at,
            id: newid(),
        }
    }

    pub fn set_dimensions(&mut self, cards: &Vec<Card>) {
        self.begin_at = cards
            .iter()
            .find(|c| c.id == self.begin_at.id)
            .unwrap()
            .clone();
        self.end_at = cards
            .iter()
            .find(|c| c.id == self.end_at.id)
            .unwrap()
            .clone();
    }

    pub fn get_points(&self) -> (Point, Point) {
        match (&self.begin_at.card_type, &self.end_at.card_type) {
            (CardType::Job, CardType::Command) => {
                (self.begin_at.right_anchor(), self.end_at.top_anchor())
            }
            (CardType::Form, CardType::Command) => {
                (self.begin_at.right_anchor(), self.end_at.top_anchor())
            }
            (CardType::Command, CardType::Event) => {
                (self.begin_at.bottom_anchor(), self.end_at.left_anchor())
            }
            (CardType::Event, CardType::View) => {
                (self.begin_at.right_anchor(), self.end_at.bottom_anchor())
            }
            (CardType::View, CardType::Job) => {
                (self.begin_at.top_anchor(), self.end_at.left_anchor())
            }
            (CardType::View, CardType::Form) => {
                (self.begin_at.top_anchor(), self.end_at.left_anchor())
            }
            (_, _) => panic!("Illegal Arrow!"),
        }
    }

    pub fn render(&self) -> String {
        let (left, right) = self.get_points();
        let control_point = match &self.begin_at.card_type {
            CardType::Job | CardType::Form => {
                let vertical = ((right.x, right.y), (right.x, 99999f64));
                let horizontal = ((left.x, left.y), (0f64, left.y));
                line_intersection(vertical, horizontal)
            }
            CardType::Command => {
                let vertical = ((left.x, left.y), (left.x, 99999f64));
                let horizontal = ((right.x, right.y), (0f64, right.y));
                line_intersection(vertical, horizontal)
            }
            CardType::Event => {
                let vertical = ((right.x, right.y), (right.x, 99999f64));
                let horizontal = ((left.x, left.y), (0f64, left.y));
                line_intersection(vertical, horizontal)
            }
            CardType::View => {
                let vertical = ((left.x, left.y), (left.x, 99999f64));
                let horizontal = ((right.x, right.y), (0f64, right.y));
                line_intersection(vertical, horizontal)
            }
        };
        format!("\
        <path d='M {left_x} {left_y} Q {cp0} {cp1} {right_x} {right_y}' stroke='black' stroke-width='2' fill='none' />\
        ",
        left_x=left.x,
        left_y=left.y,
        cp0=control_point.0,
        cp1=control_point.1,
        right_x=right.x,
        right_y=right.y,
        )
    }
}
