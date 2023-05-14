use displaytable::{row, Table, TableData};

#[test]
fn test_header() {
    let cols = vec!["A", "B", "C"];
    let data = TableData::new(cols, Vec::new());

    let t = Table::new(&data);
    let table_rep = t.represent();

    let expected = String::from("A  B  C  \n");

    assert_eq!(table_rep, expected);
}

#[test]
fn test_one_row() {
    let cols = vec!["A", "B", "C"];
    let rows = vec![row!("cat", "dog", "bird")];
    let data = TableData::new(cols, rows);

    let mut t = Table::new(&data);

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
    let cols = vec!["A", "B", "C"];

    let rows = vec![row!("cat", "dog", "bird"), row!("bird", "dog", "cat")];
    let data = TableData::new(cols, rows);

    let mut t = Table::new(&data);

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
