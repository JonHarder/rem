mod context;
mod reminder;

use clap::Parser;
use std::io;

#[derive(Debug, Parser)]
#[command(name = "rem", version = "0.0.1", author = "Jon Harder")]
struct RemApp {
    /// The thing to remember.
    item: String,

    /// Display all reminders across all projects.
    #[arg(short, long)]
    #[clap(group = "selection")]
    all: bool,

    /// Display all reminders for this project.
    #[arg(short, long)]
    #[clap(group = "selection")]
    project: bool,
}

fn main() -> io::Result<()> {
    let args = RemApp::parse();
    let reminder = reminder::Reminder::try_from(args.item.as_str())?;
    println!("{:?}", reminder);
    Ok(())
}
