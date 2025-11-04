use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use ucl::{Program, compiler::RubyCompiler, simulator::BrainSimulator};

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

        Commands::Brain { file, verbose } => {
            match brain_simulate(file, *verbose) {
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

fn brain_simulate(path: &PathBuf, verbose: bool) -> anyhow::Result<()> {
    let program = validate_file(path)?;

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

