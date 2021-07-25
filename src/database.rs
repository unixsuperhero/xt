use {
    slab::Slab,
    std::collections::HashMap,
    crate::app::Pos,
};

#[derive(Clone, Debug)]
pub struct Database<'a> {
    cells: Slab<String>,
    tables: Slab<Table<'a>>,
    columns: Slab<Column>,
    rev_cells: HashMap<String, usize>,
    rev_columns: HashMap<Column, usize>,
    head: Option<usize>,
}

impl<'a> Database<'a> {
    pub fn new() -> Database<'static> {
        Database {
            cells: Slab::new(),
            tables: Slab::new(),
            columns: Slab::new(),
            rev_cells: HashMap::new(),
            rev_columns: HashMap::new(),
            head: None,
        }
    }

    pub fn table_from_builder(&mut self, tb: &'a TableBuilder) -> usize {
        let empty_cell = self.insert_cell(&"");
        let mut cells: Vec<usize> = vec![empty_cell; tb.row_cnt * tb.col_cnt];

        for (r,row) in tb.rows.iter().enumerate() {
            for (c,cell) in row.iter().enumerate() {
                let i = (r * tb.col_cnt) + c;
                let cell_ref = self.insert_cell(&cell[..]);
                cells[i] = cell_ref;
            }
        }

        let mut columns: Vec<&Column> = vec![];
        for col in tb.cols.iter() {
            let key = self.insert_col(col.clone());
            columns.push(col);
        };

        self.tables.insert(Table { cells, columns, row_cnt: tb.row_cnt, col_cnt: tb.col_cnt })
    }

    pub fn insert_col(&mut self, col: Column) -> usize {
        match self.rev_col_lookup(&col) {
            Some(key) => *key,
            None => {
                let key = self.columns.insert(col.clone());
                self.rev_columns.insert(col, key);
                key
            }
        }
    }

    pub fn rev_col_lookup(&self, col: &Column) -> Option<&usize> {
        self.rev_columns.get(col)
    }

    pub fn insert_cell(&mut self, contents: &str) -> usize {
        match self.rev_cell_lookup(contents) {
            Some(key) => *key,
            None => {
                let key = self.cells.insert(contents.to_string());
                self.rev_cells.insert(contents.to_string(), key);
                key
            }
        }
    }

    pub fn rev_cell_lookup(&self, contents: &str) -> Option<&usize> {
        self.rev_cells.get(contents)
    }

    pub fn current_table(&self) -> Option<&Table> {
        match self.head {
            Some(key) => self.tables.get(key),
            None => None,
        }
    }

    fn dbl_vec_area(dblvec: &[Vec<String>]) -> Pos {
        let mut rows = 0;
        let mut cols = 0;

        for row in dblvec.iter() {
            rows += 1;

            let mut cur_col = 0;
            for _col in row.iter() {
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
pub struct TableBuilder {
    rows: Vec<Vec<String>>,
    cols: Vec<Column>,
    row_cnt: usize,
    col_cnt: usize,
}

impl TableBuilder {
    pub fn new() -> Self {
        let rows = Vec::new();
        let cols = Vec::new();
        let row_cnt = 0;
        let col_cnt = 0;

        Self { rows, row_cnt, col_cnt, cols }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        for (i, cell) in row.iter().enumerate() {
            if self.col_cnt <= i {
                self.col_cnt = i + 1;
                self.cols.push(Column::new());
            }

            if cell.len() > self.cols[i].width {
                self.cols[i].width = cell.len();
            }
        }

        self.rows.push(row);
        self.row_cnt += 1;
    }
}

#[cfg(test)]
mod table_builder_tests {
    use super::*;

    #[test]
    fn test_tb_new() {
        let tb = TableBuilder::new();
        let empty_string_vec: Vec<Vec<String>> = Vec::new();
        let empty_col_vec: Vec<Column> = Vec::new();

        assert_eq!(tb.rows.len(), empty_string_vec.len());
        assert_eq!(tb.cols.len(), empty_col_vec.len());
        assert_eq!(tb.row_cnt, 0);
        assert_eq!(tb.col_cnt, 0);
    }

    #[test]
    fn test_tb_add_row() {
        let mut tb = TableBuilder::new();
        tb.add_row(vec![String::from("Hello...eto..."), String::from("Worudo, desho")]);

        assert_eq!(&tb.row_cnt, &1);
        assert_eq!(&tb.col_cnt, &2);

        tb.add_row(vec![String::from("a"), String::from("bb"), String::from("ccc")]);

        assert_eq!(&tb.row_cnt, &2);
        assert_eq!(&tb.col_cnt, &3);

        assert_eq!(&tb.cols[0].width, &"Hello...eto...".len());
        assert_eq!(&tb.cols[1].width, &"Worudo, desho".len());
        assert_eq!(&tb.cols[2].width, &"ccc".len());
    }
}

#[derive(Clone, Debug)]
pub struct Table<'a> {
    cells: Vec<usize>,
    columns: Vec<&'a Column>,
    row_cnt: usize,
    col_cnt: usize,
}

#[derive(Clone, Debug, Hash, Eq)]
pub struct Column {
    header: String,
    width: usize,
}

impl PartialEq for Column {
    fn eq(&self, other: &Column) -> bool {
        self.header == other.header && self.width == other.width
    }
}

impl Column {
    pub fn new() -> Self {
        Self {
            header: String::from(""),
            width: 0,
        }
    }

    pub fn from_widths(widths: Vec<usize>) -> Vec<Column> {
        widths.iter().map(|wid| Self { header: String::from(""), width: *wid } ).collect()
    }
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
    fn test_database_insert_col() {
        panic!("implement me");
    }

    #[test]
    fn test_database_insert_col_no_dupes() {
        panic!("implement me");
    }

    #[test]
    fn test_database_rev_col_lookup() {
        panic!("implement me");
    }

    #[test]
    fn test_database_table_from_builder() {
        panic!("implement me");
    }
}
