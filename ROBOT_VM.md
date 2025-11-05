# Robot VM: Physical Operations as Programs

## Core Concept

**Recipes and physical instructions are programs that run on either a human brain or a robot.**

Just as natural language runs on the Brain VM, cooking recipes and physical procedures can run on:
- **Brain VM**: Simulates a human following instructions
- **Robot VM**: Simulates a robot executing physical operations

The Universal Causal Language (UCL) treats physical operations as executable instructions that modify robot state through actuator operations.

## Architecture

The Robot VM simulates key aspects of robotic execution:

### Robot State

```rust
pub struct RobotState {
    objects: HashMap<String, ObjectState>,     // 3D object positions
    arm_position: (f64, f64, f64),            // Robot arm coordinates
    gripper: Option<String>,                   // What's being held
    temperatures: HashMap<String, f64>,        // Temperature sensors
    log: Vec<String>,                          // Execution log
    errors: Vec<String>,                       // Error tracking
}
```

### Supported Physical Operations

The Robot VM implements these "opcodes" for physical manipulation:

- **Gather** - Collect and initialize objects
- **Measure** - Measure quantities
- **Heat** - Apply thermal energy
- **Pour** - Transfer liquids
- **Mix** - Combine ingredients
- **Stir** - Agitate contents
- **Place** - Position objects
- **Remove** - Extract objects
- **Steep** - Infusion process
- **Serve** - Presentation

## Usage

### Basic Execution

```bash
# Run a recipe on the robot VM
ucl robot examples/recipe_tea.json

# With verbose output showing each operation
ucl robot examples/recipe_tea.json --verbose

# Compare: Same recipe on brain VM
ucl brain examples/recipe_tea.json --verbose
```

### Example Output

```
=== Robot State ===

Arm Position: (0.00, 0.00, 0.00)
Gripper: Empty

Objects:
  tea_bag - pos:(0.0, 0.0, 0.0), temp:20°C, state:ready
  kettle - pos:(0.0, 0.0, 0.0), temp:100°C, state:boiling

Execution Log:
  1. Gathered items for ingredients
  2. Heated water to 100°C
  3. Poured water from kettle into cup (250ml)
  4. Steeping tea_bag for 3-5 minutes
  5. Serving tea
```

## Recipe as Universal Code

### Traditional View
```
"Heat water to 100°C" → [Text Instructions] → Human interprets → ???
```

### UCL Robot VM View
```
"Heat water to 100°C" → UCL Program → Robot VM → Physical Actions
```

The instruction becomes:
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

Which executes as:
1. Identify water container (kettle)
2. Apply heat to kettle
3. Monitor temperature sensor
4. Wait until temperature reaches 100°C
5. Stop heating when condition met

## Modular Substrate Independence

The power of UCL is that components are **mix-and-match**:

### 1. Mock AI + Mock Robot
Both components simulated (current implementation)
```bash
ucl robot examples/recipe_tea.json
```

### 2. Real Human Brain + Mock Robot
Human executes on brain VM, robot simulated
```bash
ucl brain examples/recipe_tea.json
```

### 3. Mock AI + Real Robot
Simulated AI with real robotic arms (pre-programmed operations)
```bash
# Future: Connect to real robot controller
ucl robot examples/recipe_tea.json --hardware real
```

### 4. Real AI (LLM) + Mock Robot
Real LLM generates instructions, robot simulated
```bash
# Future: LLM interprets natural language → UCL → Mock robot
echo "Make me a cup of tea" | ucl ai-brain --target robot
```

### 5. Real AI (LLM) + Real Robot
Full production system
```bash
# Future: LLM + Real robotic system
echo "Make me a cup of tea" | ucl ai-brain --target robot --hardware real
```

## Implications

### 1. Instructions as Executable Code

Physical instructions aren't just descriptions—they're **programs that execute**:

```
Recipe: "Pour water from kettle into cup"
↓
UCL Action: Pour(water, from: kettle, into: cup)
↓
Robot Execution: actuate_arm(kettle) → tilt(45°) → monitor_flow() → stop()
```

### 2. Cross-Substrate Recipes

Since recipes are just programs, they can run on:
- **Human Brain**: Person following recipe
- **Robot**: Automated cooking
- **Hybrid**: Human supervises robot

The same causal logic, different execution environments.

### 3. Testable Cooking

Every step has observable state changes:

```
Heat(water) → state.temperatures["kettle"] = 100.0
Pour(water) → state.objects["cup"].contains = "water"
Steep(tea) → state.objects["tea_bag"].state = "infusing"
```

### 4. AGI + Robotics Training

Instead of training on images or videos, train on **executable physical programs**:
- Learn operations (physical opcodes)
- Learn how operations compose
- Learn effects on object states
- Generate programs that achieve desired outcomes

## Philosophical Implications

### Physical Actions as Computation

Physical manipulation is the **computation**, not the mechanism. A human hand and a robotic gripper performing the same UCL Pour() operation are executing the same computation.

### Recipes as Universal API

Recipes become an API to any physical system:
```
Human Kitchen API: recipe → cooked_food
Robot Kitchen API: recipe → cooked_food
```

Same interface, different implementation.

### Skill Transfer

A recipe learned/developed on one substrate (human) transfers directly to another (robot). No retraining needed—just different interpreters.

## Future Directions

1. **Richer Robot State**
   - Force/torque sensors
   - Vision system integration
   - Tactile feedback
   - Real-time physics simulation

2. **More Operations**
   - Chop (cutting operations)
   - Blend (blending/mixing)
   - Bake (oven operations)
   - Season (flavor operations)

3. **Safety Constraints**
   - Collision detection
   - Temperature limits
   - Speed limits
   - Emergency stop

4. **Multi-Robot Coordination**
   - Parallel cooking operations
   - Handoff between robots
   - Shared workspace management

## Try It Yourself

```bash
# Run the demo showing both substrates
./demo_recipe.sh

# Or manually compare:
ucl brain examples/recipe_tea.json --verbose
ucl robot examples/recipe_tea.json --verbose
```

Watch as the same recipe executes on both a simulated human brain and a simulated robot. Same causal logic, different execution environments. 🍵🤖

