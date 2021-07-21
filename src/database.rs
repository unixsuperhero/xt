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
        match self.rev_cell_lookup(contents) {
            Some(key) => *key,
            None => {
                let key = self.cells.insert(contents.to_string());
                self.rev_cells.insert(contents.to_string(), key);
                key
            }
        }
    }

    pub fn rev_cell_lookup(&self, contents: &String) -> Option<&usize> {
        self.rev_cells.get(contents)
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
    use super::*;

    #[test]
    fn test_new() {
        Database::new();
    }

    #[test]
    fn test_database_insert_cell() {
        let mut db = Database::new();
        let cell1 = db.insert_cell(&String::from("a"));

        assert_eq!(db.cells.len(), 1);
    }

    #[test]
    fn test_database_insert_cell_no_dupes() {
        let mut db = Database::new();
        let cell1 = db.insert_cell(&String::from("a"));
        let cell2 = db.insert_cell(&String::from("b"));
        let cell3 = db.insert_cell(&String::from("c"));
        let cell4 = db.insert_cell(&String::from("b"));

        assert_ne!(cell1, cell2);
        assert_ne!(cell1, cell3);
        assert_eq!(cell2, cell4); // no dupes
        assert_eq!(db.cells.len(), 3);
        assert_eq!(db.cells.len(), 3);
    }

    #[test]
    fn test_database_rev_cell_lookup() {
        let mut db = Database::new();
        let cell1 = db.insert_cell(&String::from("a"));
        let cell2 = db.insert_cell(&String::from("b"));

        let result = db.rev_cell_lookup(&String::from("a"));
        assert_eq!(&cell1, result.unwrap());

        let result = db.rev_cell_lookup(&String::from("b"));
        assert_eq!(&cell2, result.unwrap());
    }

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
