use derive_more::From;
use enum_kinds::EnumKind;
use serde::{Deserialize, Serialize};
use std::ops::Sub;

#[derive(
    Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, EnumKind,
)]
#[enum_kind(LabelKind)]
pub enum Label {
    Rectangle(Rectangle),
    Sequence(Sequence),
}

impl LabelKind {
    pub const fn name(&self) -> &'static str {
        match self {
            LabelKind::Rectangle => "rectangle",
            LabelKind::Sequence => "sequence",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rectangle<T = u64> {
    pub start: Point<T>,
    pub end: Point<T>,
}

impl<T: Sub<Output = T> + Copy> Rectangle<T> {
    pub fn width(&self) -> T {
        self.end.x - self.start.x
    }

    pub fn height(&self) -> T {
        self.end.y - self.start.y
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sequence<T = u64> {
    pub start: T,
    pub end: T,
}
