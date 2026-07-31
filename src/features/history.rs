use crate::{features::selection::RectSelection, shapes::Shape};

pub enum HistoryEvent {
    Add,
    Remove(Shape),
    AddSelection,
    RemoveSelection(RectSelection),
}
