pub enum EscCode {
    // Core escape character \x1b[
    Esc,
    // Text formatting
    Bold,
    Italic,
    Underline,
    // Colors
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

fn ansi_to_str(code: &EscCode) -> String {
    match code {
        EscCode::Esc => "\x1b[".into(),
        EscCode::Bold => "1".into(),
        EscCode::Italic => "3".into(),
        EscCode::Underline => "4".into(),
        EscCode::Black => "30".into(),
        EscCode::Red => "31".into(),
        EscCode::Green => "32".into(),
        EscCode::Yellow => "33".into(),
        EscCode::Blue => "34".into(),
        EscCode::Magenta => "35".into(),
        EscCode::Cyan => "36".into(),
        EscCode::White => "37".into(),
    }
}

pub fn build_format(codes: Vec<EscCode>) -> String {
    let fmt_str = codes
        .iter()
        .map(ansi_to_str)
        .collect::<Vec<String>>()
        .join(";");

    ansi_to_str(&EscCode::Esc) + &fmt_str + "m"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_format() {
        let fmts = vec![EscCode::Bold, EscCode::Underline, EscCode::Red];

        let fmt_str = &build_format(fmts);

        let expected = "\x1b[1;4;31m";

        assert_eq!(fmt_str, expected);
    }
}
