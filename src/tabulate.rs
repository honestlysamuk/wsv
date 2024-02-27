use crate::pest::WsvValue;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::{Attribute, Cell, Color, Table};

pub fn tabulate(wsv: Vec<Vec<WsvValue>>) -> Table {
    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);
    for line in wsv {
        table.add_row(line.iter().map(|el| {
            match el {
                WsvValue::Value(val) => {
                    if val == "" {
                        Cell::new("Empty")
                            .add_attribute(Attribute::Bold)
                            .fg(Color::Blue)
                    } else {
                        Cell::new(val)
                    }
                }
                WsvValue::Null => Cell::new("NULL")
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Green),
                _ => unreachable!(),
            }
        }));
    }
    table
}
