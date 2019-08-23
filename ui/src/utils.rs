use lazy_static::lazy_static;
use protocol::{Align, Direction, FlexWidth, Justify};
use std::collections::HashMap;

pub trait ToClass {
    fn to_class(&self) -> &str;
}

impl ToClass for Align {
    fn to_class(&self) -> &str {
        match self {
            Align::Start => "align-start",
            Align::Center => "align-center",
            Align::End => "align-end",
            Align::SpaceAround => "align-space-around",
            Align::SpaceBetween => "align-space-between",
        }
    }
}

impl ToClass for Direction {
    fn to_class(&self) -> &str {
        match self {
            Direction::Row => "row",
            Direction::Column => "column",
        }
    }
}

impl ToClass for Justify {
    fn to_class(&self) -> &str {
        match self {
            Justify::Start => "justify-start",
            Justify::Center => "justify-center",
            Justify::End => "justify-end",
            Justify::SpaceAround => "justify-space-around",
            Justify::SpaceBetween => "justify-space-between",
        }
    }
}

lazy_static! {
    // TODO: Use phf crate here?
    static ref FLEX_WIDTH: HashMap<FlexWidth, String> = {
        use protocol::Breakpoint::*;
        use protocol::Cols::*;
        let mut map = HashMap::new();
        let brks = [
            (XSmall, "xs"),
            (Small, "sm"),
            (Medium, "md"),
            (Large, "la"),
            (XLarge, "xl"),
        ];
        let cols = [
            (N1, "1"),
            (N2, "2"),
            (N3, "3"),
            (N4, "4"),
            (N5, "5"),
            (N6, "6"),
            (N7, "7"),
            (N8, "8"),
            (N9, "9"),
            (N10, "10"),
            (N11, "11"),
            (N12, "12"),
        ];
        for (brk, brk_cls) in &brks {
            for (col, col_cls) in &cols {
                map.insert((brk.to_owned(), col.to_owned()), format!("{}-{}", brk_cls, col_cls));
            }
        }
        map
    };
}

impl ToClass for FlexWidth {
    fn to_class(&self) -> &str {
        FLEX_WIDTH.get(self).unwrap()
    }
}
