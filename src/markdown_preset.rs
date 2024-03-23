use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    Table,
};

pub trait MarkdownPreset {
    fn new_markdown_table() -> Self;
}

impl MarkdownPreset for Table {
    fn new_markdown_table() -> Self {
        let mut table = Table::new();

        let format = FormatBuilder::new()
            .column_separator('|')
            .borders('|')
            .separator(LinePosition::Title, LineSeparator::new('-', '|', '|', '|'))
            .separators(&[], LineSeparator::new('-', '|', '|', '|'))
            .padding(1, 1)
            .build();

        table.set_format(format);

        table
    }
}

#[test]
fn test_markdown_table() {
    use pretty_assertions::assert_eq;
    use prettytable::{Cell, Row, Table};

    let mut table = Table::new_markdown_table();

    table.set_titles(Row::new(vec![Cell::new("Title 1"), Cell::new("Title 2")]));
    table.add_row(Row::new(vec![Cell::new("Value 1"), Cell::new("Value 2")]));
    table.add_row(Row::new(vec![
        Cell::new("Value three"),
        Cell::new("Value four"),
    ]));

    let mut buffer = Vec::new();

    table.print(&mut buffer).unwrap();

    let output = String::from_utf8_lossy(&buffer);

    let expected_output = "\
        | Title 1     | Title 2    |\n\
        |-------------|------------|\n\
        | Value 1     | Value 2    |\n\
        | Value three | Value four |\n\
    ";

    let output_normalized = output.replace("\r\n", "\n");
    let expected_output_normalized = expected_output.replace("\r\n", "\n");

    assert_eq!(output_normalized, expected_output_normalized);
}
