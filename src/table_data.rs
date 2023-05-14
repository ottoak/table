type Row = Vec<String>;

#[derive(Debug, PartialEq)]
pub struct TableData {
    pub cols: Vec<String>,
    pub rows: Vec<Row>,
}

impl TableData {
    pub fn new(cols: Vec<&str>, rows: Vec<Row>) -> TableData {
        let mut td = TableData {
            cols: Self::construct_header(cols),
            rows: Vec::new(),
        };

        for row in rows {
            td.add_row(row);
        }

        td
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row)
    }

    fn construct_header(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_table_no_rows() {
        let cols = vec!["A", "B", "C"];

        let rows = Vec::new();
        let t = TableData::new(cols, rows);

        let expected = TableData {
            cols: vec!["A".to_string(), "B".into(), "C".into()],
            rows: Vec::new(),
        };

        assert_eq!(t, expected);
    }

    #[test]
    fn test_new_table_two_rows() {
        let cols = vec!["A", "B", "C"];

        let rows = vec![
            row!("test1", "test2", "test3"),
            row!("test4", "test5", "test6"),
        ];

        let t = TableData::new(cols, rows);

        let expected = TableData {
            cols: vec!["A".to_string(), "B".into(), "C".into()],
            rows: vec![
                row!("test1", "test2", "test3"),
                row!("test4", "test5", "test6"),
            ],
        };

        assert_eq!(t, expected);
    }
}
