use bigdecimal::BigDecimal;
use failure::Fail;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use strum_macros::EnumIter;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "serialization error: {}", _0)]
    SerdeError(#[cause] serde_json::error::Error),
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error::SerdeError(err)
    }
}

pub trait Message: Serialize + for<'de> Deserialize<'de> + Sized {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        serde_json::to_vec(self).map_err(Error::from)
    }

    fn deserialize(data: &[u8]) -> Result<Self, Error> {
        serde_json::from_slice(data).map_err(Error::from)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Action {
    id: Id,
    kind: Kind,
}

impl Message for Action {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Click,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Reaction {
    Scene(Scene),
    Delta(Delta),
}

impl Message for Reaction {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Scene {
    Spinner,
    App(App),
    Container(Container),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct App {
    pub navigation_drawer: List,
    pub app_bar: Bar,
    pub content: Container,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct List {
    pub dense: bool,
    pub items: Vec<ListItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ListItem {
    pub action: Icon,
    pub content: Title,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Icon {
    Home,
    MenuSandwich,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Title {
    pub caption: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Bar {
    pub nav_icon: Icon,
    pub title: Title,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Container {
    pub fluid: bool,
    pub layout: Layout,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    pub flex_vec: Vec<Flex>,
    pub wrap: bool,
    pub fill: bool,
    pub reverse: bool,
    pub direction: Option<Direction>,
    pub align: Option<Align>,
    pub justify: Option<Justify>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Row,
    Column,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Align {
    Start,
    Center,
    End,
    SpaceAround,
    SpaceBetween,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Justify {
    Start,
    Center,
    End,
    SpaceAround,
    SpaceBetween,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Flex {
    pub breakpoints: HashMap<Breakpoint, Cols>,
    pub offsets: HashMap<Breakpoint, Cols>,
    pub components: Vec<Component>,
}

pub type FlexWidth = (Breakpoint, Cols);

#[derive(Serialize, Deserialize, EnumIter, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Breakpoint {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

#[derive(Serialize, Deserialize, EnumIter, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cols {
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    N11,
    N12,
}

// TODO: Consider to replace with trait (but has issues with derived traits)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Component {
    List,
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
pub enum Bind {
    Dynamic(Id),
    Fixed(Value),
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
