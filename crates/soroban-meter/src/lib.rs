pub mod budget;
pub mod report;

use soroban_sdk::Env;

/// Extension trait for the Soroban `Env` to enable inline resource profiling.
/// 
/// This trait provides methods to automatically capture the current state of the 
/// Soroban host budget (CPU instructions and memory bytes consumed) and print a 
/// formatted report. It requires the `testutils` feature of the `soroban-sdk`.
pub trait MeterExt {
    /// Prints a beautifully formatted terminal table displaying the CPU instructions
    /// and memory bytes consumed by the Soroban environment up to the point this 
    /// function is called.
    ///
    /// # Arguments
    /// * `function_name` - A string slice that holds the name of the function being profiled, 
    ///                     which will be displayed in the header of the report.
    fn meter_report(&self, function_name: &str);
}

impl MeterExt for Env {
    fn meter_report(&self, function_name: &str) {
        let budget = self.budget();
        let cpu_insns = budget.cpu_instruction_cost();
        let mem_bytes = budget.memory_bytes_cost();

        report::print_report(function_name, cpu_insns, mem_bytes);
    }
}
