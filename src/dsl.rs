#![allow(non_snake_case)]

use protocol::{Id, Layout, Value, Widget};

// ╦  ╔═╗╦ ╦╔═╗╦ ╦╔╦╗
// ║  ╠═╣╚╦╝║ ║║ ║ ║
// ╩═╝╩ ╩ ╩ ╚═╝╚═╝ ╩

pub fn Row(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Row(layouts)
}

pub fn Column(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Column(layouts)
}

// ╦ ╦╦╔╦╗╔═╗╔═╗╔╦╗╔═╗
// ║║║║ ║║║ ╦║╣  ║ ╚═╗
// ╚╩╝╩═╩╝╚═╝╚═╝ ╩ ╚═╝

pub fn Dynamic(value: impl AsRef<str>) -> Layout {
    let id = Id::from(value);
    Layout::Widget(Widget::Dynamic(id))
}

pub fn Fixed(value: impl Into<Value>) -> Layout {
    Layout::Widget(Widget::Fixed(value.into()))
}
