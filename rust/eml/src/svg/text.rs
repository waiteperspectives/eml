use super::point::Point;

pub struct Text {
    pub origin: Point,
    pub text: String,
    pub id: String,
}

impl Text {
    pub fn render(&self) -> String {
        format!("<text id='{}' x='0' y='0'>{}</text>", self.id, self.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_render() {
        let txt = Text {
            id: "t1".to_string(),
            text: "testing 123".to_string(),
        };
        let expected = "<text id='t1' text='testing 123' />";
        let observed = txt.render();
        assert_eq!(expected, observed);
    }
}
