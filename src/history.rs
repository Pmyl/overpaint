use crate::shapes::Shape;

pub enum HistoryEvent {
    Add,
    Remove(Shape),
}
