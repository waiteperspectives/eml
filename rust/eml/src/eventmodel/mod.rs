#[derive(Debug, PartialEq)]
pub struct TextField {
    pub name: String,
    pub data: Vec<String>,
}

#[derive(Debug)]
pub struct IntegerField {
    pub name: String,
    pub data: Vec<i32>,
}

#[derive(Debug)]
pub enum Field {
    Text(TextField),
    Integer(IntegerField),
}

#[derive(Debug)]
pub struct ExpressionId(pub String);

#[derive(Debug)]
pub enum ExpressionType {
    Form,
    Job,
    Command,
    Event,
    View,
    Flow,
}

#[derive(Debug)]
pub enum Expression {
    Form(ExpressionId, Vec<Field>),
    Job(ExpressionId, Vec<Field>),
    Command(ExpressionId, Vec<Field>),
    Event(ExpressionId, Vec<Field>),
    View(ExpressionId, Vec<Field>),
    Flow(ExpressionId, Vec<ExpressionId>),
}

#[derive(Debug)]
pub struct EventModel {
    pub expressions: Vec<Expression>,
}
