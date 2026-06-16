use eshu::{Cli, CliFlag, StoreSyntax, StoreType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_flag = CliFlag::new("output")
        .with_flag_char('o')
        .with_about("Output target file", "Specify a path where the generated ROFF file should be written.")
        .with_store(StoreType::Value, StoreSyntax::Detached)
        .build()?;

    let cli = Cli::new("man-gen")
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_about("Eshu ROFF Man-page Exporter")
        .with_author_and_publish_date("Your Name <you@example.com>".to_string(), "2026-06-16".to_string())
        .add_flag(output_flag)
        .parse();

    let manpage = cli.make_manpage();

    if let Some(store) = cli.get_flag_store("output") {
        if let Some(values) = store.as_value() {
            if let Some(filepath) = values.first() {
                std::fs::write(filepath, &manpage)?;
                println!("ROFF man-page written successfully to {}", filepath);
                println!("To view it, run: man -l {}", filepath);
                return Ok(());
            }
        }
    }

    println!("{}", manpage);
    Ok(())
}
