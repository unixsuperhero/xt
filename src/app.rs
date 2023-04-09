use crate::database::*;

#[derive(Clone,Debug)]
pub struct AppState<'a> {
    pub cursor: Pos,
    pub screen_area: Pos,
    pub table: Table<'a>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}
