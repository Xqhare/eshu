use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Build individual flags
    let verbose_flag = CliFlag::new("verbose")
        .with_flag_char('v')
        .with_about("Enable verbose output", "Print detailed execution logs.")
        .build()?;

    let output_flag = CliFlag::new("output")
        .with_flag_char('o')
        .with_about("Output file path", "Specify the path where results will be saved.")
        .with_store(StoreType::Value, StoreSyntax::Detached)
        .build()?;

    let define_flag = CliFlag::new("define")
        .with_flag_char('D')
        .with_about("Define variables", "Define config options using key=value pairs.")
        .with_store(StoreType::KeyValue, StoreSyntax::Detached)
        .build()?;

    // 2. Build the main CLI interface
    let cli = Cli::new("copy-cat")
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_about("A simple file processor utility using eshu")
        .add_flag(verbose_flag)
        .add_flag(output_flag)
        .add_flag(define_flag)
        .parse();

    // 3. Retrieve parsed options
    if cli.is_flag_entered("verbose") {
        println!("Verbose mode is enabled!");
    }

    if let Some(store) = cli.get_flag_store("output") {
        if let Some(values) = store.as_value() {
            println!("Output targets: {:?}", values);
        }
    }

    if let Some(store) = cli.get_flag_store("define") {
        if let Some(kv) = store.as_key_value() {
            for (key, val) in kv {
                println!("Defined: {} = {}", key, val);
            }
        }
    }

    // Retrieve positional arguments
    let positionals = cli.get_stray_positional_args();
    if !positionals.is_empty() {
        println!("Positional arguments: {:?}", positionals);
    } else {
        println!("No positional arguments provided.");
    }

    Ok(())
}
