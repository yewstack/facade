use bigdecimal::BigDecimal;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    id: Id,
    kind: Kind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Kind {
    Click,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Reaction {
    Layout(Layout),
    Assign {
        id: Id,
        value: Value,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Layout {
    Blank,
    Widget(Widget),
    Row(Vec<Layout>),
    Column(Vec<Layout>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Widget {
    Dynamic(Id),
    Static(Value),
    Action(Id),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    String(String),
    Decimal(BigDecimal),
}
