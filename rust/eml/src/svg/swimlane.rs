pub struct Swimlane {
    pub top: f64,
    pub bottom: f64,
    pub width: f64,
}

impl Swimlane {
    pub fn new() -> Self {
        Swimlane {
            top: 0f64,
            bottom: 0f64,
            width: 0f64,
        }
    }
}
