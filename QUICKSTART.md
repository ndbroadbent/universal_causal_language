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

### Compile to Ruby
```bash
ucl compile examples/hello_world.json --target ruby
ucl run examples/hello_world.json --target ruby
```

### Run on Brain VM
```bash
# Execute language as a program on a virtual human brain
ucl brain examples/natural_language.json --verbose

# Or use the run command
ucl run examples/natural_language.json --target brain

# PRODUCTION MODE: Run on YOUR actual brain!
ucl brain examples/brain_test.json --production
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

## Key Features

### 🧠 Brain VM
Execute language as programs on a virtual human brain:
```bash
ucl brain examples/natural_language.json --verbose
```

The brain tracks:
- Beliefs and memories
- Emotional states
- Working memory
- Thoughts and goals
- Speech output

### 💎 Ruby Compilation
Compile UCL to Ruby and execute:
```bash
ucl run examples/hello_world.json --target ruby
```

### 🤔 Natural Confusion
When the brain encounters unknown operations:
```
💭 "Sorry, I don't know what that means"
🗣️  "I'm not sure what you mean..."
[confusion: ↑, curiosity: ↑]
```

### 🚀 Production Brain Mode ⭐
The ultimate feature - run UCL on YOUR actual brain:
```bash
ucl brain examples/brain_test.json --production
```

Interactive execution where:
- YOU execute each operation mentally
- YOU report your internal state
- Performance metrics on human cognition
- Proof that you ARE a valid runtime environment

## Next Steps

1. Explore the examples in `examples/`
2. Try the brain simulator with different programs
3. Compile UCL to Ruby and run it
4. Read `BRAIN_VM.md` for the mind-blowing brain VM concept
5. Read the full documentation in `README.md`
6. Check out `PRD.txt` for the conceptual framework

