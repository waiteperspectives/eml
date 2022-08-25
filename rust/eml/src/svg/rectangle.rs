use super::element::Element;
use super::point::Point;

pub struct Rectangle {
    pub origin: Point,
    pub width: usize,
    pub height: usize,
    pub fill_color: String,
    pub id: String,
    pub children: Vec<Element>,
}

impl Rectangle {
    pub fn render(&self) -> String {
        format!(
            "<rect id='{}' x='{}' y='{}' width='{}' height='{}' fill='{}' stroke='black' stroke_width='1'>{}</rect>",
            self.id,
            self.origin.x.to_string(),
            self.origin.y.to_string(),
            self.width.to_string(),
            self.height.to_string(),
            self.fill_color,
            self.children.iter().map(|el| el.render()).collect::<Vec<String>>().join("")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_render_00_min() {
        let rect = Rectangle {
            id: "r1".to_string(),
            origin: Point {
                id: "p1".to_string(),
                x: 0,
                y: 0,
            },
            width: 100,
            height: 100,
            fill_color: "red".to_string(),
            children: vec![],
        };
        let expected= "<rectangle id='r1' x='0' y='0' width='100' height='100' fill='black' stroke='black' stroke_width='1'></rectangle>";
        let observed = rect.render();
        assert_eq!(expected, observed);
    }
}
