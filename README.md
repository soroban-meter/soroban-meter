# soroban-meter

**Resource profiling for Soroban smart contracts — inside your test suite and CI pipeline.**

[![Crates.io](https://img.shields.io/crates/v/soroban-meter.svg)](https://crates.io/crates/soroban-meter)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/GideonBature/soroban-meter/ci.yml)](https://github.com/GideonBature/soroban-meter/actions)

---

Soroban's fee model is multidimensional — CPU instructions, read/write bytes, ledger entry access, rent, and events are all metered independently. A transaction fails if refundable fees don't cover actual usage at execution time. Yet no first-class tool exists that surfaces this breakdown during development, where you can actually act on it.

`soroban-meter` fills that gap. It wraps the `soroban_sdk` test environment to capture the host's budget context after each function call and prints a per-function resource breakdown alongside your normal `cargo test` output. A companion CLI tool (`soroban-meter-cli`) calls `simulateTransaction` against any network and returns a structured report you can gate in CI.

---

## Features

* Per-function CPU instruction and memory byte breakdown in `cargo test`
* Regression detection — flag when a function costs more than a baseline across runs
* Structured JSON output for CI integration
* `simulateTransaction` wrapper for testnet/mainnet profiling via CLI
* Zero required changes to contract code — works purely from the test harness

---

## Quick Start

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
soroban-meter = "0.1"
```

Wrap your test environment:

```rust
#[cfg(test)]
mod test {
    use soroban_sdk::Env;
    use soroban_meter::MeterExt;

    #[test]
    fn test_swap_resource_usage() {
        let env = Env::default();
        env.mock_all_auths();

        // reset before the call you want to measure
        env.cost_estimate().budget().reset_default();

        // invoke your contract function
        let client = YourContractClient::new(&env, &env.register(YourContract, ()));
        client.swap(&token_a, &token_b, &amount);

        // print the full resource breakdown
        env.meter_report("swap");
    }
}
```

Running `cargo test` will print:

```
---- swap resource report ----

  Function : swap
  ┌─────────────────────────────┬──────────────────┬──────────────┐
  │ Cost Type                   │ CPU Instructions │ Memory Bytes │
  ├─────────────────────────────┼──────────────────┼──────────────┤
  │ WasmInsnExec                │        1,218,796 │            0 │
  │ MemAlloc                    │       10,780,932 │   44,711,897 │
  │ MemCpy                      │        5,714,003 │            0 │
  │ DispatchHostFunction        │        1,302,310 │            0 │
  │ VmInstantiation             │        3,948,165 │      520,032 │
  │ ComputeSha256Hash           │           89,754 │            0 │
  │ ...                         │              ... │          ... │
  ├─────────────────────────────┼──────────────────┼──────────────┤
  │ TOTAL                       │       23,054,000 │   45,231,929 │
  │ LIMIT                       │      100,000,000 │   41,943,040 │
  │ USAGE %                     │            23.0% │        107.8%│  ← WARNING: mem exceeds limit
  └─────────────────────────────┴──────────────────┴──────────────┘

  Estimated resource fee: ~0.0042 XLM
```

---

## CLI Usage

Install:

```bash
cargo install soroban-meter-cli
```

Profile a deployed function against testnet:

```bash
soroban-meter profile \
  --contract CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --function swap \
  --args '["token_a_address", "token_b_address", "1000000"]' \
  --network testnet
```

Output (default: human-readable table):

```
soroban-meter v0.1.0 — testnet

Contract : CDLZFC3S...CYSC
Function : swap
Ledger   : 49103821

┌──────────────────────┬──────────────────┬───────────┐
│ Resource             │ Used             │ Limit     │
├──────────────────────┼──────────────────┼───────────┤
│ CPU Instructions     │       23,054,000 │ 100000000 │
│ Memory Bytes         │       45,231,929 │  41943040 │  ← EXCEEDS LIMIT
│ Ledger Reads         │               12 │      -    │
│ Ledger Writes        │                4 │      -    │
│ Read Bytes           │           24,560 │      -    │
│ Write Bytes          │            8,192 │      -    │
│ Events + Return      │            1,024 │      -    │
├──────────────────────┼──────────────────┼───────────┤
│ Min Resource Fee     │          0.00421 XLM         │
└──────────────────────┴──────────────────┴───────────┘
```

Output as JSON (`--output json`):

```json
{
  "contract": "CDLZFC3S...CYSC",
  "function": "swap",
  "network": "testnet",
  "ledger": 49103821,
  "resources": {
    "cpu_insns": { "used": 23054000, "limit": 100000000 },
    "mem_bytes": { "used": 45231929, "limit": 41943040, "exceeds_limit": true },
    "ledger_reads": 12,
    "ledger_writes": 4,
    "read_bytes": 24560,
    "write_bytes": 8192,
    "events_and_return_bytes": 1024
  },
  "min_resource_fee_xlm": "0.00421"
}
```

---

## CI Integration

Gate your pipeline on resource thresholds:

```yaml
# .github/workflows/ci.yml
- name: Profile contract resource usage
  run: |
    soroban-meter profile \
      --contract $CONTRACT_ID \
      --function swap \
      --args "$ARGS" \
      --network testnet \
      --threshold cpu_insns=50000000 \
      --threshold mem_bytes=40000000 \
      --output json > meter_report.json

    # exits 1 if any threshold is crossed
```

Combine with regression detection (compare against a saved baseline):

```bash
soroban-meter profile ... --baseline ./baseline.json --fail-on-regression 10%
# fails CI if any resource metric increased by more than 10% from baseline
```

---

## Caveats

The `soroban_sdk` test environment runs native Rust rather than compiled WASM. As a result, CPU instruction counts from the test harness are **underestimates** relative to the actual on-chain WASM execution — this is a known property of the SDK's budget model. Use `soroban-meter-cli` with `simulateTransaction` for WASM-accurate measurements before mainnet deployment. The test harness output is useful for catching regressions and order-of-magnitude issues during development; the CLI output is the authoritative pre-deployment check.

---

## Project Structure

```
soroban-meter/
├── crates/
│   ├── soroban-meter/          # the Rust crate (dev-dependency)
│   │   ├── src/
│   │   │   ├── lib.rs          # MeterExt trait + report formatting
│   │   │   ├── report.rs       # table rendering, threshold evaluation
│   │   │   └── budget.rs       # budget context extraction helpers
│   │   └── Cargo.toml
│   └── soroban-meter-cli/      # the CLI binary
│       ├── src/
│       │   ├── main.rs
│       │   ├── simulate.rs     # simulateTransaction RPC wrapper
│       │   └── output.rs       # JSON + table formatters
│       └── Cargo.toml
├── examples/
│   ├── amm-swap/               # AMM swap contract profiling example
│   └── token-transfer/         # SEP-41 token transfer example
├── docs/
│   └── ci-integration.md
├── Cargo.toml
└── README.md
```

---

## Roadmap

* [x] `MeterExt` trait for in-test budget capture and reporting
* [x] `soroban-meter-cli` with `simulateTransaction` integration
* [x] JSON output + threshold gating for CI
* [ ] Baseline file format + regression detection (`--baseline`, `--fail-on-regression`)
* [ ] VS Code extension: inline resource cost annotations while writing contracts
* [ ] Historical profiling: track resource costs across Soroban protocol upgrades
* [ ] Scaffold Stellar plugin integration

---

## Contributing

This project is in active early development. Contributions, issues, and feedback are welcome.

If you've hit a Soroban resource limit you didn't see coming, or if you've written your own resource measurement workaround, please open an issue — your experience is directly useful to this project's design.

```bash
git clone https://github.com/GideonBature/soroban-meter
cd soroban-meter
cargo build
cargo test
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## Why This Exists

Soroban's fee model charges separately for CPU instructions, memory bytes, ledger reads and writes, rent, and events. Transactions fail silently when refundable fees don't cover actual usage. The canonical recommendation is to call `simulateTransaction` before submitting — but that only tells you the total cost, not which function or operation drove it, and it requires a deployed contract.

The only existing community tool (`@57blocks/stellar-resource-usage`) is a JavaScript stopgap with no CLI, no CI integration, and no regression detection. `soroban-meter` is the Rust-native answer: sits inside your test suite, requires no external infrastructure, and gives you function-level visibility you can act on during development.

---

## License

MIT — see [LICENSE](LICENSE).

---

## Author

Built by [Gideon Bature](https://github.com/GideonBature) — Bitcoin/Soroban open-source developer, BTrust BOSS '25 fellow, contributor to the [Bark/Ark Protocol](https://github.com/ark-network/ark).

Writing about this and related work at [dev.to/gideonbature](https://dev.to/gideonbature).
