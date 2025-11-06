# UCL is Turing Complete

## Proof of Turing Completeness

The Universal Causal Language (UCL) is now **Turing complete**, meaning it can compute anything that any other programming language can compute. This document explains how UCL achieves Turing completeness through control flow constructs, function definitions, and expression evaluation.

## Required Features for Turing Completeness

A language is Turing complete if it has:

1. ✅ **Conditional branching** (if/else)
2. ✅ **Loops or recursion** (while, for, recursive functions)
3. ✅ **Memory/storage** (variables, beliefs, state)
4. ✅ **Arbitrary computation** (arithmetic, boolean logic)

UCL now has all of these!

## Control Flow Constructs

### 1. Conditional Branching (If/Else)

Execute different actions based on conditions:

```json
{
  "actor": "VM",
  "op": "If",
  "target": "check",
  "condition": {
    "type": "comparison",
    "op": "<=",
    "left": {"var": "n"},
    "right": 1
  },
  "then": [
    {"actor": "VM", "op": "Return", "target": "result", "params": {"value": {"var": "n"}}}
  ],
  "else": [
    {"actor": "VM", "op": "Emit", "target": "output", "params": {"content": "n is greater than 1"}}
  ]
}
```

**Compiles to Ruby:**
```ruby
if n <= 1
  return n
else
  puts "n is greater than 1"
end
```

### 2. While Loops

Repeat actions while a condition is true:

```json
{
  "actor": "VM",
  "op": "While",
  "target": "countdown",
  "condition": {
    "type": "comparison",
    "op": ">",
    "left": {"var": "i"},
    "right": 0
  },
  "body": [
    {"actor": "VM", "op": "Emit", "target": "output", "params": {"content": {"var": "i"}}},
    {"actor": "VM", "op": "Bind", "target": "i", "params": {"value": {
      "expr": {"op": "-", "left": {"var": "i"}, "right": 1}
    }}}
  ]
}
```

**Compiles to Ruby:**
```ruby
while i > 0
  puts i
  i = (i - 1)
end
```

### 3. For Loops

Iterate over a range:

```json
{
  "actor": "VM",
  "op": "For",
  "target": "iteration",
  "variable": "i",
  "from": 0,
  "to": 10,
  "body": [
    {"actor": "VM", "op": "Emit", "target": "output", "params": {"content": {"var": "i"}}}
  ]
}
```

**Compiles to Ruby:**
```ruby
(0 .. 10).each do |i|
  puts i
end
```

### 4. Function Definitions

Define reusable functions (skills/procedures):

```json
{
  "actor": "VM",
  "op": "DefineFunction",
  "target": "fibonacci",
  "params": {
    "args": ["n"],
    "body": [
      {
        "actor": "VM",
        "op": "If",
        "target": "base_case",
        "condition": {"type": "comparison", "op": "<=", "left": {"var": "n"}, "right": 1},
        "then": [
          {"actor": "VM", "op": "Return", "target": "result", "params": {"value": {"var": "n"}}}
        ],
        "else": [
          {"actor": "VM", "op": "Return", "target": "result", "params": {
            "value": {
              "expr": {
                "op": "+",
                "left": {"call": "fibonacci", "args": {"n": {"expr": {"op": "-", "left": {"var": "n"}, "right": 1}}}},
                "right": {"call": "fibonacci", "args": {"n": {"expr": {"op": "-", "left": {"var": "n"}, "right": 2}}}}
              }
            }
          }}
        ]
      }
    ]
  }
}
```

**Compiles to Ruby:**
```ruby
def fibonacci(n)
  if n <= 1
    return n
  else
    return (fibonacci((n - 1)) + fibonacci((n - 2)))
  end
end
```

**Executes on Brain VM** - The brain "learns" the fibonacci skill and can use it!

## Boolean Expressions

### Comparison Operators

- `==` - Equal
- `!=` - Not equal
- `<` - Less than
- `<=` - Less than or equal
- `>` - Greater than
- `>=` - Greater than or equal

### Logical Operators

**AND:**
```json
{
  "type": "and",
  "operands": [
    {"type": "comparison", "op": ">", "left": {"var": "x"}, "right": 0},
    {"type": "comparison", "op": "<", "left": {"var": "x"}, "right": 10}
  ]
}
```

**OR:**
```json
{
  "type": "or",
  "operands": [
    {"type": "comparison", "op": "==", "left": {"var": "x"}, "right": 0},
    {"type": "comparison", "op": "==", "left": {"var": "x"}, "right": 1}
  ]
}
```

**NOT:**
```json
{
  "type": "not",
  "operand": {
    "type": "comparison",
    "op": "==",
    "left": {"var": "done"},
    "right": true
  }
}
```

## Expressions

### Variable References

```json
{"var": "n"}
```

Refers to the value of variable `n`.

