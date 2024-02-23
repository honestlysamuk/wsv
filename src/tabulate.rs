use comfy_table::Table;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use core::fmt::Display;

pub trait Tabulatable {
    fn tabulate(self: Self) -> Table;
}
impl<T> Tabulatable for Vec<Vec<T>>  where T: Display {
    fn tabulate(self: Vec<Vec<T>>) -> Table {
        let mut table = Table::new();
        table.apply_modifier(UTF8_ROUND_CORNERS);
        for row in self {
            table.add_row(row.iter().map(|el| el.to_string()));
        }  
        table
     }
}