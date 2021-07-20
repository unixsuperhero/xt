
mod app {
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
}

mod database {
    #[derive(Clone,Debug)]
    pub struct Table {
        cells: Vec<usize>,
        columns: Vec<Column>,
        area: Pos,
    }

    #[derive(Clone,Debug)]
    pub struct Column {
        header: String,
        width: usize,
    }

    #[derive(Clone,Copy,Debug)]
    pub struct Pos {
        row: usize,
        col: usize,
    }
}

fn main() {
    println!("Hello, world!");
}
