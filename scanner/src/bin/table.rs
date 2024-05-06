use prettytable::format::{self, FormatBuilder, LinePosition};
use prettytable::{color, row, Attr, Cell, Row, Table};

fn main() {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.set_titles(row!["Title 1", "Title 2"]);
    table.add_row(row!["Value 1", "Value 2"]);
    table.add_row(row!["Value three", "Value four"]);
    table.printstd();
}
