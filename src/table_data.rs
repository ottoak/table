// TODO: finalize struct
#[derive(Debug, PartialEq)]
pub struct TableData {
    pub cols: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl TableData {
    pub fn new(cols: Vec<&str>) -> TableData {
        TableData {
            cols: Self::construct_header(cols),
            rows: Vec::new(),
        }
    }
    pub fn add_row(&mut self, row_data: Vec<String>) {
        self.rows.push(row_data)
    }

    fn construct_header(v: Vec<&str>) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_table() {
        let cols = vec!["A", "B", "C"];

        let t = TableData::new(cols);

        let expected = TableData {
            cols: vec!["A".to_string(), "B".into(), "C".into()],
            rows: Vec::new(),
        };

        assert_eq!(t, expected);
    }
}
