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
pub enum Field {
    Text(TextField),
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
pub enum Body {
    FieldBody(Vec<Field>),
    TableBody(Vec<String>),
    UseBody(ExpressionId),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Form(ExpressionId, Body),
    Job(ExpressionId, Body),
    Command(ExpressionId, Body),
    Event(ExpressionId, Body),
    View(ExpressionId, Body),
    Flow(ExpressionId, Vec<ExpressionId>),
}

#[derive(Debug)]
pub struct EventModel {
    pub expressions: Vec<Expression>,
}
