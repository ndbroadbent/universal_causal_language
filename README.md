# Universal Causal Language (UCL)

An experimental intermediate representation designed to express **causality** across all domains: natural language, programming, law, biology, and art.

## Overview

UCL treats every statement, instruction, law, or behavior as a causal operation: a structured mapping from one state of the world to another. It can encode:
- Function calls in programming languages
- Sentences in natural language
- Legal contracts
- Musical scores
- DNA transcription events

...all in the same underlying schema.

## Installation

```bash
cargo build --release
```

The CLI tool will be available at `target/release/ucl`.

## Core Concepts

### Action Schema

Each UCL instruction is an **Action** with the following structure:

```rust
{
  actor: String,           // who or what initiates the cause
  op: Operation,           // what kind of action occurs
  target: String,          // what is acted upon
  t: Option<f64>,          // when the action occurs (optional)
  dur: Option<f64>,        // how long it lasts (optional)
  params: Option<HashMap>, // contextual arguments (optional)
  pre: Option<String>,     // required preconditions (optional)
  post: Option<String>,    // resulting conditions (optional)
  effects: Option<Vec>,    // domain tags (optional)
}
```

### Operations

UCL supports the following primitive operations:

- **CRUD**: Create, Read, Write, Delete
- **Binding**: Bind, Unbind
- **Communication**: Emit, Receive
- **Observation**: Measure, Decide
- **Temporal**: Wait
- **Logical**: Assert, StoreFact
- **Legal**: Oblige, Permit, Remedy
- **Biological**: Transcribe, Translate, Express
- **Programming**: Call, Assign, Return
- **Custom**: Custom(String) for domain-specific operations

## CLI Usage

### Validate a UCL file

```bash
ucl validate examples/natural_language.json
```

### Display a UCL program

```bash
ucl display examples/music.json

# Compact output
ucl display --compact examples/ruby_code.json
```

### Analyze a UCL program

```bash
ucl analyze examples/biology.json
```

This provides statistics about operations, actors, domains, and temporal characteristics.

### Convert formats

```bash
ucl convert examples/rust_code.json --format json
```

## Examples

### Natural Language

English: *"The cat is black."*

```json
{
  "actor": "listener_brain",
  "op": "StoreFact",
  "target": "cat",
  "params": {
    "color": "black"
  }
}
```

### Programming (Ruby)

Ruby code: `result = 2 + 3`

```json
{
  "actor": "VM",
  "op": "Call",
  "target": "+",
  "params": {
    "lhs": 2,
    "rhs": 3,
    "receiver": "a"
  },
  "effects": ["CPU"]
}
```

### Music

A piano note:

```json
{
  "actor": "Piano1",
  "op": "Emit",
  "target": "Note",
  "t": 0.0,
  "dur": 0.5,
  "params": {
    "pitch": "C4",
    "velocity": 80
  },
  "effects": ["Audio"]
}
```

### Legal Contract

A payment obligation:

```json
{
  "actor": "Buyer",
  "op": "Oblige",
  "target": "Buyer",
  "params": {
    "duty": "Pay",
    "amount": "1000 USD",
    "by": "Delivery+5d"
  },
  "pre": "Goods delivered and inspected",
  "effects": ["Legal"]
}
```

### Biology

DNA transcription:

```json
{
  "actor": "RNA_Polymerase_II",
  "op": "Transcribe",
  "target": "DNA:MYC",
  "params": {
    "product": "pre-mRNA:MYC",
    "location": "nucleus"
  },
  "pre": "Promoter accessible",
  "post": "Pre-mRNA synthesized",
  "effects": ["Bio", "Nucleus"]
}
```

## Example Programs

The `examples/` directory contains complete UCL programs for various domains:

- **natural_language.json** - English sentences as UCL
- **ruby_code.json** - Ruby program execution
- **rust_code.json** - Rust program with memory management
- **music.json** - C major scale
- **legal_contract.json** - Purchase agreement
- **biology.json** - Central Dogma (DNA → RNA → Protein)

Try them out:

```bash
# View the biology example
ucl display examples/biology.json

# Analyze the music example
ucl analyze examples/music.json

# Validate all examples
for file in examples/*.json; do
  echo "Validating $file"
  ucl validate "$file"
done
```

## Library Usage

You can use UCL as a Rust library:

```rust
use ucl::{Action, Operation, Program};
use std::collections::HashMap;

// Create a simple action
let action = Action::new("VM", Operation::Call, "add")
    .with_time(0.0)
    .with_effects(vec!["CPU".to_string()]);

// Build a program
let mut program = Program::new();
program.add_action(action);

// Serialize to JSON
let json = program.to_json()?;

// Parse from JSON
let parsed = Program::from_json(&json)?;
```

## Design Philosophy

UCL is based on the hypothesis that **all forms of communication are causal programs** executed on different substrates (brains, CPUs, societies, cells, etc.). By reducing all meaning to causal primitives, UCL aims to:

1. **Unify representation** across domains
2. **Preserve semantics** during translation
3. **Enable cross-domain reasoning** and compilation
4. **Bridge human and machine understanding**

## Future Directions

- **Domain adapters** for automatic translation (English → UCL, Python → UCL, etc.)
- **UCL-to-UCL translators** for cross-domain compilation
- **Visual editor** for UCL programs
- **REPL** for interactive UCL development
- **LLM training** on UCL datasets

## Contributing

This is an experimental project exploring fundamental questions about representation and causality. Contributions, feedback, and ideas are welcome!

## License

MIT

