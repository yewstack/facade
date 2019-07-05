use protocol::{Layout, Widget};

fn Row(value: impl IntoIterator<Item=Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Row(layouts)
}
