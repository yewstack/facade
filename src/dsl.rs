#![allow(non_snake_case)]

use protocol::{Layout, Widget};

pub fn Row(value: impl IntoIterator<Item=Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Row(layouts)
}
