# AI Layer: Code Generation as a UCL Operation

## The Meta-Recursion Insight

**UCL can encode the AI that generates UCL.**

This creates perfect recursion at every layer of abstraction:

```
UCL Program (instruction to AI)
  ↓
AI generates UCL (function definition)
  ↓
Execute UCL (compute result)
  ↓
Result
```

Every step is:
- **Traceable** - See exactly what the AI generated
- **Mockable** - Swap real AI for deterministic mock
- **Testable** - Assert at each abstraction layer
- **Composable** - Mix and match components

## Architecture

### Mock LLM as a Substrate

The Mock AI is just another execution environment:

```rust
pub struct MockAIState {
    knowledge_base: HashMap<String, String>,  // instruction → UCL code
    prompts: Vec<String>,                     // History of requests
    responses: Vec<String>,                   // History of outputs
    generated_code: HashMap<String, Vec<Action>>,  // Generated UCL
    model_name: String,                       // "MockLLM-UCL-v1"
    temperature: f64,                         // Always 0.0 (deterministic)
}
```

### Supported Operations

- **Generate** - AI receives instruction, generates UCL code
- **Parse** - Parse code into executable form
- **Execute** - Execute generated code

## Usage

### Basic AI Code Generation

```bash
# AI generates factorial function
ucl ai examples/ai_generate_factorial.json --verbose
```

**Input (examples/ai_generate_factorial.json):**
```json
{
  "actions": [
    {
      "actor": "ai_agent",
      "op": "Generate",
      "target": "factorial_code",
      "params": {
        "instruction": "Write a recursive function to calculate factorial",
        "format": "ucl"
      }
    }
  ]
}
```

**Output:**
```
🧠🤖 Simulating AI code generation (Mock LLM)...

Step 1: Generate - ai_agent → factorial_code
  💭 Received instruction: "Write a recursive function to calculate factorial"
  🧠 Matched knowledge: "factorial"
  ✨ Generating UCL code...
  ✅ Generated 1 UCL actions

=== Generated Code ===

DefineFunction(factorial, args: [n],
  body: [
    If(n <= 1, then: Return(1),
    else: Return(n * factorial(n-1)))
  ]
)
```

## The Knowledge Base

The Mock AI has a **deterministic knowledge base** mapping instructions to UCL:

### Factorial
**Instruction keywords:** "factorial"

**Generates:**
```json
{
  "op": "DefineFunction",
  "target": "factorial",
  "params": {
    "args": ["n"],
    "body": [
      {
        "op": "If",
        "condition": {"type": "comparison", "op": "<=", "left": {"var": "n"}, "right": 1},
        "then": [{"op": "Return", "params": {"value": 1}}],
        "else": [{"op": "Return", "params": {"value": {
          "expr": {"op": "*", "left": {"var": "n"},
                   "right": {"call": "factorial", "args": {"n": {"expr": {"op": "-", "left": {"var": "n"}, "right": 1}}}}}
        }}}]
      }
    ]
  }
}
```

### Fibonacci
**Instruction keywords:** "fibonacci"

Generates the recursive Fibonacci function (see `examples/fibonacci.json`)

### Hello World
**Instruction keywords:** "hello world"

**Generates:**
```json
{
  "op": "Emit",
  "target": "output",
  "params": {"content": "Hello, World!"}
}
```

## Mockable Abstraction Layers

The power of this approach is **every component can be mocked**:

### Test Scenario 1: Mock AI + Mock VM
```json
{
  "actions": [
    {"actor": "mock_ai", "op": "Generate", "params": {"instruction": "factorial"}},
    {"actor": "mock_vm", "op": "Execute", "params": {"code": "factorial_code"}}
  ]
}
```

Both components are deterministic, fast, and fully traceable.

### Test Scenario 2: Mock AI + Real Ruby VM
```json
{
  "actions": [
    {"actor": "mock_ai", "op": "Generate", "params": {"instruction": "factorial"}},
    {"actor": "ruby_compiler", "op": "Compile", "params": {"code": "factorial_code"}},
    {"actor": "ruby_vm", "op": "Execute", "params": {"code": "compiled_ruby"}}
  ]
}
```

Mock AI generates code, real Ruby VM executes it.

