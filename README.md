# Universal Causal Language (UCL)

An experimental **Turing complete** intermediate representation designed to express **causality** across all domains: natural language, programming, law, biology, and art.

## Overview

UCL treats every statement, instruction, law, or behavior as a causal operation: a structured mapping from one state of the world to another. It can encode:

- Function calls in programming languages
- Sentences in natural language
- Legal contracts
- Musical scores
- DNA transcription events
- Recursive algorithms (fibonacci, quicksort, etc.)
- Control flow (if/else, loops, functions)

...all in the same underlying schema.

### ✨ Turing Completeness

UCL is now **Turing complete**, meaning it can compute anything that any other programming language can compute. It supports:

- **Conditional branching** (if/else)
- **Loops** (while, for)
- **Recursive functions** (DefineFunction)
- **Boolean logic** (and/or/not)
- **Arithmetic expressions**

See `TURING_COMPLETE.md` for details and `examples/fibonacci.json` for a working recursive Fibonacci implementation.

## One Program, Multiple Substrates

**The same UCL program can run on:**

1. **Ruby VM** (compiled to Ruby, executed on silicon)
2. **Brain VM** (simulated human cognition in software)
3. **Robot VM** (simulated physical operations)
4. **AI VM** (Mock LLM generates UCL from instructions)
5. **YOUR actual brain** (production mode - you execute it mentally)

Try it:

```bash
# Same program, three different execution environments
ucl run examples/multiply_universal.json --target ruby
ucl brain examples/multiply_universal.json --verbose
ucl brain examples/multiply_universal.json --production  # aka your real brain
```

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
- **Control Flow**: If, While, For, DefineFunction
- **Cooking**: Gather, Heat, Pour, Mix, Stir, Place, Remove, Steep, Serve
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

### Compile UCL to other languages

```bash
# Compile to Ruby
ucl compile examples/hello_world.json --target ruby

# Compile and save to file
ucl compile examples/simple_calc.json --target ruby --output program.rb
```

### Run UCL programs

```bash
# Compile to Ruby and execute
ucl run examples/hello_world.json --target ruby

# Execute on the brain VM (simulate language running on a human brain)
ucl run examples/natural_language.json --target brain

# Brain simulation with verbose output
ucl brain examples/natural_language.json --verbose

# Production Mode: Run on YOUR actual brain
ucl brain examples/brain_test.json --production
```

## Examples

### Natural Language

English: _"The cat is black."_

```json
{
  "actor": "listener",
  "op": "StoreFact",
  "target": "memory",
  "params": {
    "entity": "cat",
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

### Recipes

A recipe for making tea (runs on Brain VM or Robot VM):

```json
{
  "actor": "cook",
  "op": "Heat",
  "target": "water",
  "params": {
    "container": "kettle",
    "temperature": "100°C",
    "until": "boiling"
  },
  "dur": 180.0,
  "effects": ["Thermal"]
}
```

**Same recipe, different substrates:**

- **Brain VM**: Simulates a human following the recipe (cognitive operations)
- **Robot VM**: Simulates a robot executing the recipe (physical operations)

```bash
# Human brain following the recipe
ucl brain examples/recipe_tea.json --verbose

# Robot executing the recipe
ucl robot examples/recipe_tea.json --verbose
```

This demonstrates **substrate independence** - the same causal logic runs on different execution environments.

## Example Programs

The `examples/` directory contains complete UCL programs for various domains:

- **fibonacci.json** 🎯 - **Recursive** function with if/else and loops
- **ai_generate_factorial.json** 🤖 - **Meta-recursion**: AI generates UCL code
- **ai_chain.json** 🔗 - Full abstraction chain: AI → Compile → Execute
- **multiply_universal.json** ⭐ - Runs on all three substrates
- **recipe_tea.json** 🍵 - Runs on Brain VM and Robot VM
- **natural_language.json** - English sentences as UCL
- **ruby_code.json** - Ruby program execution
- **rust_code.json** - Rust program with memory management
- **music.json** - C major scale
- **legal_contract.json** - Purchase agreement
- **biology.json** - Central Dogma (DNA → RNA → Protein)
- **brain_test.json** - Test your actual brain in production mode
- **confusion_test.json** - Tests brain response to unknown operations
- **incomprehensible.json** - Advanced brain comprehension test

Try them out:

```bash
# Turing Complete: Recursive Fibonacci
ucl run examples/fibonacci.json
# Output: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55