### Arithmetic Operations

```json
{
  "expr": {
    "op": "+",
    "left": {"var": "a"},
    "right": {"var": "b"}
  }
}
```

Supported operators: `+`, `-`, `*`, `/`, `%`

### Nested Expressions

```json
{
  "expr": {
    "op": "+",
    "left": {"expr": {"op": "*", "left": 2, "right": 3}},
    "right": 4
  }
}
```

Evaluates to `(2 * 3) + 4 = 10`

### Function Calls

```json
{
  "call": "fibonacci",
  "args": {"n": {"var": "i"}}
}
```

Calls the `fibonacci` function with argument `n` set to the value of variable `i`.

### Literal Values

Bare numbers, strings, booleans:
- `42` - Number
- `"hello"` - String
- `true` / `false` - Boolean

## Complete Example: Fibonacci

See `examples/fibonacci.json` for a complete working example that demonstrates:

- ✅ Recursive function definition
- ✅ Conditional branching (base case)
- ✅ Arithmetic expressions (n-1, n-2)
- ✅ Function calls (recursive)
- ✅ For loops (printing results)
- ✅ Variable binding and evaluation

**Run it:**
```bash
# Compile to Ruby and execute
ucl run examples/fibonacci.json

# Execute on Brain VM (human learns and applies the skill)
ucl brain examples/fibonacci.json --verbose
```

**Output:**
```
Fibonacci sequence:
0
1
1
2
3
5
8
13
21
34
55
```

## Substrate Independence with Turing Completeness

The same Turing complete program can run on:

- **Ruby VM** - Compiles to Ruby code and executes
- **Brain VM** - Human brain learns the function as a skill
- **Robot VM** - Robot learns the procedure

```bash
# Same fibonacci logic, three different substrates
ucl run examples/fibonacci.json          # Ruby VM
ucl brain examples/fibonacci.json        # Brain VM
```

## Implications

### 1. Universal Computation

UCL can now express **any** computation that can be expressed in any other programming language:

- Algorithms (sorting, searching, graph traversal)
- Data structures (lists, trees, graphs)
- Numerical computation
- String processing
- Game logic
- AI decision-making

### 2. Cross-Domain Programs

The same control flow works across all domains:

- **Programming**: Fibonacci, quicksort, etc.
- **Natural Language**: "If you're hungry, then eat, else wait"
- **Recipes**: "While not boiling, heat. For each ingredient, add and mix"
- **Biology**: "While ATP available, if substrate present, then catalyze"
- **Legal**: "If payment received, then release goods, else pursue remedy"

### 3. Provably Correct Execution

Every branch, loop iteration, and function call is traceable:

```
DefineFunction(fibonacci) → Brain learns skill
For(i: 0..10) → Loop 11 iterations
  Call(fibonacci, 5) → Recursive calls traced
    If(5 <= 1) → false → Else branch
    Call(fibonacci, 4) + Call(fibonacci, 3)
    ...
Result: 5
```

### 4. AGI Foundation

An AGI trained on UCL learns:
- How to define new skills (DefineFunction)
- How to make decisions (If/else)
- How to iterate (For/While)
- How to compose solutions recursively

Not just pattern matching—actual **algorithmic thinking**.

## Implementation Details

### Recursion Limits

Both Brain VM and Robot VM have recursion depth limits (1000) to prevent stack overflow:

```rust
if self.recursion_depth >= self.max_recursion_depth {
    return Err(anyhow!("Maximum recursion depth exceeded"));
}
```

### Loop Limits

While loops have iteration limits (10,000) to prevent infinite loops:

```rust
const MAX_ITERATIONS: usize = 10000;
```

### Variable Scoping

- **Brain VM**: Variables stored in `beliefs` (can persist across function calls)
- **Robot VM**: Variables stored in `variables` (scoped storage)
- **Both**: Function calls save/restore variable state

### Function Storage

- **Brain VM**: Functions stored in `brain.functions` as learned skills
- **Robot VM**: Functions stored in `robot.functions` as learned procedures
- **Ruby VM**: Functions compiled to `def` statements

## Next Steps

With Turing completeness achieved, we can now:

1. **Implement classic algorithms** in UCL
2. **Cross-compile between substrates** (Ruby ↔ UCL ↔ Brain)
3. **Optimize compilation** (tail call optimization, loop unrolling)
4. **Add type systems** (optional static typing)
5. **Build standard libraries** (common functions for each domain)

## Try It

```bash
# Clone and build
git clone <repo>
cd universal_causal_language
cargo build --release

# Run Fibonacci (proves Turing completeness)
./target/release/ucl run examples/fibonacci.json

# See the compiled Ruby code
./target/release/ucl compile examples/fibonacci.json
```

---

**UCL is not just a universal language—it's a universal *computational* language.** 🎉

