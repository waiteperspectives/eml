use super::element::Element;

pub struct Document {
    pub children: Vec<Element>,
    pub width: usize,
    pub height: usize,
    pub id: String,
}

impl Document {
    pub fn render(&self) -> String {
        format!(
            "<svg version='1.1' width='{}' height='{}' xmlns=\'http://www.w3.org/2000/svg\'>{}</svg>",
            self.width,
            self.height,
            self.children.iter().map(|el| el.render()).collect::<Vec<String>>().join("")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svg::point::Point;
    use crate::svg::rectangle::Rectangle;
    use crate::svg::text::Text;

    fn test_render_00_min() {
        let doc = Document {
            children: vec![],
            width: 1000,
            height: 1000,
            id: "doc1".to_string(),
        };
        let expected = "<svg version='1.1' width='1000', height='1000' xmlns='http://www.w3.org/2000/svg'></svg>";
        let observed = doc.render();
        assert_eq!(expected, observed);
    }

    fn test_render_01_single_child() {
        let doc = Document {
            children: vec![Element::TextElement(Text {
                id: "t1".to_string(),
                text: "testing".to_string(),
            })],
            width: 1000,
            height: 1000,
            id: "doc1".to_string(),
        };
        let expected = concat!(
            "<svg version='1.1' width='1000', height='1000' xmlns='http://www.w3.org/2000/svg'>",
            "<text id='t1' text='testing 123' />",
            "</svg>"
        );
        let observed = doc.render();
        assert_eq!(expected, observed);
    }

    fn test_render_02_nested_child() {
        let doc = Document {
            children: vec![Element::RectangleElement(Rectangle {
                id: "r1".to_string(),
                origin: Point {
                    id: "p1".to_string(),
                    x: 0,
                    y: 0,
                },
                width: 100,
                height: 100,
                fill_color: "red".to_string(),
                children: vec![Element::TextElement(Text {
                    id: "t1".to_string(),
                    text: "testing".to_string(),
                })],
            })],
            width: 1000,
            height: 1000,
            id: "doc1".to_string(),
        };
        let expected = concat!(
            "<svg version='1.1' width='1000', height='1000' xmlns='http://www.w3.org/2000/svg'>",
            "<rectangle id='r1' x='0' y='0' width='100' height='100' fill='black' stroke='black' stroke_width='1'>",
            "<text id='t1' text='testing' />",
            "</rectangle>",
            "</svg>"
        );
        let observed = doc.render();
        assert_eq!(expected, observed);
    }
}