# Meta-Recursion: An AI generating UCL code
ucl ai examples/ai_generate_factorial.json --verbose
# (NOTE: An AI could also generate UCL code that runs on another AI.)

# A Universal Program: runs on all three substrates
ucl run examples/multiply_universal.json --target ruby
ucl brain examples/multiply_universal.json --verbose
ucl brain examples/multiply_universal.json --production

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

## Universal Program Example

**multiply_universal.json** - A program that runs on all three execution environments.

The program:

1. Generates a random number (A)
2. Generates another random number (B)
3. Multiplies them
4. Outputs the result

**On Ruby VM:**

```ruby
A = rand(0..9)
B = rand(0..9)
result = A * B
puts result  # Output: 35 (varies each run)
```

**On Brain VM:**

```
Generated: A = 5
Generated: B = 3
Calculated: result = 5 × 3 = 15
Output: "15.0"
```

**On Production Brain (YOU):**

```
→ Think of a random number between 0 and 9
→ Remember it as 'A'
[You think: 7]

→ Calculate: A × B
→ Store the answer in: result
[You calculate: 7 × 4 = 28]

Output: "28"
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

## Brain VM: Language as Code

UCL includes a **virtual human brain** that executes natural language as programs:

```bash
ucl brain examples/natural_language.json --verbose
```

This simulates:

- **Beliefs** (long-term memory)
- **Emotions** (affective state)
- **Working memory** (short-term, limited capacity)
- **Thoughts** (internal narrative)
- **Goals** (intentions)
- **Output** (speech/expression)

When the brain encounters an unknown operation, it responds naturally:

```
💭 "Sorry, I don't know what that means: Transcribe"
🗣️  "I'm not sure what you mean..."
[confusion: +0.4, curiosity: +0.3]
```

See [BRAIN_VM.md](BRAIN_VM.md) for the full documentation on this groundbreaking concept.

### 🚀 Production Brain Mode

Take it to the next level - run UCL on YOUR actual biological brain:

```bash
ucl brain examples/brain_test.json --production
```

This interactive mode:

- Walks you through each operation
- You execute it mentally
- You report your internal state
- Captures real human cognitive performance

No simulation. No virtual machine. Just your neurons executing UCL operations. 🧠💼

See [PRODUCTION_BRAIN.md](PRODUCTION_BRAIN.md) for the full guide to running language on production wetware.

## 🖥️ UX as Parallel Computation

Every program with a UI is running parallel computation on both a CPU and a human brain.

When you use software, you're not "observing" - you're **executing a parallel program**:

```
CPU:   [Render UI] → [Wait for input] → [Process] → [Update]
Brain: [Perceive] → [Think] → [Decide] → [Execute motor command]
```

The "interface" is the **message-passing protocol** between two computational substrates.

Key realizations:

- **UX design is substrate coordination**
- **User testing is performance benchmarking** (of the brain substrate)
- **Intuitive UI = efficient brain code**
- **Frustration = brain execution failures**
- **Accessibility = alternative protocols for different brain substrates**

See [UX_AS_COMPUTATION.md](UX_AS_COMPUTATION.md) for the full exploration of this concept.

## Cross-Domain Compilation

UCL can compile to different execution targets:

```bash
# Compile to Ruby
ucl compile examples/hello_world.json --target ruby --output hello.rb
ruby hello.rb

# Execute on brain VM
ucl brain examples/natural_language.json
```

The same causal logic, different substrates.

## Future Directions

- **Multi-substrate coordination** - Run programs across CPU + Brain + AI in parallel
- **Domain adapters** for automatic translation (English → UCL, Python → UCL, etc.)
- **More compilation targets** (Python, JavaScript, neural networks)
- **UCL-to-UCL translators** for cross-domain compilation (Code → Legal, Music → Code)
- **Richer brain models** (episodic memory, reasoning, dreaming)
- **Visual editor** for UCL programs
- **REPL** for interactive UCL development
- **LLM training** on UCL datasets for better causal understanding
- **More universal programs** that work across all substrates
- **UX frameworks** that explicitly program both CPU and brain

## Contributing

This is an experimental project exploring fundamental questions about representation and causality. Contributions, feedback, and ideas are welcome!

## License

MIT