### Test Scenario 3: Real AI + Mock VM
```json
{
  "actions": [
    {"actor": "gpt4", "op": "Generate", "params": {"instruction": "factorial", "api_key": "..."}},
    {"actor": "mock_vm", "op": "Execute", "params": {"code": "factorial_code"}}
  ]
}
```

Real AI generates code, mock VM "executes" it for testing.

### Test Scenario 4: Real AI + Real VM
Full production stack - everything is real.

## Pure Causal Unit Testing

This enables unprecedented testing granularity:

```json
{
  "test": "AI generates factorial correctly",
  "actions": [
    {"actor": "ai", "op": "Generate", "target": "code",
     "params": {"instruction": "factorial"}},

    {"actor": "test", "op": "Assert", "target": "code",
     "params": {"contains": "DefineFunction"}},

    {"actor": "test", "op": "Assert", "target": "code",
     "params": {"contains": "factorial"}},

    {"actor": "vm", "op": "Execute", "target": "code",
     "params": {"n": 5}},

    {"actor": "test", "op": "Assert", "target": "result",
     "params": {"equals": 120}}
  ]
}
```

Every step is verified:
1. ✓ AI generated code
2. ✓ Code contains DefineFunction
3. ✓ Code defines factorial
4. ✓ Code executes
5. ✓ Result is correct

## The Full Chain

Here's what happens when you run `examples/ai_chain.json`:

```
1. Developer: "I need a factorial function"
   ↓
2. AI Agent: Generate(factorial_function)
   → Matches "factorial" in knowledge base
   → Outputs UCL DefineFunction
   ↓
3. Compiler: Compile(factorial_code → Ruby)
   → Generates Ruby def statement
   ↓
4. Ruby VM: Execute(factorial(5))
   → Computes result: 120
   ↓
5. Tester: Assert(result == 120)
   → ✅ PASS
```

Every layer is encoded in UCL!

## Why This Matters

### 1. AI Development Becomes Testable

Current AI development:
```
Prompt → AI (black box) → Code → Hope it works
```

UCL AI development:
```
UCL Instruction → Mock AI (traceable) → UCL Code → Verify → Replace with Real AI
```

### 2. Deterministic During Development

Use Mock AI with known outputs to test:
- Code compilation
- Execution correctness
- Error handling
- Edge cases

Then swap in real AI for production.

### 3. Cross-Substrate AI

The AI generates **UCL**, which can run on:
- Ruby VM
- Brain VM (human learns the skill)
- Robot VM (robot executes procedure)

One AI, many execution targets!

### 4. AI Generating AI

Since AI output is UCL, an AI can generate:
- Functions
- Control flow
- **Entire programs**
- **Other AI definitions!**

```json
{
  "actor": "meta_ai",
  "op": "Generate",
  "target": "specialist_ai",
  "params": {
    "instruction": "Create an AI that specializes in sorting algorithms"
  }
}
```

Infinite meta-levels! 🪆

## Extending the Knowledge Base

Add new patterns to Mock AI:

```rust
knowledge_base.insert(
    "quicksort".to_string(),
    r#"[
  {
    "op": "DefineFunction",
    "target": "quicksort",
    "params": {
      "args": ["arr"],
      "body": [...]
    }
  }
]"#
);
```

Or create domain-specific AI agents:

- **BiologyAI** - Generates UCL for biological processes
- **LegalAI** - Generates UCL for contracts
- **RecipeAI** - Generates UCL for cooking procedures

## Future: Real AI Integration

Replace Mock AI with real LLM:

```json
{
  "actor": "gpt4",
  "op": "Generate",
  "target": "code",
  "params": {
    "instruction": "...",
    "api_key": "sk-...",
    "model": "gpt-4",
    "system_prompt": "You are a UCL code generator. Output valid UCL JSON."
  }
}
```

Same interface, different implementation!

## Try It

```bash
# Mock AI generates factorial
ucl ai examples/ai_generate_factorial.json --verbose

# See the full chain
ucl ai examples/ai_chain.json --verbose

# View generated code
ucl ai examples/ai_generate_factorial.json | grep -A 50 "Generated Code"
```

---

**AI is not special - it's just another substrate that transforms instructions into executable code.** And in UCL, that code is more UCL! 🤖✨

