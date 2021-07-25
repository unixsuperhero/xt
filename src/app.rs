use crate::database::*;

#[derive(Clone,Debug)]
pub struct AppState<'a> {
    cursor: Pos,
    screen_area: Pos,
    table: Table<'a>,
    mode: Mode,
}

// NOTE: Mode(s) probably shouldn't be enums, since they need to contain fairly dynamic data
#[derive(Clone,Copy,Debug)]
enum Mode {
    Normal,
    VisualFile,
    VisualRow(Pos, Pos),
    VisualCol(Pos, Pos),
    VisualBlk(Pos, Pos),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}
