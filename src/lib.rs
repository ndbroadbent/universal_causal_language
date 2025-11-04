use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod compiler;
pub mod simulator;

/// Core operation types in UCL
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operation {
    // Basic CRUD operations
    Create,
    Read,
    Write,
    Delete,

    // Binding operations
    Bind,
    Unbind,

    // Communication operations
    Emit,
    Receive,

    // Observation and decision
    Measure,
    Decide,

    // Temporal operations
    Wait,

    // Logical/semantic operations
    Assert,
    StoreFact,

    // Legal/obligation operations
    Oblige,
    Permit,
    Remedy,

    // Biological operations
    Transcribe,
    Translate,
    Express,

    // Programming operations
    Call,
    Assign,
    Return,

    // Custom operation for extensibility
    Custom(String),

    // Intentionally unsupported operations - for testing comprehension limits
    Flurble,  // A nonsense operation
    Grok,     // Deep understanding (not yet implemented)
    Defenestrate,  // A real word but intentionally not supported
}

/// A UCL Action represents a single causal event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Who or what initiates the cause
    pub actor: String,

    /// What kind of action occurs
    pub op: Operation,

    /// What is acted upon
    pub target: String,

    /// When the action occurs (optional, can be relative or absolute)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<f64>,

    /// How long it lasts (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dur: Option<f64>,

    /// Contextual arguments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, serde_json::Value>>,

    /// Required preconditions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre: Option<String>,

    /// Resulting conditions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<String>,

    /// Domain tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<String>>,
}

/// A UCL program is a sequence of actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    /// Optional program metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// The sequence of actions
    pub actions: Vec<Action>,
}

impl Action {
    /// Create a new Action with minimal fields
    pub fn new(actor: impl Into<String>, op: Operation, target: impl Into<String>) -> Self {
        Self {
            actor: actor.into(),
            op,
            target: target.into(),
            t: None,
            dur: None,
            params: None,
            pre: None,
            post: None,
            effects: None,
        }
    }

    /// Builder method to add timing
    pub fn with_time(mut self, t: f64) -> Self {
        self.t = Some(t);
        self
    }

    /// Builder method to add duration
    pub fn with_duration(mut self, dur: f64) -> Self {
        self.dur = Some(dur);
        self
    }

    /// Builder method to add parameters
    pub fn with_params(mut self, params: HashMap<String, serde_json::Value>) -> Self {
        self.params = Some(params);
        self
    }

    /// Builder method to add effects
    pub fn with_effects(mut self, effects: Vec<String>) -> Self {
        self.effects = Some(effects);
        self
    }
}

impl Program {
    /// Create a new empty program
    pub fn new() -> Self {
        Self {
            metadata: None,
            actions: Vec::new(),
        }
    }

    /// Add an action to the program
    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    /// Parse a UCL program from JSON
    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(json)?)
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Parse a single action from JSON
    pub fn parse_action(json: &str) -> anyhow::Result<Action> {
        Ok(serde_json::from_str(json)?)
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_creation() {
        let action = Action::new("VM", Operation::Call, "Add")
            .with_time(0.0)
            .with_effects(vec!["CPU".to_string()]);

        assert_eq!(action.actor, "VM");
        assert_eq!(action.op, Operation::Call);
        assert_eq!(action.target, "Add");
        assert_eq!(action.t, Some(0.0));
    }

    #[test]
    fn test_json_serialization() {
        let mut params = HashMap::new();
        params.insert("color".to_string(), serde_json::json!("black"));

        let action = Action::new("listener_brain", Operation::StoreFact, "cat")
            .with_params(params);

        let json = serde_json::to_string(&action).unwrap();
        let parsed: Action = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.actor, "listener_brain");
        assert_eq!(parsed.target, "cat");
    }

    #[test]
    fn test_program_creation() {
        let mut program = Program::new();
        program.add_action(Action::new("test", Operation::Create, "object"));

        assert_eq!(program.actions.len(), 1);

        let json = program.to_json().unwrap();
        let parsed = Program::from_json(&json).unwrap();
        assert_eq!(parsed.actions.len(), 1);
    }
}

