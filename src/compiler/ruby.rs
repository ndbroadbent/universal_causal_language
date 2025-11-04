use crate::{Action, Operation, Program};
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
            // Check if it's a computed value from registers
            if let (Some(lhs_reg), Some(rhs_reg), Some(op)) =
                (params.get("lhs_register"), params.get("rhs_register"), params.get("operation")) {
                let lhs_name = lhs_reg.as_str().unwrap_or("");
                let rhs_name = rhs_reg.as_str().unwrap_or("");
                let operation = op.as_str().unwrap_or("");

                let operator = match operation {
                    "multiply" => "*",
                    "add" => "+",
                    "subtract" => "-",
                    "divide" => "/",
                    _ => "*",
                };

                return Ok(format!("{}{} = {} {} {}", indent, action.target, lhs_name, operator, rhs_name));
            }

            // Otherwise use direct value
            if let Some(value) = params.get("value") {
                return Ok(format!("{}{} = {}", indent, action.target, self.value_to_ruby(value)));
            }
        }

        Err(anyhow!("Write requires 'value' parameter or register operation"))
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
                // If content matches the target, it's likely a variable reference
                if content.as_str() == Some(&action.target) {
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
        let value = action.params
            .as_ref()
            .and_then(|p| p.get("value"))
            .ok_or_else(|| anyhow!("Bind requires 'value' parameter"))?;

        let var_name = &action.target;
        self.variables.insert(var_name.clone(), "bound".to_string());

        Ok(format!("{}{} = {}", indent, var_name, self.value_to_ruby(value)))
    }

    fn compile_return(&mut self, action: &Action, indent: &str) -> Result<String> {
        let value = action.params
            .as_ref()
            .and_then(|p| p.get("value"))
            .map(|v| self.value_to_ruby(v))
            .unwrap_or_else(|| action.target.clone());

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

