use derive_more::From;
use enum_kinds::EnumKind;
use serde::{Deserialize, Serialize};

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
pub struct Rectangle {
    pub start: Point<u64>,
    pub end: Point<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sequence {
    pub start: u64,
    pub end: u64,
}
