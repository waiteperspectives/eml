use super::line::Line;
use super::rectangle::Rectangle;
use super::text::Text;

pub enum Element {
    LineElement(Line),
    TextElement(Text),
    RectangleElement(Rectangle),
}

impl Element {
    pub fn render(&self) -> String {
        match self {
            Element::LineElement(line) => line.render(),
            Element::TextElement(text) => text.render(),
            Element::RectangleElement(rect) => rect.render(),
        }
    }
}
