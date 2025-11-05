# UCL Execution Modes

## Three Ways to Run UCL Programs

### 1. Single Substrate Execution

Run a program on ONE execution environment.

```bash
# Ruby VM (silicon)
ucl run examples/hello_world.json --target ruby

# Brain VM (simulated wetware)
ucl brain examples/natural_language.json --verbose

# Production Brain (YOUR neurons!)
ucl brain examples/brain_test.json --production
```

### 2. Universal Execution

The **SAME program** runs on MULTIPLE substrates (interchangeably).

```bash
# Run multiply_universal.json on Ruby VM
ucl run examples/multiply_universal.json --target ruby
# Output: 35 (random)

# Run THE SAME program on Brain VM
ucl brain examples/multiply_universal.json --verbose
# Output: 12.0 (different random)

# Run THE SAME program on YOUR brain
ucl brain examples/multiply_universal.json --production
# Output: 28 (you choose the numbers)
```

**Same causal operations. Different execution environments. Universal computation.**

### 3. Parallel Multi-Substrate Execution ⭐

**MULTIPLE substrates** work together on the SAME problem!

```bash
ucl parallel examples/parallel_compute.json --verbose
```

Example execution:
```
💎 Ruby VM: Generate random number → 4
🧠 Brain VM: Generate random number → 7

💎 Ruby VM: Multiply 4 × 2 = 8
🧠 Brain VM: Multiply 7 × 3 = 21

🌐 Coordinator: Receive from Brain → 21
🌐 Coordinator: Send to Ruby → 21

💎 Ruby VM: Add 8 + 21 = 29
💎 Ruby VM: Output → 29

✨ Silicon + Wetware collaborated to compute the answer!
```

## Comparison

| Mode | Substrates | Use Case | Example |
|------|-----------|----------|---------|
| **Single** | 1 | Normal execution | `ucl run hello.json --target ruby` |
| **Universal** | 1 (any) | Substrate-independent code | `multiply_universal.json` |
| **Parallel** | 2+ | Distributed heterogeneous computing | `parallel_compute.json` |

## Architecture

### Single Substrate
```
UCL Program → Compiler/Interpreter → Substrate → Output
```

### Universal
```
            → Ruby Compiler → Ruby VM → Output
UCL Program → Brain Interpreter → Brain VM → Output
            → Production Interface → YOUR Brain → Output
```

### Parallel
```
                    → Ruby VM ──┐
UCL Program → Coordinator → Brain VM ──┤→ Coordinator → Combined Result
                    ← Message Passing ─┘
```

## Real-World Analogy

### Single Substrate
Like running a program on your laptop.

### Universal
Like Java's "Write Once, Run Anywhere" - but for ALL substrates (silicon, software, wetware).

### Parallel
Like MapReduce or distributed computing - but across **fundamentally different types of processors** (CPU + Brain).

## Technical Details

### Universal Programs

Requirements:
- Use only operations supported by all target substrates
- No substrate-specific assumptions
- Pure causal logic

Example operations that work everywhere:
- `GenRandomInt` - Ruby uses `rand()`, Brain uses mental random generation
- `Write` with operations - Both can do arithmetic
- `Emit` - Ruby uses `puts`, Brain uses speech/thought output

### Parallel Programs

Requirements:
- Explicitly specify actor (substrate) for each action
- Use Coordinator for inter-substrate communication
- Handle different execution speeds gracefully

Example pattern:
```json
{"actor": "SubstrateA", "op": "Compute", "target": "result_a"},
{"actor": "SubstrateB", "op": "Compute", "target": "result_b"},
{"actor": "Coordinator", "op": "Receive", "target": "result_b", "params": {"source": "SubstrateB"}},
{"actor": "Coordinator", "op": "Emit", "target": "result_b", "params": {"destination": "SubstrateA"}},
{"actor": "SubstrateA", "op": "Combine", "target": "final", "params": {"inputs": ["result_a", "result_b"]}}
```

## Performance Characteristics

### Ruby VM
- **Speed**: Milliseconds
- **Precision**: Exact
- **Parallelism**: Limited by CPU cores
- **Random**: True randomness (system entropy)
- **Cost**: Electricity

### Brain VM (Simulated)
- **Speed**: Microseconds (simulation)
- **Precision**: Exact (software)
- **Parallelism**: Sequential (single-threaded simulation)
- **Random**: Pseudo-random (hash-based)
- **Cost**: Minimal

### Production Brain (Human)
- **Speed**: Seconds (human thinking time)
- **Precision**: Approximate (human estimation)
- **Parallelism**: Massive (unconscious processing)
- **Random**: True randomness (neural chaos)
- **Cost**: Attention, mental energy

### Parallel (Ruby + Brain VM)
- **Speed**: Limited by slowest substrate
- **Precision**: Mixed (exact from Ruby, simulated from Brain)
- **Parallelism**: True parallel across heterogeneous processors
- **Random**: Multiple sources
- **Cost**: Combined

## Use Cases

### Single Substrate
- Testing individual substrate capabilities
- Simple programs
- Performance benchmarking

### Universal
- Cross-platform code
- Educational demonstrations
- Proving substrate independence
- Testing human computational abilities

### Parallel
- Distributed computing across CPU + Human
- AI-Human collaboration (add GPU substrate)
- Multi-agent systems
- Research on substrate coordination
- Future: Real distributed cognition

## Future: Adding More Substrates

The coordinator can support:

- **GPU** (parallel numerical computation)
- **TPU** (AI model inference)
- **Quantum** (quantum operations)
- **DNA** (biological computation)
- **Multiple Humans** (distributed human computation)
- **Physical Robots** (embodied cognition)

All coordinated through UCL's universal causal interface!

## Example: Three-Way Parallel

```json
{
  "actions": [
    {"actor": "RubyVM", "op": "GenRandomInt", "target": "A"},
    {"actor": "BrainVM", "op": "GenRandomInt", "target": "B"},
    {"actor": "GPU", "op": "GenRandomInt", "target": "C"},

    {"actor": "Coordinator", "op": "Receive", "target": "A", "params": {"source": "RubyVM"}},
    {"actor": "Coordinator", "op": "Receive", "target": "B", "params": {"source": "BrainVM"}},
    {"actor": "Coordinator", "op": "Receive", "target": "C", "params": {"source": "GPU"}},

    {"actor": "RubyVM", "op": "Call", "target": "+", "params": {
      "lhs_register": "A",
      "rhs_register": "B"
    }},
    {"actor": "GPU", "op": "Call", "target": "*", "params": {
      "lhs_register": "sum",
      "rhs_register": "C"
    }}
  ]
}
```

Three substrates, one computation!

## Commands

```bash
# Single substrate
ucl run program.json --target ruby
ucl brain program.json --verbose
ucl brain program.json --production

# Universal (works on any)
ucl run universal.json --target ruby
ucl brain universal.json

# Parallel (multiple substrates)
ucl parallel parallel_program.json --verbose
```

## Philosophy

### Single Substrate
"I have a computer, let me use it"

### Universal
"I have multiple computers (including my brain), let me run the same program on any of them"

### Parallel
"I have multiple DIFFERENT types of computers, let me use them ALL AT ONCE to solve one problem"

**This is the future of computing: Heterogeneous parallel execution across silicon, software, and wetware.** 🚀


