use crate::database::*;

#[derive(Clone,Debug)]
pub struct AppState {
    cursor: Pos,
    screen_area: Pos,
    table: Table,
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
