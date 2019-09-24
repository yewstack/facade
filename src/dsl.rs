#![allow(non_snake_case)]

pub use protocol::Icon;
use std::collections::HashMap;

pub fn Scene(app: protocol::App) -> protocol::Scene {
    protocol::Scene::App(app)
}

pub fn App(
    navigation_drawer: protocol::NavigationDrawer,
    content: protocol::Container,
) -> protocol::App {
    protocol::App {
        navigation_drawer,
        app_bar: protocol::Bar {
            nav_icon: protocol::Icon::MenuSandwich,
            title: protocol::Title {
                caption: "Title".into(),
            },
        },
        content,
        footer: protocol::Footer {},
    }
}

pub fn NavigationDrawer(list: protocol::List) -> protocol::NavigationDrawer {
    protocol::NavigationDrawer { list }
}

pub fn List(items: Vec<protocol::ListItem>) -> protocol::List {
    protocol::List { dense: true, items }
}

pub fn ListItem(icon: protocol::Icon, title: String) -> protocol::ListItem {
    protocol::ListItem {
        action: icon,
        content: protocol::Title { caption: title },
    }
}

pub fn Container(row: protocol::Row) -> protocol::Container {
    protocol::Container { row, fluid: true }
}

/// Replaces `Layout` of 1.x version.
pub fn Row(cols: Vec<protocol::Col>) -> protocol::Row {
    protocol::Row {
        cols,
        wrap: false,
        fill: false,
        reverse: false,
        direction: None,
        align: None,
        justify: None,
    }
}

/// Replaces `Flex` of 1.x version.
pub fn Col(components: Vec<protocol::Component>) -> protocol::Col {
    protocol::Col {
        breakpoints: HashMap::new(),
        offsets: HashMap::new(),
        components,
    }
}
