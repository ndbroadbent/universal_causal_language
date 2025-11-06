use crate::{Action, Operation, Program, Condition, ComparisonOp, Expression};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Represents a learned function (skill) in the brain
#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub args: Vec<String>,
    pub body: Vec<crate::Action>,
}

/// Represents the state of a simulated human brain
#[derive(Debug, Clone)]
pub struct BrainState {
    /// Facts and beliefs stored in memory
    pub beliefs: HashMap<String, serde_json::Value>,

    /// Emotional state
    pub emotions: HashMap<String, f64>,

    /// Working memory (short-term)
    pub working_memory: Vec<String>,

    /// Attention focus
    pub attention: Option<String>,

    /// Output buffer (what the brain wants to express)
    pub output: Vec<String>,

    /// Internal narrative/thoughts
    pub thoughts: Vec<String>,

    /// Goals and intentions
    pub goals: Vec<String>,

    /// Execution trace for debugging
    pub trace: Vec<String>,

    /// Learned functions (skills/procedures)
    pub functions: HashMap<String, FunctionDef>,
}

impl BrainState {
    pub fn new() -> Self {
        Self {
            beliefs: HashMap::new(),
            emotions: HashMap::new(),
            working_memory: Vec::new(),
            attention: None,
            output: Vec::new(),
            thoughts: Vec::new(),
            goals: Vec::new(),
            trace: Vec::new(),
            functions: HashMap::new(),
        }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str("=== Brain State ===\n\n");

        if !self.beliefs.is_empty() {
            output.push_str("Beliefs:\n");
            for (key, value) in &self.beliefs {
                output.push_str(&format!("  {} = {}\n", key, value));
            }
            output.push('\n');
        }

        if !self.emotions.is_empty() {
            output.push_str("Emotional State:\n");
            for (emotion, intensity) in &self.emotions {
                output.push_str(&format!("  {}: {:.2}\n", emotion, intensity));
            }
            output.push('\n');
        }

        if !self.working_memory.is_empty() {
            output.push_str("Working Memory:\n");
            for item in &self.working_memory {
                output.push_str(&format!("  - {}\n", item));
            }
            output.push('\n');
        }

        if let Some(focus) = &self.attention {
            output.push_str(&format!("Current Focus: {}\n\n", focus));
        }

        if !self.goals.is_empty() {
            output.push_str("Active Goals:\n");
            for goal in &self.goals {
                output.push_str(&format!("  → {}\n", goal));
            }
            output.push('\n');
        }

        if !self.thoughts.is_empty() {
            output.push_str("Internal Thoughts:\n");
            for thought in &self.thoughts {
                output.push_str(&format!("  💭 {}\n", thought));
            }
            output.push('\n');
        }

        if !self.output.is_empty() {
            output.push_str("Output/Speech:\n");
            for text in &self.output {
                output.push_str(&format!("  🗣️  {}\n", text));
            }
            output.push('\n');
        }

        output
    }
}

