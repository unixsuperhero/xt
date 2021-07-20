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
