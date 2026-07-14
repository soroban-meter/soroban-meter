use comfy_table::{Cell, Color, Table};

pub fn print_report(function_name: &str, cpu_insns: u64, mem_bytes: u64) {
    println!("---- {} resource report ----\n", function_name);
    println!("  Function : {}", function_name);

    let mut table = Table::new();
    table.set_header(vec!["Resource", "Used", "Limit"]);

    table.add_row(vec![
        Cell::new("CPU Instructions"),
        Cell::new(cpu_insns),
        Cell::new(100_000_000).fg(Color::Yellow),
    ]);

    table.add_row(vec![
        Cell::new("Memory Bytes"),
        Cell::new(mem_bytes),
        Cell::new(41_943_040).fg(Color::Yellow),
    ]);

    println!("{table}");
}
