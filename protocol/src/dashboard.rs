use super::{Layout, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dashboard {
    pub title: Value,
    pub pages: Vec<Page>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Page {
    pub title: Value,
    pub subtitle: Value,
    pub body: Layout,
}
