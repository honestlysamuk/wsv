use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::Table;
use core::fmt::Display;

fn tabulate(wsv: Vec<Vec<WsvValue>>) -> Table {
    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);
    for line in wsv {
        table.add_row(line.iter().map(|el| el.into()));
    }
    table
}
