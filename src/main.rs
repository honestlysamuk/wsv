use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::Table;
use wsv::takepest::WsvValue as w;
use wsv::takepest::*;
fn main() {
    let elements = parse("./tests/strings.wsv").expect("no errors");
    let mut table = Table::new();
    table.apply_modifier(UTF8_ROUND_CORNERS);
    for line in elements {
        let mut row: Vec<String> = Vec::new();
        for value in line {
            let string_val = match value {
                w::Null => "NULL".to_string(),
                w::Value(s) => s,
                _ => unreachable!(),
            };
            row.push(string_val);
        }
        table.add_row(row);
    }

    println!("{table}");
}
