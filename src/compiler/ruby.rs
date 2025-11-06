use crate::{Action, Operation, Program, Condition, ComparisonOp, Expression};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub struct RubyCompiler {
    indent_level: usize,
    variables: HashMap<String, String>,
}

impl RubyCompiler {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            variables: HashMap::new(),
        }
    }

    pub fn compile(&mut self, program: &Program) -> Result<String> {
        let mut output = String::new();

        // Add a header comment
        output.push_str("# Generated from UCL\n");
        output.push_str("# Universal Causal Language -> Ruby Compiler\n\n");

        // Compile each action
        for action in &program.actions {
            let code = self.compile_action(action)?;
            if !code.is_empty() {
                output.push_str(&code);
                output.push('\n');
            }
        }

        Ok(output)
    }

    fn compile_action(&mut self, action: &Action) -> Result<String> {
        let indent = "  ".repeat(self.indent_level);

        match &action.op {
            Operation::Call => self.compile_call(action, &indent),
            Operation::Assign => self.compile_assign(action, &indent),
            Operation::Write => self.compile_write(action, &indent),
            Operation::Read => self.compile_read(action, &indent),
            Operation::Create => self.compile_create(action, &indent),
            Operation::Emit => self.compile_emit(action, &indent),
            Operation::Assert => self.compile_assert(action, &indent),
            Operation::StoreFact => self.compile_store_fact(action, &indent),
            Operation::Bind => self.compile_bind(action, &indent),
            Operation::Return => self.compile_return(action, &indent),
            Operation::Decide => self.compile_decide(action, &indent),
            Operation::Wait => self.compile_wait(action, &indent),
            Operation::GenRandomInt => self.compile_gen_random_int(action, &indent),
            Operation::If => self.compile_if(action),
            Operation::While => self.compile_while(action),
            Operation::For => self.compile_for(action),
            Operation::DefineFunction => self.compile_define_function(action),
            _ => {
                // For unsupported operations, generate a comment
                Ok(format!("{}# Unsupported operation: {:?} on {}",
                    indent, action.op, action.target))
            }
        }
    }

    fn compile_call(&mut self, action: &Action, indent: &str) -> Result<String> {
        let params = action.params.as_ref();

        // Handle special case for binary operators with registers
        if let Some(p) = params {
            // Check for register references first
            if let (Some(lhs_reg), Some(rhs_reg)) = (p.get("lhs_register"), p.get("rhs_register")) {
                let target = &action.target;
                let lhs_name = lhs_reg.as_str().unwrap_or("");
                let rhs_name = rhs_reg.as_str().unwrap_or("");

                // Check if it's an operator
                if ["+", "-", "*", "/", "%", "**"].contains(&target.as_str()) {
                    return Ok(format!("{}({} {} {})",
                        indent,
                        lhs_name,
                        target,
                        rhs_name));
                }
            }
            // Then check for direct values
            else if let (Some(lhs), Some(rhs)) = (p.get("lhs"), p.get("rhs")) {
                let target = &action.target;

                // Check if it's an operator
                if ["+", "-", "*", "/", "%", "**"].contains(&target.as_str()) {
                    return Ok(format!("{}({} {} {})",
                        indent,
                        self.value_to_ruby(lhs),
                        target,
                        self.value_to_ruby(rhs)));
                }
            }
        }

        // Regular method call
        let mut args = Vec::new();
        if let Some(p) = params {
            // Extract arguments in order (if they exist)
            for key in ["a", "b", "c", "arg", "args", "n", "x", "y", "z"] {
                if let Some(val) = p.get(key) {
                    args.push(self.value_to_ruby(val));
                }
            }

            // If no standard args found, use all params
            if args.is_empty() {
                for (key, val) in p.iter() {
                    if !["lhs", "rhs", "receiver", "out"].contains(&key.as_str()) {
                        args.push(format!("{}: {}", key, self.value_to_ruby(val)));
                    }
                }
            }
        }

        let args_str = args.join(", ");
        Ok(format!("{}{}({})", indent, action.target, args_str))
    }

    fn compile_assign(&mut self, action: &Action, indent: &str) -> Result<String> {
        let value = action.params
            .as_ref()
            .and_then(|p| p.get("value"))
            .ok_or_else(|| anyhow!("Assign requires 'value' parameter"))?;

        let var_name = &action.target;
        self.variables.insert(var_name.clone(), "assigned".to_string());

        Ok(format!("{}{} = {}", indent, var_name, self.value_to_ruby(value)))
    }

    fn compile_write(&mut self, action: &Action, indent: &str) -> Result<String> {
        if let Some(params) = &action.params {
            if let Some(op) = params.get("operation") {
                let operation = op.as_str().unwrap_or("");
                let operator = match operation {
                    "multiply" => "*",
                    "add" => "+",
                    "subtract" => "-",
                    "divide" => "/",
                    _ => "*",
                };

                // Get left operand (register or value)
                let lhs = if let Some(lhs_reg) = params.get("lhs_register") {
                    lhs_reg.as_str().unwrap_or("").to_string()
                } else if let Some(lhs_val) = params.get("lhs") {
                    self.value_to_ruby(lhs_val)
                } else {
                    return Err(anyhow!("Write operation requires lhs_register or lhs"));
                };

                // Get right operand (register or value)
                let rhs = if let Some(rhs_reg) = params.get("rhs_register") {
                    rhs_reg.as_str().unwrap_or("").to_string()
                } else if let Some(rhs_val) = params.get("rhs") {
                    self.value_to_ruby(rhs_val)
                } else {
                    return Err(anyhow!("Write operation requires rhs_register or rhs"));
                };

                return Ok(format!("{}{} = {} {} {}", indent, action.target, lhs, operator, rhs));
            }

            // Otherwise use direct value
            if let Some(value) = params.get("value") {
                return Ok(format!("{}{} = {}", indent, action.target, self.value_to_ruby(value)));
            }
        }

        Err(anyhow!("Write requires 'value' parameter or operation"))
    }

    fn compile_read(&mut self, action: &Action, indent: &str) -> Result<String> {
        Ok(format!("{}{}", indent, action.target))
    }

    fn compile_create(&mut self, action: &Action, indent: &str) -> Result<String> {
        let class_name = &action.target;

        if let Some(params) = &action.params {
            let mut args = Vec::new();
            for (key, val) in params.iter() {
                args.push(format!("{}: {}", key, self.value_to_ruby(val)));
            }
            Ok(format!("{}{}.new({})", indent, class_name, args.join(", ")))
        } else {
            Ok(format!("{}{}.new", indent, class_name))
        }
    }

    fn compile_emit(&mut self, action: &Action, indent: &str) -> Result<String> {
        let msg = if let Some(params) = action.params.as_ref() {
            if let Some(content) = params.get("content") {
                // Try to parse as Expression first
                if let Ok(expr) = serde_json::from_value::<Expression>(content.clone()) {
                    self.compile_expression(&expr)?
                } else if content.as_str() == Some(&action.target) {
                    action.target.clone()
                } else {
                    self.value_to_ruby(content)
                }
            } else if let Some(message) = params.get("message") {
                self.value_to_ruby(message)
            } else {
                // No content param, just use target as variable
                action.target.clone()
            }
        } else {
            // No params, treat target as variable name
            action.target.clone()
        };

        Ok(format!("{}puts {}", indent, msg))
    }

    fn compile_assert(&mut self, action: &Action, indent: &str) -> Result<String> {
        let statement = action.params
            .as_ref()
            .and_then(|p| p.get("statement"))
            .map(|v| self.value_to_ruby(v))
            .unwrap_or_else(|| format!("\"{}\"", action.target));

        Ok(format!("{}# Assert: {}", indent, statement))
    }

    fn compile_store_fact(&mut self, action: &Action, indent: &str) -> Result<String> {
        if let Some(params) = &action.params {
            let mut facts = Vec::new();
            for (key, val) in params.iter() {
                facts.push(format!("{}.{} = {}",
                    action.target,
                    key,
                    self.value_to_ruby(val)));
            }
            Ok(format!("{}# Store fact: {}", indent, facts.join(", ")))
        } else {
            Ok(format!("{}# Store fact about {}", indent, action.target))
        }
    }

    fn compile_bind(&mut self, action: &Action, indent: &str) -> Result<String> {
        let value_json = action.params
            .as_ref()
            .and_then(|p| p.get("value"))
            .ok_or_else(|| anyhow!("Bind requires 'value' parameter"))?;

        let var_name = &action.target;
        self.variables.insert(var_name.clone(), "bound".to_string());

        // Try to parse as Expression first
        let value_str = if let Ok(expr) = serde_json::from_value::<Expression>(value_json.clone()) {
            self.compile_expression(&expr)?
        } else {
            self.value_to_ruby(value_json)
        };

        Ok(format!("{}{} = {}", indent, var_name, value_str))
    }

    fn compile_return(&mut self, action: &Action, indent: &str) -> Result<String> {
        let value = if let Some(params) = action.params.as_ref() {
            if let Some(value_json) = params.get("value") {
                // Try to parse as Expression first
                if let Ok(expr) = serde_json::from_value::<Expression>(value_json.clone()) {
                    self.compile_expression(&expr)?
                } else {
                    self.value_to_ruby(value_json)
                }
            } else {
                action.target.clone()
            }
        } else {
            action.target.clone()
        };

        Ok(format!("{}return {}", indent, value))
    }

    fn compile_decide(&mut self, action: &Action, indent: &str) -> Result<String> {
        let condition = action.params
            .as_ref()
            .and_then(|p| p.get("condition"))
            .map(|v| self.value_to_ruby(v))
            .unwrap_or_else(|| "true".to_string());

        Ok(format!("{}if {}", indent, condition))
    }

    fn compile_wait(&mut self, action: &Action, indent: &str) -> Result<String> {
        let duration = action.dur
            .or_else(|| {
                action.params.as_ref()
                    .and_then(|p| p.get("duration"))
                    .and_then(|v| v.as_f64())
            })
            .unwrap_or(1.0);

        Ok(format!("{}sleep {}", indent, duration))
    }

    fn compile_gen_random_int(&mut self, action: &Action, indent: &str) -> Result<String> {
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

        let var_name = &action.target;
        self.variables.insert(var_name.clone(), "random_int".to_string());

        // Ruby: variable = rand(min..max)
        Ok(format!("{}{} = rand({}..{})", indent, var_name, min, max))
    }

    fn compile_if(&mut self, action: &Action) -> Result<String> {
        let indent = "  ".repeat(self.indent_level);
        let condition = action.condition.as_ref()
            .ok_or_else(|| anyhow!("If operation requires condition"))?;

        let mut output = String::new();
        output.push_str(&format!("{}if {}\n", indent, self.compile_condition(condition)?));

        // Compile then branch
        if let Some(then_actions) = &action.then_actions {
            self.indent_level += 1;
            for then_action in then_actions {
                let code = self.compile_action(then_action)?;
                if !code.is_empty() {
                    output.push_str(&code);
                    output.push('\n');
                }
            }
            self.indent_level -= 1;
        }

        // Compile else branch if present
        if let Some(else_actions) = &action.else_actions {
            output.push_str(&format!("{}else\n", indent));
            self.indent_level += 1;
            for else_action in else_actions {
                let code = self.compile_action(else_action)?;
                if !code.is_empty() {
                    output.push_str(&code);
                    output.push('\n');
                }
            }
            self.indent_level -= 1;
        }

        output.push_str(&format!("{}end", indent));
        Ok(output)
    }

    fn compile_while(&mut self, action: &Action) -> Result<String> {
        let indent = "  ".repeat(self.indent_level);
        let condition = action.condition.as_ref()
            .ok_or_else(|| anyhow!("While operation requires condition"))?;

        let mut output = String::new();
        output.push_str(&format!("{}while {}\n", indent, self.compile_condition(condition)?));

        // Compile body
        if let Some(body_actions) = &action.body_actions {
            self.indent_level += 1;
            for body_action in body_actions {
                let code = self.compile_action(body_action)?;
                if !code.is_empty() {
                    output.push_str(&code);
                    output.push('\n');
                }
            }
            self.indent_level -= 1;
        }

        output.push_str(&format!("{}end", indent));
        Ok(output)
    }

    fn compile_for(&mut self, action: &Action) -> Result<String> {
        let indent = "  ".repeat(self.indent_level);
        let loop_var = action.loop_var.as_ref()
            .ok_or_else(|| anyhow!("For operation requires variable"))?;
        let from_expr = action.from_expr.as_ref()
            .ok_or_else(|| anyhow!("For operation requires from expression"))?;
        let to_expr = action.to_expr.as_ref()
            .ok_or_else(|| anyhow!("For operation requires to expression"))?;

        let from_val = self.compile_expression(from_expr)?;
        let to_val = self.compile_expression(to_expr)?;

        let mut output = String::new();
        output.push_str(&format!("{}({} .. {}).each do |{}|\n",
            indent, from_val, to_val, loop_var));

        // Compile body
        if let Some(body_actions) = &action.body_actions {
            self.indent_level += 1;
            for body_action in body_actions {
                let code = self.compile_action(body_action)?;
                if !code.is_empty() {
                    output.push_str(&code);
                    output.push('\n');
                }
            }
            self.indent_level -= 1;
        }

        output.push_str(&format!("{}end", indent));
        Ok(output)
    }

    fn compile_define_function(&mut self, action: &Action) -> Result<String> {
        let indent = "  ".repeat(self.indent_level);
        let func_name = &action.target;

        // Extract function args and body from params
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

        // Parse body as array of actions
        let body_actions: Vec<Action> = serde_json::from_value(body_value.clone())?;

        let mut output = String::new();
        output.push_str(&format!("{}def {}({})\n", indent, func_name, arg_names.join(", ")));

        // Compile function body
        self.indent_level += 1;
        for body_action in &body_actions {
            let code = self.compile_action(body_action)?;
            if !code.is_empty() {
                output.push_str(&code);
                output.push('\n');
            }
        }
        self.indent_level -= 1;

        output.push_str(&format!("{}end", indent));
        Ok(output)
    }

    fn compile_condition(&self, condition: &Condition) -> Result<String> {
        match condition {
            Condition::Comparison { op, left, right } => {
                let left_val = self.compile_expression(left)?;
                let right_val = self.compile_expression(right)?;
                let op_str = match op {
                    ComparisonOp::Equal => "==",
                    ComparisonOp::NotEqual => "!=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::LessThanOrEqual => "<=",
                    ComparisonOp::GreaterThan => ">",
                    ComparisonOp::GreaterThanOrEqual => ">=",
                };
                Ok(format!("{} {} {}", left_val, op_str, right_val))
            }
            Condition::And { operands } => {
                let parts: Result<Vec<String>> = operands.iter()
                    .map(|c| self.compile_condition(c))
                    .collect();
                Ok(format!("({})", parts?.join(" && ")))
            }
            Condition::Or { operands } => {
                let parts: Result<Vec<String>> = operands.iter()
                    .map(|c| self.compile_condition(c))
                    .collect();
                Ok(format!("({})", parts?.join(" || ")))
            }
            Condition::Not { operand } => {
                Ok(format!("!({})", self.compile_condition(operand)?))
            }
        }
    }

    fn compile_expression(&self, expr: &Expression) -> Result<String> {
        match expr {
            Expression::Value(v) => Ok(self.value_to_ruby(v)),
            Expression::Variable { var } => Ok(var.clone()),
            Expression::BinaryOp { expr: bin_op } => {
                let left_val = self.compile_expression(&bin_op.left)?;
                let right_val = self.compile_expression(&bin_op.right)?;
                Ok(format!("({} {} {})", left_val, bin_op.op, right_val))
            }
            Expression::FunctionCall { call, args } => {
                // For function calls in expressions, use positional arguments (in order of keys)
                // This assumes the args are in the right order or that there's only one arg
                let arg_strs: Result<Vec<String>> = args.values()
                    .map(|v| self.compile_expression(v))
                    .collect();
                Ok(format!("{}({})", call, arg_strs?.join(", ")))
            }
        }
    }

    fn value_to_ruby(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "nil".to_string(),
            serde_json::Value::Array(arr) => {
                let elements: Vec<String> = arr.iter()
                    .map(|v| self.value_to_ruby(v))
                    .collect();
                format!("[{}]", elements.join(", "))
            }
            serde_json::Value::Object(obj) => {
                let pairs: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, self.value_to_ruby(v)))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }
}

impl Default for RubyCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_compile_assign() {
        let mut compiler = RubyCompiler::new();
        let mut params = HashMap::new();
        params.insert("value".to_string(), serde_json::json!(42));

        let action = Action::new("VM", Operation::Assign, "x")
            .with_params(params);

        let code = compiler.compile_action(&action).unwrap();
        assert!(code.contains("x = 42"));
    }

    #[test]
    fn test_compile_call() {
        let mut compiler = RubyCompiler::new();
        let mut params = HashMap::new();
        params.insert("lhs".to_string(), serde_json::json!(2));
        params.insert("rhs".to_string(), serde_json::json!(3));

        let action = Action::new("VM", Operation::Call, "+")
            .with_params(params);

        let code = compiler.compile_action(&action).unwrap();
        assert!(code.contains("(2 + 3)"));
    }

    #[test]
    fn test_compile_emit() {
        let mut compiler = RubyCompiler::new();
        let mut params = HashMap::new();
        params.insert("content".to_string(), serde_json::json!("Hello, World!"));

        let action = Action::new("speaker", Operation::Emit, "message")
            .with_params(params);

        let code = compiler.compile_action(&action).unwrap();
        assert!(code.contains("puts"));
        assert!(code.contains("Hello, World!"));
    }
}

