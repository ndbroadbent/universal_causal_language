use crate::{Action, Operation, Program};
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Represents the state of a Mock LLM
#[derive(Debug, Clone)]
pub struct MockAIState {
    /// Deterministic mapping: instruction keyword → UCL code (as JSON)
    pub knowledge_base: HashMap<String, String>,

    /// History of prompts received
    pub prompts: Vec<String>,

    /// History of code generated
    pub responses: Vec<String>,

    /// Generated code stored by target name
    pub generated_code: HashMap<String, Vec<Action>>,

    /// Model configuration
    pub model_name: String,
    pub temperature: f64,
}

impl MockAIState {
    pub fn new() -> Self {
        let mut knowledge_base = HashMap::new();

        // Preload common programming tasks as UCL
        knowledge_base.insert(
            "factorial".to_string(),
            r#"[
  {
    "actor": "VM",
    "op": "DefineFunction",
    "target": "factorial",
    "params": {
      "args": ["n"],
      "body": [
        {
          "actor": "VM",
          "op": "If",
          "target": "base_case",
          "condition": {
            "type": "comparison",
            "op": "<=",
            "left": {"var": "n"},
            "right": 1
          },
          "then": [
            {
              "actor": "VM",
              "op": "Return",
              "target": "result",
              "params": {"value": 1}
            }
          ],
          "else": [
            {
              "actor": "VM",
              "op": "Return",
              "target": "result",
              "params": {
                "value": {
                  "expr": {
                    "op": "*",
                    "left": {"var": "n"},
                    "right": {
                      "call": "factorial",
                      "args": {
                        "n": {
                          "expr": {
                            "op": "-",
                            "left": {"var": "n"},
                            "right": 1
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          ]
        }
      ]
    }
  }
]"#.to_string(),
        );

        knowledge_base.insert(
            "fibonacci".to_string(),
            r#"[
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
          "condition": {
            "type": "comparison",
            "op": "<=",
            "left": {"var": "n"},
            "right": 1
          },
          "then": [
            {
              "actor": "VM",
              "op": "Return",
              "target": "result",
              "params": {"value": {"var": "n"}}
            }
          ],
          "else": [
            {
              "actor": "VM",
              "op": "Return",
              "target": "result",
              "params": {
                "value": {
                  "expr": {
                    "op": "+",
                    "left": {
                      "call": "fibonacci",
                      "args": {
                        "n": {
                          "expr": {
                            "op": "-",
                            "left": {"var": "n"},
                            "right": 1
                          }
                        }
                      }
                    },
                    "right": {
                      "call": "fibonacci",
                      "args": {
                        "n": {
                          "expr": {
                            "op": "-",
                            "left": {"var": "n"},
                            "right": 2
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          ]
        }
      ]
    }
  }
]"#.to_string(),
        );

        knowledge_base.insert(
            "hello world".to_string(),
            r#"[
  {
    "actor": "VM",
    "op": "Emit",
    "target": "output",
    "params": {"content": "Hello, World!"}
  }
]"#.to_string(),
        );

        Self {
            knowledge_base,
            prompts: Vec::new(),
            responses: Vec::new(),
            generated_code: HashMap::new(),
            model_name: "MockLLM-UCL-v1".to_string(),
            temperature: 0.0,
        }
    }

    pub fn display(&self) -> String {
        let mut output = String::new();

        output.push_str("=== Mock AI State ===\n\n");

        output.push_str(&format!("Model: {} (temperature: {})\n\n", self.model_name, self.temperature));

        if !self.prompts.is_empty() {
            output.push_str("Prompt History:\n");
            for (i, prompt) in self.prompts.iter().enumerate() {
                output.push_str(&format!("  {}. {}\n", i + 1, prompt));
            }
            output.push('\n');
        }

        if !self.generated_code.is_empty() {
            output.push_str("Generated Code:\n");
            for (name, actions) in &self.generated_code {
                output.push_str(&format!("  {} - {} actions\n", name, actions.len()));
            }
            output.push('\n');
        }

        output.push_str(&format!("Knowledge Base: {} preloaded tasks\n", self.knowledge_base.len()));
        output.push_str("  • factorial\n");
        output.push_str("  • fibonacci\n");
        output.push_str("  • hello world\n");

        output
    }
}

impl Default for MockAIState {
    fn default() -> Self {
        Self::new()
    }
}

/// Simulates a Mock LLM that generates code from instructions
pub struct MockAISimulator {
    state: MockAIState,
    verbose: bool,
}

impl MockAISimulator {
    pub fn new() -> Self {
        Self {
            state: MockAIState::new(),
            verbose: false,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn state(&self) -> &MockAIState {
        &self.state
    }

    pub fn execute(&mut self, program: &Program) -> Result<()> {
        if self.verbose {
            println!("🤖 Starting Mock AI execution...\n");
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
        match &action.op {
            Operation::Generate => self.generate(action),
            Operation::Parse => self.parse(action),
            Operation::Execute => self.execute_code(action),
            Operation::Emit => self.emit(action),
            _ => {
                if self.verbose {
                    println!("  ⚠️  Unsupported operation: {:?}", action.op);
                }
                Ok(())
            }
        }
    }

    fn generate(&mut self, action: &Action) -> Result<()> {
        let instruction = action.params
            .as_ref()
            .and_then(|p| p.get("instruction"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Generate requires 'instruction' parameter"))?;

        // Record the prompt
        self.state.prompts.push(instruction.to_string());

        if self.verbose {
            println!("  💭 Received instruction: \"{}\"", instruction);
        }

        // Look up in knowledge base (fuzzy match on keywords)
        let mut matched_key = None;
        for key in self.state.knowledge_base.keys() {
            if instruction.to_lowercase().contains(key) {
                matched_key = Some(key.clone());
                break;
            }
        }

        let generated_code = if let Some(key) = matched_key {
            let code_json = self.state.knowledge_base.get(&key).unwrap();

            if self.verbose {
                println!("  🧠 Matched knowledge: \"{}\"", key);
                println!("  ✨ Generating UCL code...");
            }

            // Parse the JSON into actions
            let actions: Vec<Action> = serde_json::from_str(code_json)?;

            self.state.generated_code.insert(action.target.clone(), actions.clone());
            self.state.responses.push(format!("Generated {} for: {}", key, instruction));

            if self.verbose {
                println!("  ✅ Generated {} UCL actions", actions.len());
            }

            Ok(())
        } else {
            let error = format!("I don't know how to: {}", instruction);
            self.state.responses.push(error.clone());

            if self.verbose {
                println!("  ❌ {}", error);
            }

            Err(anyhow!("No knowledge base entry for: {}", instruction))
        };

        generated_code
    }

    fn parse(&mut self, action: &Action) -> Result<()> {
        if self.verbose {
            println!("  📝 Parsing code from {}", action.target);
        }

        // In mock AI, parsing is a no-op since we generate UCL directly
        Ok(())
    }

    fn execute_code(&mut self, action: &Action) -> Result<()> {
        let code_name = action.params
            .as_ref()
            .and_then(|p| p.get("code"))
            .and_then(|v| v.as_str())
            .unwrap_or(&action.target);

        let actions = self.state.generated_code.get(code_name)
            .ok_or_else(|| anyhow!("No generated code found: {}", code_name))?
            .clone();

        if self.verbose {
            println!("  ⚙️  Executing generated code: {}", code_name);
            println!("  📊 {} actions to execute", actions.len());
        }

        // Execute the generated actions
        // Note: This would typically delegate to another VM
        // For now, we just track that execution was requested

        Ok(())
    }

    fn emit(&mut self, action: &Action) -> Result<()> {
        let msg = action.params
            .as_ref()
            .and_then(|p| p.get("content"))
            .and_then(|v| v.as_str())
            .unwrap_or(&action.target);

        if self.verbose {
            println!("  🗣️  {}", msg);
        }

        Ok(())
    }
}

impl Default for MockAISimulator {
    fn default() -> Self {
        Self::new()
    }
}

