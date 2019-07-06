use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Action {
    id: Id,
    kind: Kind,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Click,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Reaction {
    Scene(Scene),
    Delta(Delta),
}

pub type OverlayId = Option<Id>;

impl Reaction {
    pub fn overlay_id(&self) -> OverlayId {
        match self {
            Reaction::Scene(_) => None,
            Reaction::Delta(delta) => Some(delta.id.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Scene {
    Spinner,
    FullScreen(Layout),
    Dashboard {
        title: Value,
        body: Layout,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Menu {
    Item(Id, Value),
    Bar(Vec<Menu>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Container {
    Tabs(Vec<Tab>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tab {
    title: Value,
    body: Layout,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Layout {
    Blank,
    Welcome,
    Widget(Widget),
    Row(Vec<Layout>),
    Column(Vec<Layout>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Widget {
    Dynamic(Id),
    Fixed(Value),
    Button(Id),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl Default for Id {
    fn default() -> Self {
        Id("<default>".into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    Nothing,
    String(String),
    Decimal(BigDecimal),
}

impl Default for Value {
    fn default() -> Self {
        Value::Nothing
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nothing => write!(f, ""),
            Value::String(value) => write!(f, "{}", value),
            Value::Decimal(value) => write!(f, "{}", value),
        }
    }
}

macro_rules! value_convert {
    (@declare $var:ident $type:ty) => {
        impl From<$type> for Value {
            fn from(value: $type) -> Self {
                Value::$var(value.into())
            }
        }
    };
    ($var:ident : $($type:ty),*) => {
        $( value_convert!(@declare $var $type); )+
    };
}

value_convert!(Decimal: u8, i8, u16, i16, u32, i32, u64, i64, BigDecimal);

value_convert!(String: &str);
