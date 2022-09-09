// A Field is a single key: value pair (maybe Record is better?)
// A Series is a key: [values] pair

#[derive(Debug, PartialEq)]
pub struct TextField {
    pub name: String,
    pub data: String,
}

#[derive(Debug, PartialEq)]
pub struct TextSeries {
    pub name: String,
    pub data: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct IntegerField {
    pub name: String,
    pub data: i32,
}

#[derive(Debug)]
pub struct IntegerSeries {
    pub name: String,
    pub data: Vec<i32>,
}

#[derive(Debug, PartialEq)]
pub enum Field {
    Text(TextField),
    Integer(IntegerField),
}

#[derive(Debug, PartialEq)]
pub struct ExpressionId(pub String);

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    Form,
    Job,
    Command,
    Event,
    View,
    Flow,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Form(ExpressionId, Vec<Field>),
    Job(ExpressionId, Vec<Field>),
    Command(ExpressionId, Vec<Field>),
    Event(ExpressionId, Vec<Field>),
    View(ExpressionId, Vec<String>),
    Flow(ExpressionId, Vec<ExpressionId>),
}

#[derive(Debug)]
pub struct EventModel {
    pub expressions: Vec<Expression>,
}
