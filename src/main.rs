// https://github.com/rodaine/table/blob/master/table.go

mod utils;

// TODO: finalize trait
trait Table {
    fn add_row(&mut self, row_data: Vec<String>);

    fn calculate_widths(&self);
    fn print_header(&self);
    fn print_rows(&self);
    fn represent(&self) -> String;
    fn print(&self);
}

// TODO: finalize struct
#[derive(Debug)]
struct TableData {
    padding: usize,
    headers: Vec<String>,
    header_fmt: String,
    //col_widths: Vec<usize>,
    rows: Vec<Vec<String>>,
}

#[derive(Debug)]
struct Config {
    // TODO
}

// TODO
// https://stackoverflow.com/questions/28951503/how-can-i-create-a-function-with-a-variable-number-of-arguments
impl TableData {
    fn new(headers: Vec<&str>, conf: Option<Config>) -> TableData {
        match conf {
            Some(_conf) => {
                todo!()
            }
            None => {
                TableData {
                    headers: parse_args(headers),
                    padding: 2,
                    header_fmt: String::from(""), //\x1b[1m
                    rows: Vec::new(),
                }
            }
        }
    }
}

impl Table for TableData {
    fn add_row(&mut self, row_data: Vec<String>) {
        self.rows.push(row_data)
    }

    // TODO: implement
    fn calculate_widths(&self) {}

    // TODO: implement
    fn print_header(&self) {}

    // TODO: implement
    fn print_rows(&self) {}

    // TODO: complete
    fn represent(&self) -> String {
        let rep = format!(
            "{}{}{}",
            self.header_fmt,
            self.headers.join(&" ".repeat(self.padding)),
            String::from("") // "\x1b[0m" for bold
        );

        rep
    }

    // TODO: complete
    fn print(&self) {}
}

// TODO: replace?
fn parse_args(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|x| x.to_string()).collect::<Vec<String>>()
}

fn main() {
    let headers = vec!["A", "B", "C"];
    let t = TableData::new(headers, None);

    let table_rep = t.represent();

    println!("{}", table_rep);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let headers = vec!["A", "B", "C"];
        let t = TableData::new(headers, None);
        let table_rep = t.represent();

        let expected = String::from("A  B  C");

        assert_eq!(table_rep, expected);
    }
    #[test]
    fn test_one_row() {
        let headers = vec!["A", "B", "C"];
        let mut t = TableData::new(headers, None);
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
        let mut t = TableData::new(headers, None);
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
