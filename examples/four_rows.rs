use displaytable::{row, EscCode, Table, TableData};

fn main() {
    let cols = vec!["A", "B", "C"];
    let mut data = TableData::new(cols);

    data.add_row(row!("banana", "apple", "berry"));
    data.add_row(row!("bananarama", "appleicious", "huh"));
    data.add_row(row!(1, 2, 3));
    data.add_row(row!(1, 2, "this is getting absurd"));

    let mut t = Table::new(&data);
    t.set_header_format(vec![EscCode::Bold, EscCode::Underline, EscCode::Red]);
    t.calculate_widths();

    t.print();
}
