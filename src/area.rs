use {
    crate::app::Pos,
    std::ops::RangeInclusive,
};

pub struct Area {
    lhs: Pos,
    rhs: Pos,
}

impl Area {
    pub fn row_rng(&self) -> RangeInclusive<usize> {
        self.lhs.row.min(self.rhs.row)..=self.lhs.row.max(self.rhs.row)
    }

    pub fn col_rng(&self) -> RangeInclusive<usize> {
        self.lhs.col.min(self.rhs.col)..=self.lhs.col.max(self.rhs.col)
    }

    pub fn row_cnt(&self) -> usize {
        self.lhs.row.max(self.rhs.row) - self.lhs.row.min(self.rhs.row)
    }

    pub fn col_cnt(&self) -> usize {
        self.lhs.col.max(self.rhs.col) - self.lhs.col.min(self.rhs.col)
    }

    pub fn size(&self) -> usize {
        self.row_cnt() * self.col_cnt()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_area() {
        let area = Area {
            lhs: Pos{ row: 1, col: 10 },
            rhs: Pos{ row: 8, col: 2 },
        };

        assert_eq!(area.lhs.row, 1);
        assert_eq!(area.lhs.col, 10);
        assert_eq!(area.rhs.row, 8);
        assert_eq!(area.rhs.col, 2);
    }

    #[test]
    fn test_area_size() {
        let area = Area {
            lhs: Pos{ row: 1, col: 10 },
            rhs: Pos{ row: 8, col: 2 },
        };

        let rows = 8 - 1;
        let cols = 10 - 2;

        assert_eq!(area.size(), &rows * &cols);

        let area = Area {
            lhs: Pos{ row: 8, col: 2 },
            rhs: Pos{ row: 1, col: 10 },
        };

        assert_eq!(area.size(), &rows * &cols);
    }

    #[test]
    fn test_area_row_cnt() {
        let area = Area {
            lhs: Pos{ row: 4, col: 1 },
            rhs: Pos{ row: 24, col: 2 },
        };

        assert_eq!(area.row_cnt(), 24 - 4);

        let area = Area {
            lhs: Pos{ row: 20, col: 2 },
            rhs: Pos{ row:  5, col: 1 },
        };

        assert_eq!(area.row_cnt(), 20 - 5);
    }

    #[test]
    fn test_area_col_cnt() {
        let area = Area {
            lhs: Pos{ col: 4, row: 1 },
            rhs: Pos{ col: 24, row: 2 },
        };

        assert_eq!(area.col_cnt(), 24 - 4);

        let area = Area {
            lhs: Pos{ col: 20, row: 2 },
            rhs: Pos{ col:  5, row: 1 },
        };

        assert_eq!(area.col_cnt(), 20 - 5);
    }

    #[test]
    fn test_area_row_rng() {
        let area = Area {
            lhs: Pos{ row: 4, col: 1 },
            rhs: Pos{ row: 24, col: 2 },
        };

        assert_eq!(area.row_rng(), 4..=24);

        let area = Area {
            lhs: Pos{ row: 20, col: 2 },
            rhs: Pos{ row:  5, col: 1 },
        };

        assert_eq!(area.row_rng(), 5..=20);
    }

    #[test]
    fn test_area_col_rng() {
        let area = Area {
            lhs: Pos{ col: 4, row: 1 },
            rhs: Pos{ col: 24, row: 2 },
        };

        assert_eq!(area.col_rng(), 4..=24);

        let area = Area {
            lhs: Pos{ col: 20, row: 2 },
            rhs: Pos{ col:  5, row: 1 },
        };

        assert_eq!(area.col_rng(), 5..=20);
    }
}
