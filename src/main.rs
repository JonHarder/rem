mod context;
mod reminder;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rem", version = "0.0.1", author = "Jon Harder")]
struct RemApp {
    item: String,

    /// Display all reminders, not just contextually relavant ones.
    #[arg(short, long)]
    all: bool,
}

fn main() -> std::io::Result<()> {
    let args = RemApp::parse();
    let reminder = reminder::Reminder::new(&args.item);
    println!("{}", serde_json::to_string(&reminder.context)?);
    // println!("{:?}", args);
    // println!("{:?}", reminder);
    Ok(())
}
