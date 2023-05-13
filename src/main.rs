// https://github.com/rodaine/table/blob/master/table.go

mod utils;

mod ansi_esc_codes;
pub use ansi_esc_codes::{build_format, EscCode};

// TODO: finalize trait
trait Table {
    fn new_table(headers: Vec<&str>, conf: Option<Config>) -> TableData;
    fn add_row(&mut self, row_data: Vec<String>);
    fn set_header_format(&mut self, fmt_str: String);

    fn calculate_widths(&self);
    fn print_header(&self);
    fn print_rows(&self);
    fn represent(&self) -> String;
    fn print(&self);
}

// TODO: finalize struct
#[derive(Debug, Clone)]
struct TableData {
    padding: usize,
    headers: Vec<String>,
    header_fmt: String,
    col_widths: Vec<usize>,
    rows: Vec<Vec<String>>,
}

#[derive(Debug)]
struct Config {
    // TODO
}

// TODO
// https://stackoverflow.com/questions/28951503/how-can-i-create-a-function-with-a-variable-number-of-arguments
impl Table for TableData {
    fn new_table(headers: Vec<&str>, conf: Option<Config>) -> TableData {
        match conf {
            Some(_conf) => {
                todo!()
            }
            None => {
                TableData {
                    headers: parse_args(&headers),
                    padding: 2,
                    header_fmt: String::from(""), //\x1b[1m
                    col_widths: headers
                        .iter()
                        .map(|h| h.len() + 2)
                        .collect::<Vec<usize>>(),
                    rows: Vec::new(),
                }
            }
        }
    }

    fn add_row(&mut self, row_data: Vec<String>) {
        self.rows.push(row_data)
    }

    fn set_header_format(&mut self, fmt_str: String) {
        self.header_fmt = fmt_str;
    }

    // TODO: implement
    fn calculate_widths(&self) {}

    // TODO: implement
    fn print_header(&self) {}

    // TODO: implement
    fn print_rows(&self) {}

    // TODO: complete
    fn represent(&self) -> String {
        let header_rep = format!(
            "{}{}{}",
            self.header_fmt,
            self.headers.join(&" ".repeat(self.padding)),
            String::from("") // "\x1b[0m" for bold
        );

        header_rep
    }

    // TODO: complete
    fn print(&self) {}
}

// TODO: replace?
fn parse_args(v: &Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn main() {
    let headers = vec!["A", "B", "C"];
    let t = TableData::new_table(headers, None);

    let table_rep = t.represent();

    println!("{}", table_rep);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let headers = vec!["A", "B", "C"];
        let t = TableData::new_table(headers, None);
        let table_rep = t.represent();

        let expected = String::from("A  B  C");

        assert_eq!(table_rep, expected);
    }

    #[test]
    fn test_col_widths_no_rows() {
        let headers = vec!["dog", "capybara", "panda"];
        let t = TableData::new_table(headers, None);

        let expected_widths = vec![5, 10, 7];

        assert_eq!(t.col_widths, expected_widths);
    }

    #[test]
    fn test_one_row() {
        let headers = vec!["A", "B", "C"];
        let mut t = TableData::new_table(headers, None);
        t.add_row(row!("cat", "dog", "bird"));

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
        let mut t = TableData::new_table(headers, None);
        t.add_row(row!("cat", "dog", "bird"));
        t.add_row(row!("bird", "dog", "cat"));

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
