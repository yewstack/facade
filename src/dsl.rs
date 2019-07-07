#![allow(non_snake_case)]

use protocol::{self as frame, Id, Layout, Value, Widget};

// ╔═╗┌─┐┌─┐┌┐┌┌─┐┌─┐
// ╚═╗│  ├┤ │││├┤ └─┐
// ╚═╝└─┘└─┘┘└┘└─┘└─┘

pub fn FullScreen(value: Layout) -> frame::Scene {
    frame::Scene::FullScreen(value)
}

pub fn Spinner() -> frame::Scene {
    frame::Scene::Spinner
}

pub fn Dashboard(
    title: impl Into<Value>,
    pages: impl IntoIterator<Item = frame::dashboard::Page>,
) -> frame::Scene {
    let dashboard = frame::dashboard::Dashboard {
        title: title.into(),
        pages: pages.into_iter().collect(),
    };
    frame::Scene::Dashboard(dashboard)
}

pub fn Page(
    title: impl Into<Value>,
    subtitle: impl Into<Value>,
    body: impl Into<Layout>,
) -> frame::dashboard::Page {
    frame::dashboard::Page {
        title: title.into(),
        subtitle: subtitle.into(),
        body: body.into(),
    }
}

pub fn Menu(value: impl IntoIterator<Item = frame::MenuItem>) -> frame::Menu {
    frame::Menu {
        items: value.into_iter().collect(),
    }
}

pub fn Item(value: impl Into<Value>) -> frame::MenuItem {
    frame::MenuItem {
        caption: value.into(),
    }
}

// ╔═╗┌─┐┌┐┌┌┬┐┌─┐┬┌┐┌┌─┐┬─┐┌─┐
// ║  │ ││││ │ ├─┤││││├┤ ├┬┘└─┐
// ╚═╝└─┘┘└┘ ┴ ┴ ┴┴┘└┘└─┘┴└─└─┘

// ╦  ┌─┐┬ ┬┌─┐┬ ┬┌┬┐
// ║  ├─┤└┬┘│ ││ │ │
// ╩═╝┴ ┴ ┴ └─┘└─┘ ┴

pub fn Blank() -> frame::Layout {
    frame::Layout::Blank
}

pub fn Row(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Row(layouts)
}

pub fn Column(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Column(layouts)
}

// ╦ ╦┬┌┬┐┌─┐┌─┐┌┬┐┌─┐
// ║║║│ │││ ┬├┤  │ └─┐
// ╚╩╝┴─┴┘└─┘└─┘ ┴ └─┘

pub fn Dynamic(value: impl AsRef<str>) -> Layout {
    let id = Id::from(value);
    Layout::Widget(Widget::Dynamic(id))
}

pub fn Fixed(value: impl Into<Value>) -> Layout {
    Layout::Widget(Widget::Fixed(value.into()))
}
