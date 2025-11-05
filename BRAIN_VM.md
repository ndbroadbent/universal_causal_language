# Brain VM: Language as a Program Running on the Human Brain

## Core Concept

**Language is a program that runs on a human brain substrate.**

Just as Ruby code runs on a Ruby VM and machine code runs on a CPU, natural language runs on the human brain. The Universal Causal Language (UCL) brain simulator treats language as executable instructions that modify brain state through cognitive operations.

## Architecture

The brain VM simulates key aspects of human cognition:

### Brain State

```rust
pub struct BrainState {
    beliefs: HashMap<String, Value>,     // Long-term memory/knowledge
    emotions: HashMap<String, f64>,      // Emotional state (0.0-1.0)
    working_memory: Vec<String>,         // Short-term memory (limited capacity)
    attention: Option<String>,           // Current focus
    output: Vec<String>,                 // Speech/expression buffer
    thoughts: Vec<String>,               // Internal narrative
    goals: Vec<String>,                  // Active intentions
    trace: Vec<String>,                  // Execution history
}
```

### Supported Cognitive Operations

The brain VM implements these "opcodes" that language can execute:

- **StoreFact** - Store information in memory
- **Assert** - Establish a strong belief
- **Emit** - Generate speech/output
- **Receive** - Process input/perception
- **Measure** - Observe something
- **Decide** - Make a decision
- **Read** - Retrieve from memory
- **Write** - Update memory
- **Create** - Form a new concept
- **Bind** - Associate concept with value
- **Oblige** - Create an obligation/goal
- **Wait** - Simulate time passing

### Unknown Operations

When the brain encounters an operation it doesn't understand (like `Transcribe`, `Delete`, or `Call`), it responds naturally:

```
💭 "Sorry, I don't know what that means: Transcribe"
🗣️  "I'm not sure what you mean..."
```

This increases:

- **Confusion** emotion (+0.4)
- **Curiosity** emotion (+0.3)

This simulates how real brains handle incomprehensible input!

## Usage

### Basic Execution

```bash
# Run natural language as a program on the brain VM
ucl brain examples/natural_language.json

# With verbose output showing each cognitive operation
ucl brain examples/natural_language.json --verbose

# Also works with the run command
ucl run examples/natural_language.json --target brain
```

### Example Output

```
=== Brain State ===

Beliefs:
  cat.color = "black"
  dog.location = "yard"

Emotional State:
  warmth: 0.30
  responsibility: 0.50

Working Memory:
  - The cat has properties: color
  - The dog has properties: location

Active Goals:
  → Must: clean_room

Internal Thoughts:
  💭 I believe that: Water boils at 100°C

Output/Speech:
  🗣️  Hello, world!
```

## Natural Language as Code

### Traditional View

```
"The cat is black"  →  [Text String]  →  Statistical Model  →  ???
```

### UCL Brain VM View

```
"The cat is black"  →  UCL Program  →  Brain VM  →  Updated Brain State
```

The sentence becomes:

```json
{
  "actor": "listener",
  "op": "StoreFact",
  "target": "memory",
  "params": { "entity": "cat", "color": "black" }
}
```

Which executes as:

1. Access memory subsystem (target)
2. Store entity and properties: `cat.color = "black"`
3. Add to working memory
4. Update emotional state (if relevant)

## Implications

### 1. Language Understanding as Execution

Understanding isn't recognition—it's **successful execution**. If the brain can't execute an instruction (unknown opcode), it doesn't understand it.

### 2. Cross-Substrate Compilation

Since language is just a program, we can:

- **Compile UCL → Ruby**: Execute on Ruby VM
- **Compile UCL → Brain**: Execute on brain VM
- **Compile UCL → Rust**: Execute on native code

The same causal logic, different execution environments.

### 3. Explainable Cognition

Every thought, emotion, and utterance traces back to specific operations:

```
StoreFact(target: memory, entity: cat) → beliefs["cat.color"] = "black"
Emit(target: speech) → output["Hello!"], emotions["warmth"] += 0.3
Oblige(target: goals) → goals["clean_room"], emotions["responsibility"] += 0.5
```

### 4. AGI Training

Instead of training on text, train on **executable brain programs**:

- Learn operations (opcodes)
- Learn how operations compose
- Learn effects on brain state
- Generate programs that achieve desired states

## Philosophical Implications

### Substrate Independence

Cognition is the **computation**, not the substrate. A biological brain and a silicon brain running the same UCL program are performing the same cognition.

### Language as Universal API

Language becomes an API to any cognitive system:

```
Human Brain API: natural_language → brain_state
AI Brain API: natural_language → brain_state
```

Same interface, different implementation.

### Consciousness as Execution Context

The subjective experience might be the **what it's like** to be the execution environment for these programs. Different substrates feel different because they execute differently, even with identical programs.

## Future Directions

1. **Richer Brain State**

   - Episodic memory
   - Procedural memory
   - Multiple attention streams
   - Complex emotion models

2. **More Operations**

   - Imagine (simulate counterfactuals)
   - Reason (logical inference)
   - Feel (process emotions)
   - Dream (offline processing)

3. **Learning**

   - Adapt operation implementations based on experience
   - Form new compound operations
   - Optimize execution patterns

4. **Multi-Brain Systems**
   - Conversation as message passing between brain VMs
   - Shared beliefs (culture)
   - Collective goals (society)

## Try It Yourself

```bash
# Create a simple language program
cat > my_language.json << 'EOF'
{
  "actions": [
    {"actor": "me", "op": "StoreFact", "target": "memory",
     "params": {"entity": "UCL", "property": "mind-blowing"}},
    {"actor": "me", "op": "Emit", "target": "speech",
     "params": {"content": "This changes everything!"}}
  ]
}
EOF

# Execute it on the brain VM
ucl brain my_language.json --verbose
```

Watch as language literally executes, step by step, on a simulated human brain. 🧠✨
