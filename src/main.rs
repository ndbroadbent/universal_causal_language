use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use ucl::{Program, Operation, compiler::RubyCompiler, simulator::{BrainSimulator, RobotSimulator}, coordinator::MultiSubstrateCoordinator};

#[derive(Parser)]
#[command(name = "ucl")]
#[command(about = "Universal Causal Language CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a UCL file
    Validate {
        /// Path to the UCL file
        file: PathBuf,
    },

    /// Display a UCL file in human-readable format
    Display {
        /// Path to the UCL file
        file: PathBuf,

        /// Show compact output
        #[arg(short, long)]
        compact: bool,
    },

    /// Convert a UCL file to a different format
    Convert {
        /// Path to the UCL file
        file: PathBuf,

        /// Output format (currently only json)
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Analyze a UCL program
    Analyze {
        /// Path to the UCL file
        file: PathBuf,
    },

    /// Compile a UCL program to another language
    Compile {
        /// Path to the UCL file
        file: PathBuf,

        /// Target language (currently only ruby)
        #[arg(short, long, default_value = "ruby")]
        target: String,

        /// Output file (optional, defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Compile and run a UCL program
    Run {
        /// Path to the UCL file
        file: PathBuf,

        /// Target language (ruby or brain)
        #[arg(short, long, default_value = "ruby")]
        target: String,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Simulate execution on a virtual human brain
    Brain {
        /// Path to the UCL file
        file: PathBuf,

        /// Verbose output showing each cognitive operation
        #[arg(short, long)]
        verbose: bool,

        /// Run on production (your actual brain) instead of simulated brain
        #[arg(short, long)]
        production: bool,
    },

    /// Simulate execution on a virtual robot
    Robot {
        /// Path to the UCL file
        file: PathBuf,

        /// Verbose output showing each physical operation
        #[arg(short, long)]
        verbose: bool,
    },

    /// Execute across multiple substrates in parallel
    Parallel {
        /// Path to the UCL file
        file: PathBuf,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Validate { file } => {
            match validate_file(file) {
                Ok(_) => {
                    println!("✓ Valid UCL program");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("✗ Validation error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Display { file, compact } => {
            match display_file(file, *compact) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Convert { file, format } => {
            match convert_file(file, format) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Analyze { file } => {
            match analyze_file(file) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Compile { file, target, output } => {
            match compile_file(file, target, output.as_ref()) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Run { file, target, verbose } => {
            match run_file(file, target, *verbose) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Brain { file, verbose, production } => {
            match brain_simulate(file, *verbose, *production) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Robot { file, verbose } => {
            match robot_simulate(file, *verbose) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Parallel { file, verbose } => {
            match parallel_execute(file, *verbose) {
                Ok(_) => std::process::exit(0),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}

fn validate_file(path: &PathBuf) -> anyhow::Result<Program> {
    let content = fs::read_to_string(path)?;
    let program = Program::from_json(&content)?;
    Ok(program)
}

fn display_file(path: &PathBuf, compact: bool) -> anyhow::Result<()> {
    let program = validate_file(path)?;

    if compact {
        println!("{}", serde_json::to_string(&program)?);
    } else {
        if let Some(metadata) = &program.metadata {
            println!("=== Metadata ===");
            for (key, value) in metadata {
                println!("  {}: {}", key, value);
            }
            println!();
        }

        println!("=== Actions ({}) ===", program.actions.len());
        for (i, action) in program.actions.iter().enumerate() {
            println!("\n[{}] {:?}", i, action.op);
            println!("  Actor:  {}", action.actor);
            println!("  Target: {}", action.target);

            if let Some(t) = action.t {
                println!("  Time:   {}", t);
            }

            if let Some(dur) = action.dur {
                println!("  Duration: {}", dur);
            }

            if let Some(params) = &action.params {
                println!("  Parameters:");
                for (key, value) in params {
                    println!("    {}: {}", key, value);
                }
            }

            if let Some(effects) = &action.effects {
                println!("  Effects: [{}]", effects.join(", "));
            }
        }
    }

    Ok(())
}

fn convert_file(path: &PathBuf, format: &str) -> anyhow::Result<()> {
    let program = validate_file(path)?;

    match format {
        "json" => {
            println!("{}", program.to_json()?);
        }
        _ => {
            anyhow::bail!("Unsupported format: {}. Currently only 'json' is supported.", format);
        }
    }

    Ok(())
}

fn analyze_file(path: &PathBuf) -> anyhow::Result<()> {
    let program = validate_file(path)?;

    println!("=== UCL Program Analysis ===\n");
    println!("Total actions: {}", program.actions.len());

    // Count operations
    let mut op_counts = std::collections::HashMap::new();
    for action in &program.actions {
        *op_counts.entry(format!("{:?}", action.op)).or_insert(0) += 1;
    }

    println!("\nOperation distribution:");
    let mut ops: Vec<_> = op_counts.iter().collect();
    ops.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    for (op, count) in ops {
        println!("  {}: {}", op, count);
    }

    // Count actors
    let mut actor_counts = std::collections::HashMap::new();
    for action in &program.actions {
        *actor_counts.entry(&action.actor).or_insert(0) += 1;
    }

    println!("\nTop actors:");
    let mut actors: Vec<_> = actor_counts.iter().collect();
    actors.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    for (actor, count) in actors.iter().take(10) {
        println!("  {}: {}", actor, count);
    }

    // Effects domains
    let mut domain_counts = std::collections::HashMap::new();
    for action in &program.actions {
        if let Some(effects) = &action.effects {
            for effect in effects {
                *domain_counts.entry(effect).or_insert(0) += 1;
            }
        }
    }

    if !domain_counts.is_empty() {
        println!("\nDomain tags:");
        for (domain, count) in domain_counts.iter() {
            println!("  {}: {}", domain, count);
        }
    }

    // Temporal analysis
    let timed_actions = program.actions.iter().filter(|a| a.t.is_some()).count();
    if timed_actions > 0 {
        println!("\nTemporal analysis:");
        println!("  Actions with timestamps: {}", timed_actions);

        let times: Vec<f64> = program.actions.iter().filter_map(|a| a.t).collect();
        if !times.is_empty() {
            let min = times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = times.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            println!("  Time range: {} to {}", min, max);
        }
    }

    Ok(())
}

fn compile_file(path: &PathBuf, target: &str, output: Option<&PathBuf>) -> anyhow::Result<()> {
    let program = validate_file(path)?;

    let code = match target {
        "ruby" => {
            let mut compiler = RubyCompiler::new();
            compiler.compile(&program)?
        }
        _ => {
            anyhow::bail!("Unsupported target language: {}. Currently only 'ruby' is supported.", target);
        }
    };

    if let Some(output_path) = output {
        fs::write(output_path, code)?;
        println!("Compiled to {}", output_path.display());
    } else {
        println!("{}", code);
    }

    Ok(())
}

fn run_file(path: &PathBuf, target: &str, verbose: bool) -> anyhow::Result<()> {
    let program = validate_file(path)?;

    match target {
        "brain" => {
            let mut simulator = BrainSimulator::new().with_verbose(verbose);
            simulator.execute(&program)?;

            println!("\n{}", simulator.state().display());
        }
        "ruby" => {
            let mut compiler = RubyCompiler::new();
            let code = compiler.compile(&program)?;

            // Check if ruby is available
            let ruby_check = Command::new("ruby")
                .arg("--version")
                .output();

            if ruby_check.is_err() {
                anyhow::bail!("Ruby is not installed or not in PATH. Please install Ruby to run UCL programs.");
            }

            println!("=== Compiled Ruby Code ===");
            println!("{}", code);
            println!("\n=== Execution Output ===");

            // Execute the Ruby code
            let output = Command::new("ruby")
                .arg("-e")
                .arg(&code)
                .output()?;

            if !output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }

            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }

            if !output.status.success() {
                anyhow::bail!("Ruby execution failed with status: {}", output.status);
            }
        }
        _ => {
            anyhow::bail!("Unsupported target language: {}. Currently 'ruby' and 'brain' are supported.", target);
        }
    }

    Ok(())
}

fn brain_simulate(path: &PathBuf, verbose: bool, production: bool) -> anyhow::Result<()> {
    let program = validate_file(path)?;

    if production {
        return run_on_production_brain(&program);
    }

    let mut simulator = BrainSimulator::new().with_verbose(verbose);

    println!("🧠 Simulating language execution on virtual human brain...\n");

    simulator.execute(&program)?;

    println!("\n{}", simulator.state().display());

    if !simulator.state().trace.is_empty() {
        println!("Execution Trace:");
        for (i, step) in simulator.state().trace.iter().enumerate() {
            println!("  {}. {}", i + 1, step);
        }
    }

    Ok(())
}

fn robot_simulate(path: &PathBuf, verbose: bool) -> anyhow::Result<()> {
    let program = validate_file(path)?;

    let mut simulator = RobotSimulator::new().with_verbose(verbose);

    println!("🤖 Simulating physical execution on virtual robot...\n");

    simulator.execute(&program)?;

    println!("\n{}", simulator.state().display());

    Ok(())
}

fn run_on_production_brain(program: &Program) -> anyhow::Result<()> {
    use std::io::{self, Write};

    println!("🧠💼 PRODUCTION MODE: Running on YOUR actual brain!");
    println!("{}", "=".repeat(60));
    println!();
    println!("⚠️  WARNING: This will execute directly on human wetware.");
    println!("    No virtual machine. No sandbox. Just your neurons.");
    println!();
    println!("Instructions:");
    println!("  - Read each operation carefully");
    println!("  - Execute it using your brain");
    println!("  - Report your internal state after each step");
    println!();
    print!("Ready to begin? (y/n): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        println!("\n❌ Aborted. Your brain remains in its current state.");
        return Ok(());
    }

    println!("\n🚀 Initiating brain program execution...\n");

    let mut execution_log = Vec::new();
    let start_time = std::time::Instant::now();

    for (i, action) in program.actions.iter().enumerate() {
        println!("{}", "─".repeat(60));
        println!("STEP {}/{}: {:?} Operation", i + 1, program.actions.len(), action.op);
        println!("{}", "─".repeat(60));
        println!();
        println!("📋 Instruction:");
        println!("   Actor:  {}", action.actor);
        println!("   Op:     {:?}", action.op);
        println!("   Target: {}", action.target);

        if let Some(params) = &action.params {
            println!("   Params:");
            for (key, value) in params {
                println!("     • {} = {}", key, value);
            }
        }

        if let Some(effects) = &action.effects {
            println!("   Effects: [{}]", effects.join(", "));
        }

        println!();
        println!("🎯 Your Task:");

        match action.op {
            Operation::StoreFact => {
                println!("   → Store this information in your memory");
                println!("   → Commit '{}' to long-term storage", action.target);
            }
            Operation::Assert => {
                println!("   → Assert this as a strong belief");
                println!("   → Make this a core part of your worldview");
            }
            Operation::Emit => {
                println!("   → Generate and speak this output");
                println!("   → Say it out loud or in your mind");
            }
            Operation::Receive => {
                println!("   → Process this input through your senses");
                println!("   → Pay attention to: {}", action.target);
            }
            Operation::Decide => {
                println!("   → Make this decision");
                println!("   → Commit to: {}", action.target);
            }
            Operation::Measure => {
                println!("   → Observe and measure");
                println!("   → Focus your attention on: {}", action.target);
            }
            Operation::Create => {
                println!("   → Create this new concept in your mind");
                println!("   → Imagine: {}", action.target);
            }
            Operation::Bind => {
                println!("   → Bind this concept to a mental variable");
                println!("   → Associate '{}' with a value", action.target);
            }
            Operation::Write => {
                if let Some(params) = &action.params {
                    if params.contains_key("lhs_register") && params.contains_key("rhs_register") {
                        let lhs = params.get("lhs_register").and_then(|v| v.as_str()).unwrap_or("?");
                        let rhs = params.get("rhs_register").and_then(|v| v.as_str()).unwrap_or("?");
                        let op = params.get("operation").and_then(|v| v.as_str()).unwrap_or("multiply");
                        let symbol = match op {
                            "multiply" => "×",
                            "add" => "+",
                            "subtract" => "-",
                            "divide" => "÷",
                            _ => "×"
                        };
                        println!("   → Recall {} and {}", lhs, rhs);
                        println!("   → Calculate: {} {} {}", lhs, symbol, rhs);
                        println!("   → Store the answer in: {}", action.target);
                    } else {
                        println!("   → Update memory: {}", action.target);
                        println!("   → Store a new value");
                    }
                } else {
                    println!("   → Update memory: {}", action.target);
                }
            }
            Operation::Oblige => {
                println!("   → Accept this obligation");
                println!("   → Add to your active goals");
            }
            Operation::Wait => {
                let duration = action.dur.unwrap_or(1.0);
                println!("   → Wait and let {} seconds pass", duration);
                println!("   → Be present in this moment");
            }
            Operation::GenRandomInt => {
                if let Some(params) = &action.params {
                    let min = params.get("min").and_then(|v| v.as_i64()).unwrap_or(0);
                    let max = params.get("max").and_then(|v| v.as_i64()).unwrap_or(9);
                    println!("   → Think of a random number between {} and {}", min, max);
                    println!("   → Remember it as '{}'", action.target);
                } else {
                    println!("   → Generate a random number");
                    println!("   → Remember it as '{}'", action.target);
                }
            }
            _ => {
                println!("   ⚠️  UNKNOWN OPERATION!");
                println!("   → Experience confusion");
                println!("   → Notice you don't understand");
            }
        }

        println!();
        print!("✅ Press ENTER when you've executed this operation...");
        io::stdout().flush()?;

        let mut _dummy = String::new();
        io::stdin().read_line(&mut _dummy)?;

        // Ask for state report
        println!();
        println!("📊 Post-Execution Report:");
        println!();

        print!("What are you thinking right now? ");
        io::stdout().flush()?;
        let mut thought = String::new();
        io::stdin().read_line(&mut thought)?;

        print!("How do you feel? (emotion): ");
        io::stdout().flush()?;
        let mut emotion = String::new();
        io::stdin().read_line(&mut emotion)?;

        print!("What do you remember? ");
        io::stdout().flush()?;
        let mut memory = String::new();
        io::stdin().read_line(&mut memory)?;

        execution_log.push(format!(
            "Step {}: {:?}({})\n  Thought: {}\n  Emotion: {}\n  Memory: {}",
            i + 1,
            action.op,
            action.target,
            thought.trim(),
            emotion.trim(),
            memory.trim()
        ));

        println!("\n✓ Step {} complete. Brain state updated.\n", i + 1);
    }

    let elapsed = start_time.elapsed();

    println!("\n");
    println!("🎉 PROGRAM EXECUTION COMPLETE 🎉");
    println!("{}", "=".repeat(60));
    println!();
    println!("📈 Performance Metrics:");
    println!("   Total Operations: {}", program.actions.len());
    println!("   Execution Time: {:.2?}", elapsed);
    println!("   Avg Time/Op: {:.2?}", elapsed / program.actions.len() as u32);
    println!();
    println!("🧠 Production Brain State Capture:");
    println!("{}", "─".repeat(60));

    for log in &execution_log {
        println!("{}", log);
        println!();
    }

    println!("{}", "=".repeat(60));
    println!();
    println!("💡 Insights:");
    println!("   • Your brain successfully executed {} UCL operations", program.actions.len());
    println!("   • Language literally ran as a program on your neurons");
    println!("   • You are now running UCL in production 🚀");
    println!();
    println!("Thank you for being a biological runtime environment! 🧠✨");

    Ok(())
}

fn parallel_execute(path: &PathBuf, verbose: bool) -> anyhow::Result<()> {
    let program = validate_file(path)?;

    println!("🌐 Multi-Substrate Parallel Execution");
    println!("{}", "=".repeat(60));
    println!();

    let mut coordinator = MultiSubstrateCoordinator::new().with_verbose(verbose);
    coordinator.execute(&program)?;

    coordinator.show_results();

    println!("\n{}", "=".repeat(60));
    println!("✨ Parallel execution complete!");
    println!("\n💡 Different substrates (Silicon + Wetware) worked together");
    println!("   on the same problem. This is the future of computing. 🚀");

    Ok(())
}

