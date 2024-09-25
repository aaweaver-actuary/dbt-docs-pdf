#[derive(Debug, Clone)]
pub enum DbtIncrementalStratgey {
    Append,
    Merge,
    InsertOverwrite,
}