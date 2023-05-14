use displaytable::{row, EscCode, Table, TableData};

fn main() {
    let cols = vec!["A", "B", "C"];

    let rows = vec![
        row!("banana", "apple", "berry"),
        row!("bananarama", "appleicious", "huh"),
        row!(1, 2, 3),
        row!(1, 2, "this is getting absurd"),
    ];
    let data = TableData::new(cols, rows);

    let mut t = Table::new(&data);
    t.set_header_format(vec![EscCode::Bold, EscCode::Underline, EscCode::Red]);
    t.set_padding(20);

    t.print();
}
