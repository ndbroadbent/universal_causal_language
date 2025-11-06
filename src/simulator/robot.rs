use crate::{Action, Operation, Program, Condition, ComparisonOp, Expression};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Represents a learned function in robot memory
#[derive(Debug, Clone)]
pub struct RobotFunctionDef {
    pub args: Vec<String>,
    pub body: Vec<Action>,
}

/// Represents the state of a simulated robot
#[derive(Debug, Clone)]
pub struct RobotState {
    /// Position of objects in 3D space
    pub objects: HashMap<String, ObjectState>,

    /// Robot arm position
    pub arm_position: (f64, f64, f64),

    /// What the robot is currently holding
    pub gripper: Option<String>,

    /// Temperature sensors
    pub temperatures: HashMap<String, f64>,

    /// Execution log
    pub log: Vec<String>,

    /// Error state
    pub errors: Vec<String>,

    /// Variables/memory
    pub variables: HashMap<String, serde_json::Value>,

    /// Learned functions/procedures
    pub functions: HashMap<String, RobotFunctionDef>,
}

#[derive(Debug, Clone)]
pub struct ObjectState {
    pub position: (f64, f64, f64),
    pub container: Option<String>,
    pub temperature: f64,
    pub state: String,  // "solid", "liquid", "gas", "mixed", etc.
}

impl RobotState {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            arm_position: (0.0, 0.0, 0.0),
            gripper: None,
            temperatures: HashMap::new(),
            log: Vec::new(),
            errors: Vec::new(),
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str("=== Robot State ===\n\n");

        output.push_str(&format!("Arm Position: ({:.2}, {:.2}, {:.2})\n",
            self.arm_position.0, self.arm_position.1, self.arm_position.2));

        if let Some(held) = &self.gripper {
            output.push_str(&format!("Gripper: Holding {}\n", held));
        } else {
            output.push_str("Gripper: Empty\n");
        }
        output.push('\n');

        if !self.objects.is_empty() {
            output.push_str("Objects:\n");
            for (name, obj) in &self.objects {
                output.push_str(&format!("  {} - pos:({:.1}, {:.1}, {:.1}), temp:{:.0}°C, state:{}\n",
                    name, obj.position.0, obj.position.1, obj.position.2, obj.temperature, obj.state));
            }
            output.push('\n');
        }

        if !self.temperatures.is_empty() {
            output.push_str("Temperature Sensors:\n");
            for (sensor, temp) in &self.temperatures {
                output.push_str(&format!("  {}: {:.1}°C\n", sensor, temp));
            }
            output.push('\n');
        }

        if !self.log.is_empty() {
            output.push_str("Execution Log:\n");
            for (i, entry) in self.log.iter().enumerate() {
                output.push_str(&format!("  {}. {}\n", i + 1, entry));
            }
            output.push('\n');
        }

        if !self.errors.is_empty() {
            output.push_str("⚠️  Errors:\n");
            for error in &self.errors {
                output.push_str(&format!("  • {}\n", error));
            }
        }

        output
    }
}

impl Default for RobotState {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulates a robot as a VM that executes physical operations
pub struct RobotSimulator {
    state: RobotState,
    verbose: bool,
    recursion_depth: usize,
    max_recursion_depth: usize,
}

impl RobotSimulator {
    pub fn new() -> Self {
        Self {
            state: RobotState::new(),
            verbose: false,
            recursion_depth: 0,
            max_recursion_depth: 1000,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn state(&self) -> &RobotState {
        &self.state
    }

    pub fn execute(&mut self, program: &Program) -> Result<()> {
        if self.verbose {
            println!("🤖 Starting robot execution...\n");
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

        match &action.op {
            // Control flow operations
            Operation::If => self.execute_if(action),
            Operation::While => self.execute_while(action),
            Operation::For => self.execute_for(action),
            Operation::DefineFunction => self.execute_define_function(action),
            Operation::Bind => self.bind_variable(action),
            Operation::Return => Ok(()), // Handled by function call

            // Physical operations
            Operation::Gather => self.gather(action),
            Operation::Measure => self.measure(action),
            Operation::Heat => self.heat(action),
            Operation::Pour => self.pour(action),
            Operation::Mix => self.mix(action),
            Operation::Stir => self.stir(action),
            Operation::Place => self.place(action),
            Operation::Remove => self.remove(action),
            Operation::Steep => self.steep(action),
            Operation::Serve => self.serve(action),
            Operation::Wait => self.wait(action),
            Operation::Emit => self.emit(action),

            _ => {
                let error = format!("Unsupported operation: {:?}", action.op);
                self.state.errors.push(error.clone());

                if self.verbose {
                    println!("  ⚠️  {}", error);
                }

                Ok(())
            }
        }
    }

    fn gather(&mut self, action: &Action) -> Result<()> {
        if let Some(params) = &action.params {
            if let Some(items) = params.get("items").and_then(|v| v.as_array()) {
                for item in items {
                    if let Some(item_name) = item.as_str() {
                        self.state.objects.insert(
                            item_name.to_string(),
                            ObjectState {
                                position: (0.0, 0.0, 0.0),
                                container: None,
                                temperature: 20.0,
                                state: "ready".to_string(),
                            }
                        );
                    }
                }
            }
        }

        let msg = format!("Gathered items for {}", action.target);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  🤖 {}", msg);
        }

        Ok(())
    }

    fn measure(&mut self, action: &Action) -> Result<()> {
        let amount = action.params
            .as_ref()
            .and_then(|p| p.get("amount"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let msg = format!("Measured {} of {}", amount, action.target);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  📏 {}", msg);
        }

        Ok(())
    }

    fn heat(&mut self, action: &Action) -> Result<()> {
        let temp = action.params
            .as_ref()
            .and_then(|p| p.get("temperature"))
            .and_then(|v| v.as_str())
            .unwrap_or("100°C");

        if let Some(obj) = self.state.objects.get_mut(&action.target) {
            obj.temperature = 100.0;
            obj.state = "boiling".to_string();
        }

        let msg = format!("Heating {} to {}", action.target, temp);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  🔥 {}", msg);
        }

        Ok(())
    }

    fn pour(&mut self, action: &Action) -> Result<()> {
        if let Some(params) = &action.params {
            let from = params.get("from").and_then(|v| v.as_str()).unwrap_or("?");
            let into = params.get("into").and_then(|v| v.as_str()).unwrap_or("?");
            let amount = params.get("amount").and_then(|v| v.as_str()).unwrap_or("?");

            let msg = format!("Poured {} from {} into {} ({})", action.target, from, into, amount);
            self.state.log.push(msg.clone());

            if self.verbose {
                println!("  🫗 {}", msg);
            }
        }

        Ok(())
    }

    fn mix(&mut self, action: &Action) -> Result<()> {
        if let Some(obj) = self.state.objects.get_mut(&action.target) {
            obj.state = "mixed".to_string();
        }

        let msg = format!("Mixed {}", action.target);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  🥄 {}", msg);
        }

        Ok(())
    }

    fn stir(&mut self, action: &Action) -> Result<()> {
        let msg = format!("Stirred {}", action.target);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  🥄 {}", msg);
        }

        Ok(())
    }

