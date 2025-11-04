# UCL Quick Start Guide

## Build & Install

```bash
cargo build --release
```

The CLI will be at `target/release/ucl`.

## Run the Demo

```bash
./demo.sh
```

## CLI Commands

### Validate a UCL file
```bash
ucl validate examples/natural_language.json
```

### Display a UCL program
```bash
ucl display examples/music.json
ucl display --compact examples/biology.json
```

### Analyze a UCL program
```bash
ucl analyze examples/ruby_code.json
```

### Convert formats
```bash
ucl convert examples/rust_code.json --format json
```

## Run Tests

```bash
cargo test
```

This runs:
- 3 unit tests (core functionality)
- 7 integration tests (all examples)

## Example Files

All examples are in the `examples/` directory:

- **natural_language.json** - English sentences as causal actions
- **ruby_code.json** - Ruby program execution trace
- **rust_code.json** - Rust program with memory management
- **music.json** - C major scale as temporal audio events
- **legal_contract.json** - Purchase agreement with obligations
- **biology.json** - Central Dogma (DNA → RNA → Protein)

## Using as a Library

```rust
use ucl::{Action, Operation, Program};
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    // Create an action
    let action = Action::new("VM", Operation::Call, "function")
        .with_time(0.0)
        .with_effects(vec!["CPU".to_string()]);

    // Build a program
    let mut program = Program::new();
    program.add_action(action);

    // Serialize
    println!("{}", program.to_json()?);

    Ok(())
}
```

## Next Steps

1. Explore the examples in `examples/`
2. Try creating your own UCL programs
3. Read the full documentation in `README.md`
4. Check out `PRD.txt` for the conceptual framework

