use crate::ansi_esc_codes::{build_format, EscCode};
use crate::table_data::TableData;

// TODO: finalize struct
#[derive(Debug)]
pub struct Table<'a> {
    data: &'a TableData,
    padding: usize,
    header_fmt: String,
    col_widths: Vec<usize>,
}

// TODO
// https://stackoverflow.com/questions/28951503/how-can-i-create-a-function-with-a-variable-number-of-arguments
impl<'a> Table<'a> {
    pub fn new(data: &'a TableData) -> Table<'a> {
        let default_padding = 2;
        Table {
            data: data,
            padding: default_padding,
            header_fmt: String::from(""),
            col_widths: data
                .cols
                .iter()
                .map(|h| h.len() + default_padding)
                .collect::<Vec<usize>>(),
        }
    }

    pub fn set_header_format(&mut self, fmt: Vec<EscCode>) {
        self.header_fmt = build_format(fmt);
    }

    pub fn calculate_widths(&mut self) {
        for row in self.data.rows.iter() {
            for (i, val) in row.iter().enumerate() {
                if val.len() + self.padding > self.col_widths[i] {
                    self.col_widths[i] = val.len() + self.padding;
                }
            }
        }
    }

    pub fn represent(&self) -> String {
        format!("{}\n{}", self.header_string(), self.data_string())
    }

    pub fn header_string(&self) -> String {
        let mut end_code = String::from("");

        if !self.header_fmt.is_empty() {
            end_code = String::from("\x1b[0m");
        }

        let header_rep = format!(
            "{}{}{}",
            self.header_fmt,
            self.data
                .cols
                .iter()
                .enumerate()
                .map(|(i, c)| c.to_owned()
                    + &" ".repeat(self.col_widths[i] - c.len()))
                .collect::<Vec<String>>()
                .join(""),
            end_code
        );

        header_rep
    }

    pub fn data_string(&self) -> String {
        let mut data_rep = String::from("");

        for row in self.data.rows.iter() {
            data_rep += &format!(
                "{}\n",
                row.iter()
                    .enumerate()
                    .map(|(i, d)| d.to_owned()
                        + &" ".repeat(self.col_widths[i] - d.len()))
                    .collect::<Vec<String>>()
                    .join("")
            )
        }

        data_rep
    }

    pub fn print_header(&self) {
        println!("\n{}", self.header_string())
    }

    pub fn print_rows(&self) {
        println!("{}", self.data_string())
    }

    pub fn print(&self) {
        self.print_header();
        self.print_rows();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_col_widths_no_rows() {
        let cols = vec!["dog", "capybara", "panda"];
        let data = TableData::new(cols);

        let t = Table::new(&data);

        let expected_widths = vec![5, 10, 7];

        assert_eq!(t.col_widths, expected_widths);
    }

    #[test]
    fn test_col_widths_with_rows() {
        let cols = vec!["dog", "capybara", "panda"];

        let mut data = TableData::new(cols);
        data.add_row(row!("banana", "apple", "berry"));

        let mut t = Table::new(&data);

        t.calculate_widths();
        let expected_widths = vec![8, 10, 7];

        assert_eq!(t.col_widths, expected_widths);
    }
}
