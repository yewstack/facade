mod live;
mod widgets;

use crate::live::Requirement;
use std::collections::HashSet;
use widgets::{Widget, WidgetModel};

pub type Model = WidgetModel<LayoutWidget>;

pub struct LayoutWidget {
}

impl Default for LayoutWidget {
    fn default() -> Self {
        Self {
        }
    }
}

impl Widget for LayoutWidget {
    fn requirements(&self) -> HashSet<Requirement> {
        vec![Requirement::LayoutChange].into_iter().collect()
    }
}
