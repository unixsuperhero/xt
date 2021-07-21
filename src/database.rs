use slab::Slab;

#[derive(Clone,Debug)]
pub struct Database {
    cells: Slab<String>,
    tables: Slab<Table>,
    columns: Slab<Column>,
    head: usize,
}

impl Database {
    pub fn from_dbl_vec(grid: Vec<Vec<String>>) -> Database {
        let mut cells = Slab::new();
        let mut tables = Slab::new();
        let mut columns = Slab::new();
        let mut table = Table::new();
        let mut metacols = Vec::new();

        let mut area = Pos { row: 0, col: 0 };
        for (row_idx, row_vec) in grid.iter().enumerate() {
            area = Pos { row: row_idx, ..area };

            for (col_idx, col_val) in row_vec.iter().enumerate() {
                let cell_ref = cells.insert(col_val.clone());
                let cur_cell = cells.get(cell_ref).unwrap();

                if area.col <= col_idx {
                    area = Pos { col: col_idx + 1, ..area };
                };

                // originally i used columns.len() <= col_idx,
                // but i don't think it needs to be that wide of a range
                if columns.len() == col_idx {
                    metacols.push(MetaColumn { width: 0, has_data: false });
                }

                if ! metacols[col_idx].has_data && cur_cell.trim().len() != 0 {
                    metacols[col_idx].has_data = true;
                }

                if cur_cell.len() > metacols[col_idx].width {
                    metacols[col_idx].width = cur_cell.len(); // update the recorded column width
                }
            }
        }

        let head = tables.insert(Table::new());

        Database {
            cells,
            tables,
            columns,
            head,
        }
    }
}

#[derive(Clone,Debug)]
pub struct MetaColumn {
    width: usize,
    has_data: bool,
}

#[derive(Clone,Debug)]
pub struct Table {
    cells: Vec<usize>,
    columns: Vec<Column>,
    area: Pos,
}

impl Table {
    pub fn new() -> Self {
        Self {
            cells: vec![],
            columns: vec![],
            area: Pos{row: 0, col: 0},
        }
    }
}

#[derive(Clone,Debug)]
pub struct Column {
    header: String,
    width: usize,
}

impl Column {
    pub fn new() -> Self {
        Self {
            header: String::from(""),
            width: 0,
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Pos {
    row: usize,
    col: usize,
}


#[cfg(test)]

mod test {
    use crate::database::*;

    #[test]
    fn test_database_from_dbl_vec() {
        let dblvec = vec![vec!["a".to_string()],vec!["b".to_string(),"c".to_string()]];
        let db = Database::from_dbl_vec(dblvec);

        assert_eq!(db.cells.len(), 3);
    }
}