    fn place(&mut self, action: &Action) -> Result<()> {
        let into = action.params
            .as_ref()
            .and_then(|p| p.get("into"))
            .and_then(|v| v.as_str())
            .unwrap_or("?");

        if let Some(obj) = self.state.objects.get_mut(&action.target) {
            obj.container = Some(into.to_string());
        }

        let msg = format!("Placed {} into {}", action.target, into);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  📍 {}", msg);
        }

        Ok(())
    }

    fn remove(&mut self, action: &Action) -> Result<()> {
        let from = action.params
            .as_ref()
            .and_then(|p| p.get("from"))
            .and_then(|v| v.as_str())
            .unwrap_or("?");

        if let Some(obj) = self.state.objects.get_mut(&action.target) {
            obj.container = None;
        }

        let msg = format!("Removed {} from {}", action.target, from);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  ✋ {}", msg);
        }

        Ok(())
    }

    fn steep(&mut self, action: &Action) -> Result<()> {
        let duration = action.params
            .as_ref()
            .and_then(|p| p.get("duration"))
            .and_then(|v| v.as_str())
            .unwrap_or("?");

        let msg = format!("Steeping {} for {}", action.target, duration);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  ⏱️  {}", msg);
        }

        Ok(())
    }

    fn serve(&mut self, action: &Action) -> Result<()> {
        let msg = format!("Serving {}", action.target);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  🍽️  {}", msg);
        }

        Ok(())
    }

    fn wait(&mut self, action: &Action) -> Result<()> {
        let duration = action.dur.unwrap_or(1.0);

        let msg = format!("Waiting {:.0}s for {}", duration, action.target);
        self.state.log.push(msg.clone());

        if self.verbose {
            println!("  ⏳ {}", msg);
        }

        Ok(())
    }

    fn emit(&mut self, action: &Action) -> Result<()> {
        let msg = action.params
            .as_ref()
            .and_then(|p| p.get("content"))
            .and_then(|v| v.as_str())
            .unwrap_or(&action.target);

        let log_msg = format!("Output: {}", msg);
        self.state.log.push(log_msg);

        if self.verbose {
            println!("  📢 {}", msg);
        }

        Ok(())
    }

    fn bind_variable(&mut self, action: &Action) -> Result<()> {
        if let Some(params) = &action.params {
            if let Some(value) = params.get("value") {
                self.state.variables.insert(action.target.clone(), value.clone());

                if self.verbose {
                    println!("  💾 Stored: {} = {}", action.target, value);
                }
            }
        }

        Ok(())
    }

    fn execute_if(&mut self, action: &Action) -> Result<()> {
        let condition = action.condition.as_ref()
            .ok_or_else(|| anyhow!("If requires condition"))?;

        let result = self.evaluate_condition(condition)?;

        if self.verbose {
            println!("  🤔 Condition: {}", result);
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
            println!("  🔄 Loop: {} iterations", iterations);
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
            self.state.variables.insert(loop_var.clone(), serde_json::json!(i));

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

        let func_def = RobotFunctionDef {
            args: arg_names.clone(),
            body: body_actions,
        };

        self.state.functions.insert(func_name.clone(), func_def);

        if self.verbose {
            println!("  📚 Learned: {}({})", func_name, arg_names.join(", "));
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
                self.state.variables.get(var)
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
                    .filter_map(|arg| self.state.variables.get(arg).map(|v| (arg.clone(), v.clone())))
                    .collect();

                // Bind arguments
                for (arg_name, arg_expr) in args {
                    let arg_value = self.evaluate_expression(arg_expr)?;
                    self.state.variables.insert(arg_name.clone(), arg_value);
                }

                // Execute function body
                let mut return_value = serde_json::Value::Null;
                for action in &func_def.body {
                    // Check for Return operation
                    if matches!(action.op, Operation::Return) {
                        if let Some(params) = &action.params {
                            if let Some(value_expr) = params.get("value") {
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
                    self.state.variables.insert(arg_name, saved_value);
                }

                Ok(return_value)
            }
        }
    }
}

impl Default for RobotSimulator {
    fn default() -> Self {
        Self::new()
    }
}

