#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct TextTranslate {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub enum CardType {
    Job,
    Form,
    Command,
    Event,
    View,
}

#[derive(Clone)]
pub struct Card {
    pub id: String,
    pub card_type: CardType,
    pub origin: Point,
    pub width: f64,
    pub height: f64,
    pub text_lines: Vec<String>,
    pub text_translate: TextTranslate,
}

impl Card {
    pub fn new(id: String, card_type: CardType, text_lines: Vec<String>) -> Self {
        Card {
            id,
            card_type,
            text_lines,
            origin: Point { x: 0f64, y: 0f64 },
            width: 0f64,
            height: 0f64,
            text_translate: TextTranslate { x: 0f64, y: 0f64 },
        }
    }

    pub fn top_anchor(&self) -> Point {
        let x = self.origin.x + self.width / 2f64;
        let y = self.origin.y;
        Point { x, y }
    }

    pub fn right_anchor(&self) -> Point {
        let x = self.origin.x + self.width;
        let y = self.origin.y + self.height / 2f64;
        Point { x, y }
    }

    pub fn bottom_anchor(&self) -> Point {
        let x = self.origin.x + self.width / 2f64;
        let y = self.origin.y + self.height;
        Point { x, y }
    }

    pub fn left_anchor(&self) -> Point {
        let x = self.origin.x;
        let y = self.origin.y + self.height / 2f64;
        Point { x, y }
    }

    pub fn set_origin(&mut self, x: f64, y: f64) {
        self.origin = Point { x, y };
    }

    pub fn set_dimensions(&mut self, width: f64, height: f64, pad: f64) {
        self.height = height;
        self.width = width;
        self.text_translate.x = self.origin.x as f64 + pad as f64 / 4f64;
        self.text_translate.y = self.origin.y as f64 + pad as f64 / 6f64;
    }

    fn _render(&self, fill: &str) -> String {
        let text = if self.text_lines.len() as f64 > 0f64 {
            let lines = self
                .text_lines
                .iter()
                .map(|line| {
                    format!(
                        "<tspan x='0' dy='1rem' xml:space='preserve'>{}</tspan>",
                        line
                    )
                })
                .collect::<Vec<String>>()
                .join("");
            format!(
                "\
            <text transform='translate({translate_x} {translate_y})' x='0' y='0' xml:space='preserve' >\
            <tspan x='0' dy='1rem' xml:space='preserve'>{id}</tspan>\
            <tspan x='0' dy='1rem'>==========</tspan>\
            {lines}\
            </text>\
            ",
                id = self.id,
                translate_x = self.text_translate.x,
                translate_y = self.text_translate.y,
                lines = lines
            )
        } else {
            "".to_string()
        };
        format!("\
        <rect id='{id}' stroke='black' stroke-width='2' x='{x}' y='{y}' height='{height}' width='{width}' fill='{fill}' />\
        {text}\
        ",
        id=self.id,
        x=self.origin.x,
        y=self.origin.y,
        width=self.width,
        height=self.height,
        fill=fill,
        text=text,
        )
    }

    pub fn render(&self) -> String {
        match self.card_type {
            CardType::Job => self._render("#ffffff"),
            CardType::Form => self._render("#ffffff"),
            CardType::Command => self._render("#60b3f7"),
            CardType::Event => self._render("#f7a660"),
            CardType::View => self._render("#60f765"),
        }
    }
}
