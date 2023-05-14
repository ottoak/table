#[macro_export]
macro_rules! row {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_row = Vec::new();
            $(
                temp_row.push($x.to_string());
            )*

			temp_row
        }
    };
}

pub use row;
