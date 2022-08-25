use super::point::Point;

pub struct Line {
    pub point1: Point,
    pub point2: Point,
    pub id: String,
}

impl Line {
    pub fn render(&self) -> String {
        format!(
            "<line id='{}' x1='{}' y1='{}' x2='{}' y2='{}' stroke='black' stroke_width=3 />",
            self.id, self.point1.x, self.point1.y, self.point2.x, self.point2.y
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_render() {
        let line = Line {
            point1: Point {
                x: 0,
                y: 0,
                id: "p1".to_string(),
            },
            point2: Point {
                x: 10,
                y: 10,
                id: "p2".to_string(),
            },
            id: "zzz".to_string(),
        };
        let expected =
            "<line id='zzz' x1='0' y1='0' x2='10' y2='10' stroke='black' stroke_width=3 />";
        let observed = line.render();
        assert_eq!(expected, observed);
    }
}
