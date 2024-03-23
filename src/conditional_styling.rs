use prettytable::{Attr, Cell};

pub trait ConditionalStyling {
    fn style_with_condition<F>(self, callback: F, attr: Attr) -> Self
    where
        Self: Sized,
        F: Fn(&Self) -> bool;
}

impl ConditionalStyling for Cell {
    fn style_with_condition<F>(mut self, callback: F, attr: Attr) -> Self
    where
        F: Fn(&Self) -> bool,
    {
        if callback(&self) {
            self.style(attr);
        }

        self
    }
}

#[test]
fn test_conditional_cell_styling() {
    use pretty_assertions::assert_eq;
    use prettytable::{color, Attr, Cell, Row, Table};

    let mut table = Table::new();

    let correct_value = "42";
    let value = correct_value;

    table.add_row(Row::new(vec![Cell::new(value).style_with_condition(
        |c| c.get_content() == correct_value.to_string(),
        Attr::ForegroundColor(color::GREEN),
    )]));

    let mut buffer = Vec::new();

    table.print_html(&mut buffer).unwrap();

    let output = String::from_utf8_lossy(&buffer);

    let expected_output =
        r#"<table><tr><td style="color: #00aa00;text-align: left;">42</td></tr></table>"#;

    let output_normalized = output.replace("\r\n", "\n");
    let expected_output_normalized = expected_output.replace("\r\n", "\n");

    assert_eq!(output_normalized, expected_output_normalized);
}
