use crate::simulate::SimulateResult;
use comfy_table::{Cell, Color, Table};

pub fn print_table(result: &SimulateResult, contract: &str, function: &str, network: &str) {
    println!("soroban-meter v0.1.0 — {}", network);
    println!("\nContract : {}", contract);
    println!("Function : {}", function);
    println!("\n");

    let mut table = Table::new();
    table.set_header(vec!["Resource", "Used", "Limit"]);

    table.add_row(vec![
        Cell::new("CPU Instructions"),
        Cell::new(result.cpu_insns),
        Cell::new("100000000"),
    ]);

    table.add_row(vec![
        Cell::new("Memory Bytes"),
        Cell::new(result.mem_bytes),
        Cell::new("41943040").fg(Color::Yellow),
    ]);

    table.add_row(vec![
        Cell::new("Ledger Reads"),
        Cell::new(result.ledger_reads),
        Cell::new("-"),
    ]);

    table.add_row(vec![
        Cell::new("Ledger Writes"),
        Cell::new(result.ledger_writes),
        Cell::new("-"),
    ]);

    println!("{table}");
    println!("\nMin Resource Fee: {} XLM", result.min_resource_fee);
}

pub fn print_json(result: &SimulateResult) {
    if let Ok(json) = serde_json::to_string_pretty(result) {
        println!("{}", json);
    }
}

pub fn check_regression(
    current: &SimulateResult,
    baseline_path: &str,
    fail_on_regression: &Option<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let baseline_data = std::fs::read_to_string(baseline_path)?;
    let baseline: SimulateResult = serde_json::from_str(&baseline_data)?;

    let threshold_percent = fail_on_regression.unwrap_or(0.0);
    let mut regression_found = false;

    let check = |name: &str, curr: u64, base: u64| {
        if base == 0 {
            return false;
        }
        let diff = (curr as f64 - base as f64) / (base as f64) * 100.0;
        if diff > threshold_percent {
            println!(
                "Regression detected in {}: Current = {}, Baseline = {} (+{:.2}%)",
                name, curr, base, diff
            );
            true
        } else {
            false
        }
    };

    regression_found |= check("CPU Instructions", current.cpu_insns, baseline.cpu_insns);
    regression_found |= check("Memory Bytes", current.mem_bytes, baseline.mem_bytes);
    regression_found |= check(
        "Ledger Reads",
        current.ledger_reads as u64,
        baseline.ledger_reads as u64,
    );
    regression_found |= check(
        "Ledger Writes",
        current.ledger_writes as u64,
        baseline.ledger_writes as u64,
    );
    regression_found |= check(
        "Read Bytes",
        current.read_bytes as u64,
        baseline.read_bytes as u64,
    );
    regression_found |= check(
        "Write Bytes",
        current.write_bytes as u64,
        baseline.write_bytes as u64,
    );

    if regression_found {
        println!(
            "Error: Regression exceeded allowed threshold ({}%)",
            threshold_percent
        );
        std::process::exit(1);
    }

    Ok(())
}
