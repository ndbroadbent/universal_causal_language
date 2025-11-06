use ucl::Program;
use std::fs;

#[test]
fn test_natural_language_example() {
    let content = fs::read_to_string("examples/natural_language.json")
        .expect("Failed to read natural_language.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse natural_language.json");

    assert_eq!(program.actions.len(), 5);
    assert_eq!(program.actions[0].actor, "listener");
    assert_eq!(program.actions[0].target, "memory");
}

#[test]
fn test_ruby_code_example() {
    let content = fs::read_to_string("examples/ruby_code.json")
        .expect("Failed to read ruby_code.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse ruby_code.json");

    assert_eq!(program.actions.len(), 5);
    assert!(program.metadata.is_some());

    let metadata = program.metadata.as_ref().unwrap();
    assert_eq!(metadata.get("language").unwrap().as_str().unwrap(), "Ruby");
}

#[test]
fn test_rust_code_example() {
    let content = fs::read_to_string("examples/rust_code.json")
        .expect("Failed to read rust_code.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse rust_code.json");

    assert_eq!(program.actions.len(), 9);

    let metadata = program.metadata.as_ref().unwrap();
    assert_eq!(metadata.get("language").unwrap().as_str().unwrap(), "Rust");
}

#[test]
fn test_music_example() {
    let content = fs::read_to_string("examples/music.json")
        .expect("Failed to read music.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse music.json");

    // C major scale has 8 notes
    assert_eq!(program.actions.len(), 8);

    // All actions should have timing
    for action in &program.actions {
        assert!(action.t.is_some());
        assert!(action.dur.is_some());
    }

    // First note should be C4
    if let Some(params) = &program.actions[0].params {
        assert_eq!(params.get("pitch").unwrap().as_str().unwrap(), "C4");
    }
}

#[test]
fn test_legal_contract_example() {
    let content = fs::read_to_string("examples/legal_contract.json")
        .expect("Failed to read legal_contract.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse legal_contract.json");

    assert_eq!(program.actions.len(), 5);

    // All actions should have Legal effect
    for action in &program.actions {
        assert!(action.effects.is_some());
        let effects = action.effects.as_ref().unwrap();
        assert!(effects.contains(&"Legal".to_string()));
    }
}

#[test]
fn test_biology_example() {
    let content = fs::read_to_string("examples/biology.json")
        .expect("Failed to read biology.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse biology.json");

    assert_eq!(program.actions.len(), 7);

    let metadata = program.metadata.as_ref().unwrap();
    assert_eq!(metadata.get("gene").unwrap().as_str().unwrap(), "MYC");
}

#[test]
fn test_fibonacci_example() {
    let content = fs::read_to_string("examples/fibonacci.json")
        .expect("Failed to read fibonacci.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse fibonacci.json");

    // Should have 3 actions: DefineFunction, Emit, For
    assert_eq!(program.actions.len(), 3);

    // First action should be DefineFunction
    assert!(matches!(program.actions[0].op, ucl::Operation::DefineFunction));
    assert_eq!(program.actions[0].target, "fibonacci");

    // Should have function definition with args and body
    assert!(program.actions[0].params.is_some());
    let params = program.actions[0].params.as_ref().unwrap();
    assert!(params.contains_key("args"));
    assert!(params.contains_key("body"));
}

#[test]
fn test_fibonacci_compiles_and_runs() {
    use std::process::Command;

    let content = fs::read_to_string("examples/fibonacci.json")
        .expect("Failed to read fibonacci.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse fibonacci.json");

    // Compile to Ruby
    let mut compiler = ucl::compiler::RubyCompiler::new();
    let ruby_code = compiler.compile(&program)
        .expect("Failed to compile fibonacci");

    // Verify it contains the key components
    assert!(ruby_code.contains("def fibonacci(n)"));
    assert!(ruby_code.contains("if n <= 1"));
    assert!(ruby_code.contains("return"));
    assert!(ruby_code.contains("fibonacci((n - 1))"));
    assert!(ruby_code.contains("fibonacci((n - 2))"));

    // Execute with Ruby and verify output
    let output = Command::new("ruby")
        .arg("-e")
        .arg(&ruby_code)
        .output()
        .expect("Failed to execute Ruby");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should contain the Fibonacci sequence
    assert!(stdout.contains("0"));
    assert!(stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(stdout.contains("3"));
    assert!(stdout.contains("5"));
    assert!(stdout.contains("8"));
    assert!(stdout.contains("13"));
    assert!(stdout.contains("55"));
}

#[test]
fn test_ai_generate_factorial() {
    let content = fs::read_to_string("examples/ai_generate_factorial.json")
        .expect("Failed to read ai_generate_factorial.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse ai_generate_factorial.json");

    // Should have 2 actions: Generate and Emit
    assert_eq!(program.actions.len(), 2);

    // First action should be Generate
    assert!(matches!(program.actions[0].op, ucl::Operation::Generate));
    assert_eq!(program.actions[0].actor, "ai_agent");

    // Should have instruction parameter
    let params = program.actions[0].params.as_ref().unwrap();
    assert!(params.contains_key("instruction"));
}

#[test]
fn test_mock_ai_generates_code() {
    use ucl::simulator::MockAISimulator;

    let content = fs::read_to_string("examples/ai_generate_factorial.json")
        .expect("Failed to read ai_generate_factorial.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse ai_generate_factorial.json");

    // Execute on Mock AI
    let mut ai = MockAISimulator::new();
    ai.execute(&program).expect("Failed to execute on Mock AI");

    // Verify AI generated code
    assert_eq!(ai.state().prompts.len(), 1);
    assert!(ai.state().prompts[0].contains("factorial"));

    // Verify code was generated
    assert!(ai.state().generated_code.contains_key("factorial_code"));
    let generated = ai.state().generated_code.get("factorial_code").unwrap();
    assert_eq!(generated.len(), 1);
    assert!(matches!(generated[0].op, ucl::Operation::DefineFunction));
}

#[test]
fn test_all_examples_roundtrip() {
    let examples = [
        "examples/natural_language.json",
        "examples/ruby_code.json",
        "examples/rust_code.json",
        "examples/music.json",
        "examples/legal_contract.json",
        "examples/biology.json",
        "examples/fibonacci.json",
        "examples/recipe_tea.json",
        "examples/ai_generate_factorial.json",
        "examples/ai_chain.json",
    ];

    for example in &examples {
        let content = fs::read_to_string(example)
            .unwrap_or_else(|_| panic!("Failed to read {}", example));

        let program = Program::from_json(&content)
            .unwrap_or_else(|_| panic!("Failed to parse {}", example));

        let json = program.to_json()
            .unwrap_or_else(|_| panic!("Failed to serialize {}", example));

        let reparsed = Program::from_json(&json)
            .unwrap_or_else(|_| panic!("Failed to reparse {}", example));

        assert_eq!(program.actions.len(), reparsed.actions.len());
    }
}

