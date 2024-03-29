pub use {crate::app::Pos, slab::Slab, std::collections::HashMap};

#[derive(Clone, Debug)]
pub struct Database<'a> {
    cells: Slab<String>,
    tables: Slab<Table<'a>>,
    columns: Slab<Column>,
    head: Option<usize>,
}

impl<'a> Database<'a> {
    pub fn new() -> Database<'static> {
        Database {
            cells: Slab::new(),
            tables: Slab::new(),
            columns: Slab::new(),
            head: None,
        }
    }

    pub fn load_table(&mut self, tb: &'a TableBuilder) -> usize {
        let empty_key = self.insert_cell(&String::from(""));

        let mut cells = vec![empty_key; tb.row_cnt * tb.col_cnt];

        for (r, row) in tb.rows.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                cells[(r * tb.col_cnt) + c] = self.insert_cell(&val.to_string());
            }
        }

        let mut columns: Vec<&Column> = vec![];
        for col in tb.cols.iter() {
            columns.push(col);
        }

        self.tables.insert(Table {
            cells,
            columns,
            row_cnt: tb.row_cnt,
            col_cnt: tb.col_cnt,
        })
    }

    pub fn insert_cell(&mut self, contents: &str) -> usize {
        self.cells.insert(contents.to_string())
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
pub struct TableBuilder {
    cells: Slab<String>,
    rows: Vec<Vec<usize>>,
    cols: Vec<Column>,
    pub row_cnt: usize,
    pub col_cnt: usize,
}

impl TableBuilder {
    pub fn new() -> Self {
        let cells = Slab::new();
        let rows = Vec::new();
        let cols = Vec::new();
        let row_cnt = 0;
        let col_cnt = 0;

        Self {
            cells,
            rows,
            row_cnt,
            col_cnt,
            cols,
        }
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        let mut new_row: Vec<usize> = vec![];
        for (i, val) in row.into_iter().enumerate() {
            if self.col_cnt <= i {
                self.col_cnt = i + 1;
                self.cols.push(Column::new());
            }

            if val.len() > self.cols[i].width {
                self.cols[i].width = val.len();
            }

            let key = self.cells.insert(val.clone());

            new_row.push(key);
        }

        self.rows.push(new_row);
        self.row_cnt += 1;
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
    pub header: String,
    pub width: usize,
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

    pub fn header(&mut self, new_header: String) {
        self.header = new_header;
    }

    pub fn from_widths(widths: Vec<usize>) -> Vec<Column> {
        widths
            .iter()
            .map(|wid| Self {
                header: String::from(""),
                width: *wid,
            })
            .collect()
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
    fn test_database_insert_col() {
        let mut db = Database::new();

        let key = db.insert_col(Column {
            header: String::from("FNAME"),
            width: 10,
        });
        assert_eq!(key, 0);

        let key = db.insert_col(Column {
            header: String::from("LNAME"),
            width: 10,
        });
        assert_eq!(key, 1);

        // no dupes
        let key = db.insert_col(Column {
            header: String::from("FNAME"),
            width: 10,
        });
        assert_eq!(key, 0);
    }

    #[test]
    fn test_database_load_table() {
        let mut db = Database::new();

        let mut tb = TableBuilder::new();
        tb.add_row(vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ]);
        tb.add_row(vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ]);
        tb.add_row(vec![
            String::from("onejjcjcjcj c jc"),
            String::from(""),
            String::from("thrsdfkjlsdjee"),
        ]);
        tb.add_row(vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ]);

        let tab = db.load_table(&tb);
        assert_eq!(db.tables.len(), 1);
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
        tb.add_row(vec![
            String::from("Hello...eto..."),
            String::from("Worudo, desho"),
        ]);

        assert_eq!(&tb.row_cnt, &1);
        assert_eq!(&tb.col_cnt, &2);

        tb.add_row(vec![
            String::from("a"),
            String::from("bb"),
            String::from("ccc"),
        ]);

        assert_eq!(&tb.row_cnt, &2);
        assert_eq!(&tb.col_cnt, &3);

        assert_eq!(&tb.cols[0].width, &"Hello...eto...".len());
        assert_eq!(&tb.cols[1].width, &"Worudo, desho".len());
        assert_eq!(&tb.cols[2].width, &"ccc".len());
    }
}

#[cfg(test)]
mod column_tests {
    use super::*;

    #[test]
    fn test_column() {
        let mut col = Column {
            header: String::from("FIRSTCOL"),
            width: 10,
        };

        assert_eq!(&col.header, &String::from("FIRSTCOL"));

        col.header(String::from("ANOTHER HEADER"));

        assert_eq!(&col.header, &String::from("ANOTHER HEADER"));
    }

    #[test]
    fn test_tb_add_row() {
        let mut tb = TableBuilder::new();
        tb.add_row(vec![
            String::from("Hello...eto..."),
            String::from("Worudo, desho"),
        ]);

        assert_eq!(&tb.row_cnt, &1);
        assert_eq!(&tb.col_cnt, &2);

        tb.add_row(vec![
            String::from("a"),
            String::from("bb"),
            String::from("ccc"),
        ]);

        assert_eq!(&tb.row_cnt, &2);
        assert_eq!(&tb.col_cnt, &3);

        assert_eq!(&tb.cols[0].width, &"Hello...eto...".len());
        assert_eq!(&tb.cols[1].width, &"Worudo, desho".len());
        assert_eq!(&tb.cols[2].width, &"ccc".len());
    }
}