impl Default for BrainState {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulates a human brain as a VM that executes language programs
pub struct BrainSimulator {
    state: BrainState,
    verbose: bool,
    recursion_depth: usize,
    max_recursion_depth: usize,
}

impl BrainSimulator {
    pub fn new() -> Self {
        Self {
            state: BrainState::new(),
            verbose: false,
            recursion_depth: 0,
            max_recursion_depth: 1000,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn state(&self) -> &BrainState {
        &self.state
    }

    pub fn execute(&mut self, program: &Program) -> Result<()> {
        if self.verbose {
            println!("🧠 Starting brain simulation...\n");
        }

        for (i, action) in program.actions.iter().enumerate() {
            if self.verbose {
                println!("Step {}: {:?} - {} → {}",
                    i + 1, action.op, action.actor, action.target);
            }

            self.execute_action(action)?;

            if self.verbose {
                println!();
            }
        }

        Ok(())
    }

    fn execute_action(&mut self, action: &Action) -> Result<()> {
        // Check recursion depth
        if self.recursion_depth >= self.max_recursion_depth {
            return Err(anyhow!("Maximum recursion depth exceeded"));
        }

        let trace_msg = format!("{:?}({})", action.op, action.target);
        self.state.trace.push(trace_msg);

        match &action.op {
            Operation::StoreFact => self.store_fact(action),
            Operation::Assert => self.assert_fact(action),
            Operation::Emit => self.emit(action),
            Operation::Receive => self.receive(action),
            Operation::Measure => self.measure(action),
            Operation::Decide => self.decide(action),
            Operation::Read => self.read(action),
            Operation::Write => self.write_memory(action),
            Operation::Create => self.create_concept(action),
            Operation::Bind => self.bind_concept(action),
            Operation::Oblige => self.create_obligation(action),
            Operation::Wait => self.wait(action),
            Operation::GenRandomInt => self.gen_random_int(action),

            // Control flow operations
            Operation::If => self.execute_if(action),
            Operation::While => self.execute_while(action),
            Operation::For => self.execute_for(action),
            Operation::DefineFunction => self.execute_define_function(action),

            // Cooking operations - simulated as physical actions
            Operation::Gather => self.physical_action(action, "👐", "Gathering"),
            Operation::Heat => self.physical_action(action, "🔥", "Heating"),
            Operation::Pour => self.physical_action(action, "🫗", "Pouring"),
            Operation::Mix => self.physical_action(action, "🥄", "Mixing"),
            Operation::Stir => self.physical_action(action, "🥄", "Stirring"),
            Operation::Place => self.physical_action(action, "📍", "Placing"),
            Operation::Remove => self.physical_action(action, "✋", "Removing"),
            Operation::Steep => self.physical_action(action, "⏱️", "Steeping"),
            Operation::Serve => self.physical_action(action, "🍽️", "Serving"),

            _ => {
                // Brain encounters something it doesn't understand
                let confusion = format!("Sorry, I don't know what that means: {:?}", action.op);
                self.state.thoughts.push(confusion.clone());
                self.state.output.push("I'm not sure what you mean...".to_string());

                // Encountering unknown concepts creates mild confusion/curiosity
                *self.state.emotions.entry("confusion".to_string()).or_insert(0.0) += 0.4;
                *self.state.emotions.entry("curiosity".to_string()).or_insert(0.0) += 0.3;

                if self.verbose {
                    println!("  🤔 {}", confusion);
                    println!("  🗣️  \"I'm not sure what you mean...\"");
                }

                Ok(())
            }
        }
    }

    fn store_fact(&mut self, action: &Action) -> Result<()> {
        // Store a fact in belief memory
        if let Some(params) = &action.params {
            // Extract the entity from params (new structure) or fall back to target (old structure)
            let entity = params.get("entity")
                .and_then(|v| v.as_str())
                .unwrap_or(&action.target);

            // Filter out "entity" from properties to store
            let properties: HashMap<String, serde_json::Value> = params
                .iter()
                .filter(|(k, _)| k.as_str() != "entity")
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            for (key, value) in &properties {
                let fact_key = format!("{}.{}", entity, key);
                self.state.beliefs.insert(fact_key.clone(), value.clone());

                if self.verbose {
                    println!("  📝 Stored: {} = {}", fact_key, value);
                }
            }

            // Update working memory
            if !properties.is_empty() {
                let memory_item = format!("The {} has properties: {}",
                    entity,
                    properties.keys().map(|k| k.as_str()).collect::<Vec<_>>().join(", "));
                self.state.working_memory.push(memory_item);

                // Keep working memory limited
                if self.state.working_memory.len() > 7 {
                    self.state.working_memory.remove(0);
                }
            }
        }
        Ok(())
    }

    fn assert_fact(&mut self, action: &Action) -> Result<()> {
        // Assert a fact (stronger than store - becomes a belief)
        let statement = action.params
            .as_ref()
            .and_then(|p| p.get("statement"))
            .and_then(|v| v.as_str())
            .unwrap_or(&action.target);

        self.state.beliefs.insert(
            format!("assertion.{}", action.target),
            serde_json::json!(statement)
        );

        self.state.thoughts.push(format!("I believe that: {}", statement));

        if self.verbose {
            println!("  ✓ Asserted: {}", statement);
        }

        Ok(())
    }

    fn emit(&mut self, action: &Action) -> Result<()> {
        // Generate output (speech/expression)
        let message = if let Some(params) = action.params.as_ref() {
            if let Some(content) = params.get("content") {
                // If content is a string matching a variable, output the variable's value
                if let Some(content_str) = content.as_str() {
                    if let Some(value) = self.state.beliefs.get(content_str) {
                        value.to_string()
                    } else {
                        content_str.to_string()
                    }
                } else {
                    content.to_string()
                }
            } else if let Some(message) = params.get("message") {
                message.as_str().map(|s| s.to_string()).unwrap_or_else(|| message.to_string())
            } else {
                action.target.clone()
            }
        } else {
            // No params - check if target is a variable
            if let Some(value) = self.state.beliefs.get(&action.target) {
                value.to_string()
            } else {
                action.target.clone()
            }
        };

        self.state.output.push(message.clone());

        // Check for emotional content
        if let Some(params) = &action.params {
            if let Some(intent) = params.get("intent").and_then(|v| v.as_str()) {
                if intent == "greeting" {
                    *self.state.emotions.entry("warmth".to_string()).or_insert(0.0) += 0.3;
                }
            }
        }

        if self.verbose {
            println!("  🗣️  Output: \"{}\"", message);
        }

        Ok(())
    }

    fn receive(&mut self, action: &Action) -> Result<()> {
        // Receive input (perception)
        let input = action.params
            .as_ref()
            .and_then(|p| p.get("content"))
            .and_then(|v| v.as_str())
            .unwrap_or(&action.target);

        self.state.working_memory.push(format!("Heard: {}", input));
        self.state.attention = Some(input.to_string());

        if self.verbose {
            println!("  👂 Received: \"{}\"", input);
        }

        Ok(())
    }

    fn measure(&mut self, action: &Action) -> Result<()> {
        // Observe/measure something
        self.state.attention = Some(action.target.clone());

        if let Some(params) = &action.params {
            for (key, value) in params {
                let obs_key = format!("observed.{}.{}", action.target, key);
                self.state.beliefs.insert(obs_key, value.clone());
            }
        }

        if self.verbose {
            println!("  👁️  Observing: {}", action.target);
        }

        Ok(())
    }

    fn decide(&mut self, action: &Action) -> Result<()> {
        // Make a decision
        let decision = action.params
            .as_ref()
            .and_then(|p| p.get("choice").or_else(|| p.get("decision")))
            .and_then(|v| v.as_str())
            .unwrap_or(&action.target);

        self.state.thoughts.push(format!("Decided to: {}", decision));

        // Decisions often become goals
        if let Some(params) = &action.params {
            if let Some(goal) = params.get("goal").and_then(|v| v.as_str()) {
                self.state.goals.push(goal.to_string());
            }
        }

        if self.verbose {
            println!("  🤔 Decision: {}", decision);
        }

        Ok(())
    }

    fn read(&mut self, action: &Action) -> Result<()> {
        // Read from memory
        let value = self.state.beliefs.get(&action.target);

        if let Some(v) = value {
            self.state.working_memory.push(format!("Recalled: {} = {}", action.target, v));

            if self.verbose {
                println!("  📖 Recalled: {} = {}", action.target, v);
            }
        } else if self.verbose {
            println!("  ❓ No memory of: {}", action.target);
        }

        Ok(())
    }

    fn write_memory(&mut self, action: &Action) -> Result<()> {
        // Write to memory
        if let Some(params) = &action.params {
            // Check if it's a computed value
            if let Some(op) = params.get("operation") {
                let operation = op.as_str().unwrap_or("");

                // Get left operand (register or value)
                let lhs_val = if let Some(lhs_reg) = params.get("lhs_register") {
                    self.state.beliefs.get(lhs_reg.as_str().unwrap_or(""))
                        .and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
                        .unwrap_or(0.0)
                } else if let Some(lhs) = params.get("lhs") {
                    lhs.as_f64().or_else(|| lhs.as_i64().map(|i| i as f64)).unwrap_or(0.0)
                } else {
                    0.0
                };

                // Get right operand (register or value)
                let rhs_val = if let Some(rhs_reg) = params.get("rhs_register") {
                    self.state.beliefs.get(rhs_reg.as_str().unwrap_or(""))
                        .and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
                        .unwrap_or(0.0)
                } else if let Some(rhs) = params.get("rhs") {
                    rhs.as_f64().or_else(|| rhs.as_i64().map(|i| i as f64)).unwrap_or(0.0)
                } else {
                    0.0
                };

                let result = match operation {
                    "multiply" => lhs_val * rhs_val,
                    "add" => lhs_val + rhs_val,
                    "subtract" => lhs_val - rhs_val,
                    "divide" => if rhs_val != 0.0 { lhs_val / rhs_val } else { 0.0 },
                    _ => lhs_val * rhs_val,
                };

                self.state.beliefs.insert(action.target.clone(), serde_json::json!(result));
                self.state.thoughts.push(format!("Calculated: {} = {} {} {} = {}",
                    action.target, lhs_val,
                    match operation { "multiply" => "×", "add" => "+", "subtract" => "-", "divide" => "÷", _ => "×" },
                    rhs_val, result));

                if self.verbose {
                    println!("  🧮 Calculated: {} = {}", action.target, result);
                }

                return Ok(());
            }

            // Otherwise use direct value
            if let Some(value) = params.get("value") {
                self.state.beliefs.insert(action.target.clone(), value.clone());

                if self.verbose {
                    println!("  💾 Stored: {} = {}", action.target, value);
                }
            }
        }

        Ok(())
    }

    fn create_concept(&mut self, action: &Action) -> Result<()> {
        // Create a new concept/idea
        self.state.thoughts.push(format!("Conceived of: {}", action.target));
        self.state.beliefs.insert(
            format!("concept.{}", action.target),
            serde_json::json!({"exists": true})
        );

        if self.verbose {
            println!("  💡 Created concept: {}", action.target);
        }

        Ok(())
    }

    fn bind_concept(&mut self, action: &Action) -> Result<()> {
        // Bind a concept to a value (mental variable)
        if let Some(params) = &action.params {
            if let Some(value) = params.get("value") {
                self.state.beliefs.insert(action.target.clone(), value.clone());

                if self.verbose {
                    println!("  🔗 Bound: {} = {}", action.target, value);
                }
            }
        }

        Ok(())
    }

    fn create_obligation(&mut self, action: &Action) -> Result<()> {
        // Create an obligation/commitment
        if let Some(params) = &action.params {
            if let Some(duty) = params.get("duty").and_then(|v| v.as_str()) {
                self.state.goals.push(format!("Must: {}", duty));

                // Obligations can create stress/emotion
                *self.state.emotions.entry("responsibility".to_string()).or_insert(0.0) += 0.5;

                if self.verbose {
                    println!("  ⚖️  Obligation: {}", duty);
                }
            }
        }

        Ok(())
    }

    fn wait(&mut self, action: &Action) -> Result<()> {
        // Simulate waiting (time passing)
        let duration = action.dur.unwrap_or(1.0);

        self.state.thoughts.push(format!("Waiting for {:.1}s", duration));

        if self.verbose {
            println!("  ⏳ Waiting: {:.1}s", duration);
        }

        Ok(())
    }

    fn gen_random_int(&mut self, action: &Action) -> Result<()> {
        // Generate a random integer
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let (min, max) = if let Some(params) = &action.params {
            let min_val = params.get("min")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            let max_val = params.get("max")
                .and_then(|v| v.as_i64())
                .unwrap_or(9);
            (min_val, max_val)
        } else {
            (0, 9)
        };

        // Simple random number generation using system time
        let state = RandomState::new();
        let mut hasher = state.build_hasher();
        std::time::SystemTime::now().hash(&mut hasher);
        action.target.hash(&mut hasher);
        let hash = hasher.finish();
        let range = (max - min + 1) as u64;
        let random_num = min + (hash % range) as i64;

        // Store in beliefs
        self.state.beliefs.insert(
            action.target.clone(),
            serde_json::json!(random_num)
        );

        self.state.thoughts.push(format!("Generated random number: {} = {}", action.target, random_num));

        if self.verbose {
            println!("  🎲 Generated: {} = {}", action.target, random_num);
        }

        Ok(())
    }

    fn execute_if(&mut self, action: &Action) -> Result<()> {
        let condition = action.condition.as_ref()
            .ok_or_else(|| anyhow!("If requires condition"))?;

        let result = self.evaluate_condition(condition)?;

        if self.verbose {
            println!("  🤔 Evaluating condition: {}", result);
        }

        if result {
            if let Some(then_actions) = &action.then_actions {
                for then_action in then_actions {
                    self.recursion_depth += 1;
                    self.execute_action(then_action)?;
                    self.recursion_depth -= 1;
                }
            }
        } else if let Some(else_actions) = &action.else_actions {
            for else_action in else_actions {
                self.recursion_depth += 1;
                self.execute_action(else_action)?;
                self.recursion_depth -= 1;
            }
        }

        Ok(())
    }

    fn execute_while(&mut self, action: &Action) -> Result<()> {
        let condition = action.condition.as_ref()
            .ok_or_else(|| anyhow!("While requires condition"))?;

        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 10000;

        while self.evaluate_condition(condition)? {
            if iterations >= MAX_ITERATIONS {
                return Err(anyhow!("While loop exceeded maximum iterations"));
            }

            if let Some(body_actions) = &action.body_actions {
                for body_action in body_actions {
                    self.recursion_depth += 1;
                    self.execute_action(body_action)?;
                    self.recursion_depth -= 1;
                }
            }

            iterations += 1;
        }

        if self.verbose {
            println!("  🔄 Loop completed {} iterations", iterations);
        }

        Ok(())
    }

    fn execute_for(&mut self, action: &Action) -> Result<()> {
        let loop_var = action.loop_var.as_ref()
            .ok_or_else(|| anyhow!("For requires variable"))?;
        let from_expr = action.from_expr.as_ref()
            .ok_or_else(|| anyhow!("For requires from expression"))?;
        let to_expr = action.to_expr.as_ref()
            .ok_or_else(|| anyhow!("For requires to expression"))?;

        let from_val = self.evaluate_expression(from_expr)?;
        let to_val = self.evaluate_expression(to_expr)?;

        let from_i = from_val.as_i64().ok_or_else(|| anyhow!("For from must be integer"))?;
        let to_i = to_val.as_i64().ok_or_else(|| anyhow!("For to must be integer"))?;

        for i in from_i..=to_i {
            // Set loop variable
            self.state.beliefs.insert(loop_var.clone(), serde_json::json!(i));

            if let Some(body_actions) = &action.body_actions {
                for body_action in body_actions {
                    self.recursion_depth += 1;
                    self.execute_action(body_action)?;
                    self.recursion_depth -= 1;
                }
            }
        }

        Ok(())
    }

    fn execute_define_function(&mut self, action: &Action) -> Result<()> {
        let func_name = &action.target;
        let params = action.params.as_ref()
            .ok_or_else(|| anyhow!("DefineFunction requires params"))?;

        let args = params.get("args")
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("DefineFunction requires args array"))?;

        let arg_names: Vec<String> = args.iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        let body_value = params.get("body")
            .ok_or_else(|| anyhow!("DefineFunction requires body"))?;

        let body_actions: Vec<Action> = serde_json::from_value(body_value.clone())?;

        let func_def = FunctionDef {
            args: arg_names.clone(),
            body: body_actions,
        };

        self.state.functions.insert(func_name.clone(), func_def);
        self.state.thoughts.push(format!("Learned new skill: {}({})", func_name, arg_names.join(", ")));

        if self.verbose {
            println!("  💡 Learned function: {}({})", func_name, arg_names.join(", "));
        }

        Ok(())
    }

    fn evaluate_condition(&mut self, condition: &Condition) -> Result<bool> {
        match condition {
            Condition::Comparison { op, left, right } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;

                let result = match op {
                    ComparisonOp::Equal => left_val == right_val,
                    ComparisonOp::NotEqual => left_val != right_val,
                    ComparisonOp::LessThan => {
                        if let (Some(l), Some(r)) = (left_val.as_f64(), right_val.as_f64()) {
                            l < r
                        } else {
                            false
                        }
                    }
                    ComparisonOp::LessThanOrEqual => {
                        if let (Some(l), Some(r)) = (left_val.as_f64(), right_val.as_f64()) {
                            l <= r
                        } else {
                            false
                        }
                    }
                    ComparisonOp::GreaterThan => {
                        if let (Some(l), Some(r)) = (left_val.as_f64(), right_val.as_f64()) {
                            l > r
                        } else {
                            false
                        }
                    }
                    ComparisonOp::GreaterThanOrEqual => {
                        if let (Some(l), Some(r)) = (left_val.as_f64(), right_val.as_f64()) {
                            l >= r
                        } else {
                            false
                        }
                    }
                };
                Ok(result)
            }
            Condition::And { operands } => {
                for cond in operands {
                    if !self.evaluate_condition(cond)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            Condition::Or { operands } => {
                for cond in operands {
                    if self.evaluate_condition(cond)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            Condition::Not { operand } => {
                Ok(!self.evaluate_condition(operand)?)
            }
        }
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Result<serde_json::Value> {
        match expr {
            Expression::Value(v) => Ok(v.clone()),
            Expression::Variable { var } => {
                self.state.beliefs.get(var)
                    .cloned()
                    .ok_or_else(|| anyhow!("Variable not found: {}", var))
            }
            Expression::BinaryOp { expr: bin_op } => {
                let left_val = self.evaluate_expression(&bin_op.left)?;
                let right_val = self.evaluate_expression(&bin_op.right)?;

                let left_num = left_val.as_f64().ok_or_else(|| anyhow!("Left operand must be number"))?;
                let right_num = right_val.as_f64().ok_or_else(|| anyhow!("Right operand must be number"))?;

                let result = match bin_op.op.as_str() {
                    "+" => left_num + right_num,
                    "-" => left_num - right_num,
                    "*" => left_num * right_num,
                    "/" => {
                        if right_num == 0.0 {
                            return Err(anyhow!("Division by zero"));
                        }
                        left_num / right_num
                    }
                    "%" => left_num % right_num,
                    _ => return Err(anyhow!("Unknown operator: {}", bin_op.op)),
                };

                Ok(serde_json::json!(result))
            }
            Expression::FunctionCall { call, args } => {
                // Get function definition
                let func_def = self.state.functions.get(call)
                    .ok_or_else(|| anyhow!("Function not defined: {}", call))?
                    .clone();

                // Save current variable state
                let saved_vars: HashMap<String, serde_json::Value> = func_def.args.iter()
                    .filter_map(|arg| self.state.beliefs.get(arg).map(|v| (arg.clone(), v.clone())))
                    .collect();

                // Bind arguments
                for (arg_name, arg_expr) in args {
                    let arg_value = self.evaluate_expression(arg_expr)?;
                    self.state.beliefs.insert(arg_name.clone(), arg_value);
                }

                // Execute function body
                let mut return_value = serde_json::Value::Null;
                for action in &func_def.body {
                    // Check for Return operation
                    if matches!(action.op, Operation::Return) {
                        if let Some(params) = &action.params {
                            if let Some(value_expr) = params.get("value") {
                                // value_expr might be an Expression wrapped in JSON
                                // Try to deserialize it as Expression
                                if let Ok(expr) = serde_json::from_value::<Expression>(value_expr.clone()) {
                                    return_value = self.evaluate_expression(&expr)?;
                                } else {
                                    return_value = value_expr.clone();
                                }
                            }
                        }
                        break;
                    }

                    self.recursion_depth += 1;
                    self.execute_action(action)?;
                    self.recursion_depth -= 1;
                }

                // Restore saved variables
                for (arg_name, saved_value) in saved_vars {
                    self.state.beliefs.insert(arg_name, saved_value);
                }

                Ok(return_value)
            }
        }
    }

    fn physical_action(&mut self, action: &Action, emoji: &str, verb: &str) -> Result<()> {
        // Simulate performing a physical action
        let description = if let Some(params) = &action.params {
            // Build a natural description from params
            let mut parts = vec![format!("{} {}", verb, action.target)];

            if let Some(from) = params.get("from") {
                parts.push(format!("from {}", from.as_str().unwrap_or("?")));
            }
            if let Some(into) = params.get("into") {
                parts.push(format!("into {}", into.as_str().unwrap_or("?")));
            }
            if let Some(amount) = params.get("amount") {
                parts.push(format!("({})", amount.as_str().unwrap_or("?")));
            }

            parts.join(" ")
        } else {
            format!("{} {}", verb, action.target)
        };

        self.state.thoughts.push(format!("Performing action: {}", description));

        // Track the action in working memory
        self.state.working_memory.push(description.clone());
        if self.state.working_memory.len() > 7 {
            self.state.working_memory.remove(0);
        }

        // Physical actions create mild satisfaction
        *self.state.emotions.entry("focus".to_string()).or_insert(0.0) += 0.2;

        if self.verbose {
            println!("  {} {}", emoji, description);
        }

        Ok(())
    }
}

impl Default for BrainSimulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_fact() {
        let mut brain = BrainSimulator::new();
        let mut params = HashMap::new();
        params.insert("entity".to_string(), serde_json::json!("cat"));
        params.insert("color".to_string(), serde_json::json!("black"));

        let action = Action::new("listener", Operation::StoreFact, "memory")
            .with_params(params);

        brain.execute_action(&action).unwrap();

        assert!(brain.state.beliefs.contains_key("cat.color"));
        assert_eq!(brain.state.beliefs.get("cat.color").unwrap(), "black");
    }

    #[test]
    fn test_emit() {
        let mut brain = BrainSimulator::new();
        let mut params = HashMap::new();
        params.insert("content".to_string(), serde_json::json!("Hello!"));

        let action = Action::new("speaker", Operation::Emit, "greeting")
            .with_params(params);

        brain.execute_action(&action).unwrap();

        assert_eq!(brain.state.output.len(), 1);
        assert_eq!(brain.state.output[0], "Hello!");
    }

    #[test]
    fn test_decide() {
        let mut brain = BrainSimulator::new();
        let mut params = HashMap::new();
        params.insert("choice".to_string(), serde_json::json!("go left"));

        let action = Action::new("decider", Operation::Decide, "path")
            .with_params(params);

        brain.execute_action(&action).unwrap();

        assert!(!brain.state.thoughts.is_empty());
    }
}

