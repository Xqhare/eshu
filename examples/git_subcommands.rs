use eshu::{Cli, CliCommand, CliFlag, StoreSyntax, StoreType};
use std::rc::Rc;

struct AddCommand {
    flags: Vec<CliFlag>,
}

impl AddCommand {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let dry_run = CliFlag::new("dry-run")
            .with_flag_char('n')
            .with_about("Perform a dry run", "Show files that would be added without actually doing it.")
            .build()?;
        Ok(Self { flags: vec![dry_run] })
    }
}

impl CliCommand<'static> for AddCommand {
    fn name(&self) -> String {
        "add".to_string()
    }

    fn short_about(&self) -> String {
        "Add file contents to the index".to_string()
    }

    fn long_about(&self) -> String {
        "Add files or patterns to the staging area to prepare them for commit.".to_string()
    }

    fn flags(&self) -> &Vec<CliFlag> {
        &self.flags
    }

    fn subcommands(&self) -> Vec<Rc<dyn CliCommand<'static>>> {
        vec![]
    }

    fn execute(&self, cli: &Cli<'static>) {
        let positionals = cli.get_stray_positional_args();
        let dry_run = cli.is_flag_entered("dry-run");

        if dry_run {
            println!("Dry-run: Preparing to add files: {:?}", positionals);
        } else {
            println!("Successfully added files: {:?}", positionals);
        }
    }
}

struct CommitCommand {
    flags: Vec<CliFlag>,
}

impl CommitCommand {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let message = CliFlag::new("message")
            .with_flag_char('m')
            .with_about("Commit message", "Use the given message as the commit message.")
            .with_required_store(StoreType::Value, StoreSyntax::Detached)
            .build()?;
        Ok(Self { flags: vec![message] })
    }
}

impl CliCommand<'static> for CommitCommand {
    fn name(&self) -> String {
        "commit".to_string()
    }

    fn short_about(&self) -> String {
        "Record changes to the repository".to_string()
    }

    fn long_about(&self) -> String {
        "Create a new commit containing the current contents of the index and the log message.".to_string()
    }

    fn flags(&self) -> &Vec<CliFlag> {
        &self.flags
    }

    fn subcommands(&self) -> Vec<Rc<dyn CliCommand<'static>>> {
        vec![]
    }

    fn execute(&self, cli: &Cli<'static>) {
        if let Some(store) = cli.get_flag_store("message") {
            if let Some(msgs) = store.as_value() {
                if let Some(m) = msgs.first() {
                    println!("[master] Committed changes: '{}'", m);
                    return;
                }
            }
        }
        println!("Error: Commit message required.");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let add_cmd = Rc::new(AddCommand::new()?);
    let commit_cmd = Rc::new(CommitCommand::new()?);

    let cli = Cli::new("mini-git")
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_about("A minimal git subcommand simulation using eshu")
        .add_command(add_cmd)
        .add_command(commit_cmd)
        .parse();

    if !cli.is_subcommand_entered("add") && !cli.is_subcommand_entered("commit") {
        cli.print_help();
    }

    Ok(())
}
