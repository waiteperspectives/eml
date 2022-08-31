use std::collections::BTreeMap;

#[derive(Debug)]
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
pub struct ExpressionId(String);

#[derive(Debug)]
pub enum Expression {
    Form(Vec<Field>),
    Job(Vec<Field>),
    Command(Vec<Field>),
    Event(Vec<Field>),
    View(Vec<Field>),
    Flow(Vec<ExpressionId>),
}

#[derive(Debug)]
pub struct EventModel {
    pub expressions: BTreeMap<ExpressionId, Expression>,
}
