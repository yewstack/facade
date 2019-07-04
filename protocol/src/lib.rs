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
    Delta(Delta),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Layout {
    Welcome,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Delta {
    pub id: Id,
    pub value: Value,
}

impl From<(Id, Value)> for Delta {
    fn from((id, value): (Id, Value)) -> Self {
        Self { id, value }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(String);

impl<T: AsRef<str>> From<T> for Id {
    fn from(value: T) -> Self {
        Id(value.as_ref().to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    String(String),
    Decimal(BigDecimal),
}

macro_rules! decimal_from {
    (@declare $type:ty) => {
        impl From<$type> for Value {
            fn from(value: $type) -> Self {
                Value::Decimal(value.into())
            }
        }
    };
    ($($type:ty),*) => {
        $( decimal_from!(@declare $type); )+
    };
}

decimal_from!(u8, i8, u16, i16, u32, i32, u64, i64);
