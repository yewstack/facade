use protocol::{Align, Direction, Justify};

pub trait ToClass {
    fn to_class(&self) -> &'static str;
}

impl ToClass for Align {
    fn to_class(&self) -> &'static str {
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
    fn to_class(&self) -> &'static str {
        match self {
            Direction::Row => "row",
            Direction::Column => "column",
        }
    }
}

impl ToClass for Justify {
    fn to_class(&self) -> &'static str {
        match self {
            Justify::Start => "justify-start",
            Justify::Center => "justify-center",
            Justify::End => "justify-end",
            Justify::SpaceAround => "justify-space-around",
            Justify::SpaceBetween => "justify-space-between",
        }
    }
}
