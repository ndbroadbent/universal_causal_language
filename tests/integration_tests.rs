use ucl::Program;
use std::fs;

#[test]
fn test_natural_language_example() {
    let content = fs::read_to_string("examples/natural_language.json")
        .expect("Failed to read natural_language.json");
    let program = Program::from_json(&content)
        .expect("Failed to parse natural_language.json");

    assert_eq!(program.actions.len(), 5);
    assert_eq!(program.actions[0].actor, "listener_brain");
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
fn test_all_examples_roundtrip() {
    let examples = [
        "examples/natural_language.json",
        "examples/ruby_code.json",
        "examples/rust_code.json",
        "examples/music.json",
        "examples/legal_contract.json",
        "examples/biology.json",
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

