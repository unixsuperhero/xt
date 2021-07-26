use crate::database::*;

#[derive(Clone,Debug)]
pub struct AppState<'a> {
    cursor: Pos,
    screen_area: Pos,
    table: Table<'a>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}
