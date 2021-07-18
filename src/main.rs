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

#[derive(Clone,Copy,Debug)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Clone,Debug)]
struct Table {
    cells: Vec<usize>,
    columns: Vec<Column>,
    area: Pos,
}

#[derive(Clone,Debug)]
struct Column {
    header: String,
    width: usize,
}

fn main() {
    println!("Hello, world!");
}
