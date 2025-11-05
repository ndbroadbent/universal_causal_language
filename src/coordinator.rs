use crate::{Action, Operation, Program};
use crate::compiler::RubyCompiler;
use crate::simulator::BrainSimulator;
use anyhow::Result;
use std::collections::HashMap;
use std::process::Command;

/// Coordinates execution across multiple substrates in parallel
pub struct MultiSubstrateCoordinator {
    ruby_state: HashMap<String, serde_json::Value>,
    brain_simulator: BrainSimulator,
    shared_memory: HashMap<String, serde_json::Value>,
    verbose: bool,
}

impl MultiSubstrateCoordinator {
    pub fn new() -> Self {
        Self {
            ruby_state: HashMap::new(),
            brain_simulator: BrainSimulator::new(),
            shared_memory: HashMap::new(),
            verbose: false,
        }
    }

    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self.brain_simulator = self.brain_simulator.with_verbose(verbose);
        self
    }

    pub fn execute(&mut self, program: &Program) -> Result<()> {
        if self.verbose {
            println!("🌐 Multi-Substrate Parallel Execution Engine");
            println!();
        }

        // Separate actions by actor (substrate)
        let mut ruby_actions = Vec::new();
        let mut brain_actions = Vec::new();
        let mut coordinator_actions = Vec::new();

        for action in &program.actions {
            match action.actor.as_str() {
                "RubyVM" => ruby_actions.push(action),
                "BrainVM" => brain_actions.push(action),
                "Coordinator" => coordinator_actions.push(action),
                _ => brain_actions.push(action), // Default to brain
            }
        }

        if self.verbose {
            println!("📊 Execution Plan:");
            println!("   💎 Ruby VM: {} operations", ruby_actions.len());
            println!("   🧠 Brain VM: {} operations", brain_actions.len());
            println!("   🌐 Coordinator: {} operations", coordinator_actions.len());
            println!();
        }

        // Execute in original order, switching substrates as needed
        let mut current_substrate = "";

        for action in &program.actions {
            let substrate = action.actor.as_str();

            if substrate != current_substrate {
                if self.verbose && !current_substrate.is_empty() {
                    println!();
                }
                current_substrate = substrate;
            }

            match substrate {
                "RubyVM" => self.execute_ruby_action(action)?,
                "BrainVM" => self.execute_brain_action(action)?,
                "Coordinator" => self.execute_coordinator_action(action)?,
                _ => self.execute_brain_action(action)?,
            }
        }

        Ok(())
    }

    fn execute_ruby_action(&mut self, action: &Action) -> Result<()> {
        if self.verbose {
            println!("💎 Ruby VM: {:?} → {}", action.op, action.target);
        }

        // Build a mini program for this action
        let program = Program {
            metadata: None,
            actions: vec![action.clone()],
        };

        let mut compiler = RubyCompiler::new();
        let code = compiler.compile(&program)?;

        // Execute and capture the result
        let output = Command::new("ruby")
            .arg("-e")
            .arg(&code)
            .output()?;

        if !output.stdout.is_empty() {
            let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Store in ruby state
            if let Ok(num) = result_str.parse::<f64>() {
                self.ruby_state.insert(action.target.clone(), serde_json::json!(num));

                if self.verbose {
                    println!("   ✓ Result: {} = {}", action.target, num);
                }
            } else if !result_str.is_empty() {
                self.ruby_state.insert(action.target.clone(), serde_json::json!(result_str));

                if self.verbose {
                    println!("   ✓ Output: {}", result_str);
                }
            }
        }

        Ok(())
    }

    fn execute_brain_action(&mut self, action: &Action) -> Result<()> {
        if self.verbose {
            println!("🧠 Brain VM: {:?} → {}", action.op, action.target);
        }

        let program = Program {
            metadata: None,
            actions: vec![action.clone()],
        };

        self.brain_simulator.execute(&program)?;

        // Check if brain generated output or stored a value
        let brain_state = self.brain_simulator.state();

        if let Some(value) = brain_state.beliefs.get(&action.target) {
            if self.verbose {
                println!("   ✓ Brain stored: {} = {}", action.target, value);
            }
        }

        Ok(())
    }

    fn execute_coordinator_action(&mut self, action: &Action) -> Result<()> {
        if self.verbose {
            println!("🌐 Coordinator: {:?} → {}", action.op, action.target);
        }

        match &action.op {
            Operation::Receive => {
                // Receive data from a substrate
                if let Some(params) = &action.params {
                    if let Some(source) = params.get("source").and_then(|v| v.as_str()) {
                        if source == "BrainVM" {
                            // Get value from brain
                            if let Some(value) = self.brain_simulator.state().beliefs.get(&action.target) {
                                self.shared_memory.insert(action.target.clone(), value.clone());

                                if self.verbose {
                                    println!("   📨 Received from Brain: {} = {}", action.target, value);
                                }
                            }
                        } else if source == "RubyVM" {
                            // Get value from ruby
                            if let Some(value) = self.ruby_state.get(&action.target) {
                                self.shared_memory.insert(action.target.clone(), value.clone());

                                if self.verbose {
                                    println!("   📨 Received from Ruby: {} = {}", action.target, value);
                                }
                            }
                        }
                    }
                }
            }
            Operation::Emit => {
                // Send data to a substrate
                if let Some(params) = &action.params {
                    if let Some(dest) = params.get("destination").and_then(|v| v.as_str()) {
                        if let Some(value) = self.shared_memory.get(&action.target) {
                            if dest == "RubyVM" {
                                self.ruby_state.insert(action.target.clone(), value.clone());

                                if self.verbose {
                                    println!("   📤 Sent to Ruby: {} = {}", action.target, value);
                                }
                            } else if dest == "BrainVM" {
                                // Store in brain's beliefs
                                if self.verbose {
                                    println!("   📤 Sent to Brain: {} = {}", action.target, value);
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                if self.verbose {
                    println!("   ⚠️  Unsupported coordinator operation");
                }
            }
        }

        Ok(())
    }

    pub fn show_results(&self) {
        println!("\n📊 Final State Across All Substrates:");
        println!("{}", "─".repeat(60));

        if !self.ruby_state.is_empty() {
            println!("\n💎 Ruby VM State:");
            for (key, value) in &self.ruby_state {
                println!("   {} = {}", key, value);
            }
        }

        println!("\n🧠 Brain VM State:");
        let brain_state = self.brain_simulator.state();

        if !brain_state.beliefs.is_empty() {
            println!("   Beliefs:");
            for (key, value) in &brain_state.beliefs {
                println!("     {} = {}", key, value);
            }
        }

        if !brain_state.thoughts.is_empty() {
            println!("   Thoughts:");
            for thought in &brain_state.thoughts {
                println!("     💭 {}", thought);
            }
        }

        if !brain_state.output.is_empty() {
            println!("   Output:");
            for out in &brain_state.output {
                println!("     🗣️  {}", out);
            }
        }

        if !self.shared_memory.is_empty() {
            println!("\n🌐 Shared Memory:");
            for (key, value) in &self.shared_memory {
                println!("   {} = {}", key, value);
            }
        }
    }
}

impl Default for MultiSubstrateCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

