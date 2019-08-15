#![allow(non_snake_case)]

use std::collections::HashMap;

pub fn Scene(app: protocol::App) -> protocol::Scene {
    protocol::Scene::App(app)
}

pub fn App(list: protocol::List, content: protocol::Container) -> protocol::App {
    protocol::App {
        navigation_drawer: list,
        app_bar: protocol::Bar {
            nav_icon: protocol::Icon::MenuSandwich,
            title: protocol::Title {
                caption: "Title".into(),
            }
        },
        content,
    }
}

pub fn List(items: Vec<protocol::ListItem>) -> protocol::List {
    protocol::List {
        items,
    }
}

pub fn Container(layout: protocol::Layout) -> protocol::Container {
    protocol::Container {
        layout,
        fluid: false,
    }
}

pub fn Layout(flex_vec: Vec<protocol::Flex>) -> protocol::Layout {
    protocol::Layout {
        flex_vec,
        wrap: false,
        fill: false,
        reverse: false,
        direction: None,
        align: None,
        justify: None,
    }
}

pub fn Flex() -> protocol::Flex {
    protocol::Flex {
        breakpoints: HashMap::new(),
        offsets: HashMap::new(),
        components: Vec::new(),
    }
}

/*
use protocol::{self, Id, Layout, Value};

// ╔═╗┌─┐┌─┐┌┐┌┌─┐┌─┐
// ╚═╗│  ├┤ │││├┤ └─┐
// ╚═╝└─┘└─┘┘└┘└─┘└─┘

pub fn FullScreen(value: impl Into<Layout>) -> protocol::Scene {
    protocol::Scene::FullScreen(value.into())
}

pub fn Spinner() -> protocol::Scene {
    protocol::Scene::Spinner
}

pub fn Dashboard(
    title: impl Into<Value>,
    pages: impl IntoIterator<Item = protocol::dashboard::Page>,
) -> protocol::Scene {
    let dashboard = protocol::dashboard::Dashboard {
        title: title.into(),
        pages: pages.into_iter().collect(),
    };
    protocol::Scene::Dashboard(dashboard)
}

pub fn Page(
    title: impl Into<Value>,
    subtitle: impl Into<Value>,
    body: impl Into<Layout>,
) -> protocol::dashboard::Page {
    protocol::dashboard::Page {
        title: title.into(),
        subtitle: subtitle.into(),
        body: body.into(),
    }
}

pub fn Menu(value: impl IntoIterator<Item = protocol::MenuItem>) -> protocol::Menu {
    protocol::Menu {
        items: value.into_iter().collect(),
    }
}

pub fn Item(value: impl Into<Value>) -> protocol::MenuItem {
    protocol::MenuItem {
        caption: value.into(),
    }
}

// ╔═╗┌─┐┌┐┌┌┬┐┌─┐┬┌┐┌┌─┐┬─┐┌─┐
// ║  │ ││││ │ ├─┤││││├┤ ├┬┘└─┐
// ╚═╝└─┘┘└┘ ┴ ┴ ┴┴┘└┘└─┘┴└─└─┘

pub fn Panel(value: impl Into<Layout>) -> Layout {
    let panel = protocol::Panel {
        title: None,
        body: value.into(),
    };
    let container = protocol::Container::Panel(panel);
    Layout::Container(Box::new(container))
}

pub fn TitledPanel(title: impl Into<Value>, value: impl Into<Layout>) -> Layout {
    let panel = protocol::Panel {
        title: Some(title.into()),
        body: value.into(),
    };
    let container = protocol::Container::Panel(panel);
    Layout::Container(Box::new(container))
}

// ╦  ┌─┐┬ ┬┌─┐┬ ┬┌┬┐
// ║  ├─┤└┬┘│ ││ │ │
// ╩═╝┴ ┴ ┴ └─┘└─┘ ┴

pub fn Blank() -> protocol::Layout {
    protocol::Layout::Blank
}

pub fn Row(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Row(layouts)
}

pub fn Column(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Column(layouts)
}

pub fn List(value: impl IntoIterator<Item = protocol::ListItem>) -> Layout {
    let items = value.into_iter().collect();
    let list = protocol::List { items };
    Layout::List(list)
}

pub fn ListItem(
    title: impl Into<Value>,
    description: impl Into<Value>,
    bind: impl Into<protocol::Bind>,
) -> protocol::ListItem {
    protocol::ListItem {
        title: title.into(),
        description: description.into(),
        bind: bind.into(),
    }
}

// ╦ ╦┬┌┬┐┌─┐┌─┐┌┬┐┌─┐
// ║║║│ │││ ┬├┤  │ └─┐
// ╚╩╝┴─┴┘└─┘└─┘ ┴ └─┘

pub fn Dynamic(value: impl Into<Id>) -> protocol::Bind {
    protocol::Bind::Dynamic(value.into())
}

pub fn Fixed(value: impl Into<Value>) -> protocol::Bind {
    protocol::Bind::Fixed(value.into())
}

// MACROS

pub mod macros {
    #[macro_export]
    macro_rules! many {
        [ $( $x:expr ),* ] => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x.into());
                )*
                temp_vec
            }
        };
    }
}
pub use super::many;
*/
