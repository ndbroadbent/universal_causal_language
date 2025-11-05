use crate::{Action, Operation, Program};
use anyhow::Result;
use std::collections::HashMap;

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
}

impl RobotSimulator {
    pub fn new() -> Self {
        Self {
            state: RobotState::new(),
            verbose: false,
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
        match &action.op {
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
}

impl Default for RobotSimulator {
    fn default() -> Self {
        Self::new()
    }
}

