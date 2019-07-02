use bigdecimal::BigDecimal;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Reaction {
    Layout(Layout),
    Assing {
        id: Id,
        value: Value,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Layout {
    // TODO Add widgets
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Id(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    String(String),
    Decimal(BigDecimal),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dynamic {
    id: Id,
    value: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Static {
    value: Value,
}
