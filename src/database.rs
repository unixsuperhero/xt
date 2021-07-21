use {slab::Slab, std::collections::HashMap};

#[derive(Clone, Debug)]
pub struct Database {
    cells: Slab<String>,
    rev_cells: HashMap<String, usize>,
    tables: Slab<Table>,
    columns: Slab<Column>,
    head: Option<usize>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            cells: Slab::new(),
            rev_cells: HashMap::new(),
            tables: Slab::new(),
            columns: Slab::new(),
            head: None,
        }
    }

    pub fn insert_cell(&mut self, contents: &String) -> usize {
        match self.rev_cells.get(contents) {
            Some(key) => *key,
            None => {
                let key = self.cells.insert(contents.to_string());
                self.rev_cells.insert(contents.to_string(), key);
                key
            }
        }
    }

    pub fn table_from_dbl_vec(&mut self, grid: Vec<Vec<String>>) {
        let area = Database::dbl_vec_area(&grid);

        let mut table_cells = Vec::new();

        for row in grid.iter() {
            let mut col_len = 0;
            for col in row.iter() {
                let cell = self.insert_cell(col);
                table_cells.push(cell);
                col_len += 1;
            }

            while col_len < area.col {
                let cell = self.insert_cell(&String::from(""));
                table_cells.push(cell);
                col_len += 1;
            }
        }

        let tbl = Table::from_area_and_cells(area, table_cells);
        self.head = Some(self.tables.insert(tbl));
    }

    fn dbl_vec_area(dblvec: &Vec<Vec<String>>) -> Pos {
        let mut rows = 0;
        let mut cols = 0;

        for row in dblvec.iter() {
            rows += 1;

            let mut cur_col = 0;
            for col in row.iter() {
                cur_col += 1;
                if cur_col > cols {
                    cols += 1;
                }
            }
        }

        Pos {
            row: rows,
            col: cols,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MetaColumn {
    width: usize,
    has_data: bool,
}

#[derive(Clone, Debug)]
pub struct Table {
    cells: Vec<usize>,
    columns: Vec<Column>,
    area: Pos,
}

impl Table {
    pub fn from_area_and_cells(area: Pos, cells: Vec<usize>) -> Self {
        Self {
            cells,
            columns: vec![Column::new(); area.col],
            area,
        }
    }

    pub fn from_dbl_vec(grid: Vec<Vec<String>>) -> Database {
        //let area = Self::dbl_vec_area(grid);
        //let mut table_cells: Vec<usize> = Vec::with_capacity(area.row * area.col);

        //for cur_row in grid.iter() {
        //    let mut cols = 0;
        //    for cur_cell in cur_row.iter() {
        //        cols += 1;
        //        let text = grid[row_idx][col_idx];
        //        table_cells.push(cells.insert(grid[row_idx].entry(col_idx)
        //    }
        //}
        unimplemented!();
        //let mut cells = Slab::new();
        //let mut tables = Slab::new();
        //let mut columns = Slab::new();
        //let mut table = Table::new();
        //let mut metacols = Vec::new();

        //let mut area = Pos { row: 0, col: 0 };
        //
        //for (row_idx, row_vec) in grid.iter().enumerate() {
        //    area = Pos { row: row_idx, ..area };

        //    for (col_idx, col_val) in row_vec.iter().enumerate() {
        //        // let cell_ref = cells.insert(col_val.clone());
        //        // let cur_cell = cells.get(cell_ref).unwrap();

        //        if area.col <= col_idx {
        //            area = Pos { col: col_idx + 1, ..area };
        //        };

        //        // // originally i used columns.len() <= col_idx,
        //        // // but i don't think it needs to be that wide of a range
        //        // if columns.len() == col_idx {
        //        //     metacols.push(MetaColumn { width: 0, has_data: false });
        //        // }

        //        // if ! metacols[col_idx].has_data && cur_cell.trim().len() != 0 {
        //        //     metacols[col_idx].has_data = true;
        //        // }

        //        // if cur_cell.len() > metacols[col_idx].width {
        //        //     metacols[col_idx].width = cur_cell.len(); // update the recorded column width
        //        // }
        //    }
        //}

        //let head = tables.insert(Table::new());

        //Database {
        //    cells,
        //    tables,
        //    columns,
        //    head,
        //}
    }
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pos {
    row: usize,
    col: usize,
}

#[cfg(test)]
mod test {
    use crate::database::*;

    #[test]
    fn test_database_dbl_vec_area() {
        let dblvec = vec![
            vec!["a".to_string()],
            vec!["b".to_string(), "c".to_string()],
        ];
        let mut db = Database::new();
        db.table_from_dbl_vec(dblvec);

        let tbl = db.tables.get(db.head.unwrap()).unwrap();
        assert_eq!(tbl.area, Pos { row: 2, col: 2 });
        assert_eq!(tbl.cells.len(), 4);

        assert_eq!(db.cells.len(), 4);
        assert_eq!(db.rev_cells.len(), 4);

        let dblvec = vec![
            vec!["a".to_string()],
            vec![
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
            ],
            vec!["f".to_string()],
        ];
        let mut db = Database::new();
        db.table_from_dbl_vec(dblvec);

        let tbl = db.tables.get(db.head.unwrap()).unwrap();
        assert_eq!(tbl.area, Pos { row: 3, col: 4 });
        assert_eq!(tbl.cells.len(), 12);

        assert_eq!(db.rev_cells.len(), 7);
    }
}
