// https://github.com/rodaine/table/blob/master/table.go

mod utils;

mod ansi_esc_codes;
pub use ansi_esc_codes::{build_format, EscCode};

// TODO: finalize trait
trait Table {
    fn new_table(headers: Vec<&str>) -> TableData;
    fn add_row(&mut self, row_data: Vec<String>);
    fn set_header_format(&mut self, fmt: Vec<EscCode>);

    fn calculate_widths(&mut self);
    fn header_string(&self) -> String;
    fn data_string(&self) -> String;
    fn represent(&self) -> String;
    fn print_header(&self);
    fn print_rows(&self);
    fn print(&self);
}

// TODO: finalize struct
#[derive(Debug)]
struct TableData {
    padding: usize,
    headers: Vec<String>,
    header_fmt: String,
    col_widths: Vec<usize>,
    rows: Vec<Vec<String>>,
}

// TODO
// https://stackoverflow.com/questions/28951503/how-can-i-create-a-function-with-a-variable-number-of-arguments
impl Table for TableData {
    fn new_table(headers: Vec<&str>) -> TableData {
        let default_padding = 2;
        TableData {
            headers: parse_args(&headers),
            padding: default_padding, // default padding of 2
            header_fmt: String::from(""),
            col_widths: headers
                .iter()
                .map(|h| h.len() + default_padding)
                .collect::<Vec<usize>>(),
            rows: Vec::new(),
        }
    }

    fn add_row(&mut self, row_data: Vec<String>) {
        self.rows.push(row_data)
    }

    fn set_header_format(&mut self, fmt: Vec<EscCode>) {
        self.header_fmt = build_format(fmt);
    }

    fn calculate_widths(&mut self) {
        for row in self.rows.iter() {
            for (i, val) in row.iter().enumerate() {
                if val.len() + self.padding > self.col_widths[i] {
                    self.col_widths[i] = val.len() + self.padding;
                }
            }
        }
    }

    fn represent(&self) -> String {
        format!("{}\n{}", self.header_string(), self.data_string())
    }

    fn header_string(&self) -> String {
        let mut end_code = String::from("");

        if !self.header_fmt.is_empty() {
            end_code = String::from("\x1b[0m");
        }

        let header_rep = format!(
            "{}{}{}",
            self.header_fmt,
            self.headers
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

    fn data_string(&self) -> String {
        let mut data_rep = String::from("");

        for row in self.rows.iter() {
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

    fn print_header(&self) {
        println!("{}", self.header_string())
    }

    fn print_rows(&self) {
        println!("{}", self.data_string())
    }

    fn print(&self) {
        self.print_header();
        self.print_rows();
    }
}

// TODO: replace?
fn parse_args(v: &[&str]) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn main() {
    let headers = vec!["A", "B", "C"];
    let mut t = TableData::new_table(headers);
    t.set_header_format(vec![EscCode::Bold, EscCode::Underline, EscCode::Red]);
    t.add_row(row!("banana", "apple", "berry"));
    t.add_row(row!("bananarama", "appleicious", "huh"));
    t.calculate_widths();

    t.print();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let headers = vec!["A", "B", "C"];
        let t = TableData::new_table(headers);
        let table_rep = t.represent();

        let expected = String::from("A  B  C  \n");

        assert_eq!(table_rep, expected);
    }

    #[test]
    fn test_col_widths_no_rows() {
        let headers = vec!["dog", "capybara", "panda"];
        let t = TableData::new_table(headers);

        let expected_widths = vec![5, 10, 7];

        assert_eq!(t.col_widths, expected_widths);
    }

    #[test]
    fn test_col_widths_with_rows() {
        let headers = vec!["dog", "capybara", "panda"];
        let mut t = TableData::new_table(headers);
        t.add_row(row!("banana", "apple", "berry"));

        t.calculate_widths();
        let expected_widths = vec![8, 10, 7];

        assert_eq!(t.col_widths, expected_widths);
    }

    #[test]
    fn test_one_row() {
        let headers = vec!["A", "B", "C"];
        let mut t = TableData::new_table(headers);
        t.add_row(row!("cat", "dog", "bird"));
        t.calculate_widths();

        let table_rep = t.represent();

        let expected = String::from(
            "\
A    B    C     
cat  dog  bird  
",
        );

        assert_eq!(table_rep, expected);
    }

    #[test]
    fn test_two_rows() {
        let headers = vec!["A", "B", "C"];
        let mut t = TableData::new_table(headers);
        t.add_row(row!("cat", "dog", "bird"));
        t.add_row(row!("bird", "dog", "cat"));
        t.calculate_widths();

        let table_rep = t.represent();

        let expected = String::from(
            "\
A     B    C     
cat   dog  bird  
bird  dog  cat   
",
        );

        assert_eq!(table_rep, expected);
    }
}
