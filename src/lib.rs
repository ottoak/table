// https://github.com/rodaine/table/blob/master/table.go
#[macro_use]
mod utils;

mod ansi_esc_codes;
pub use ansi_esc_codes::{build_format, EscCode};

mod table_data;
pub use table_data::TableData;

mod table;
pub use table::Table;
