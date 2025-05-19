use clap::Parser;
use std::fs;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Add a new task
    Add { task: String },

    /// List all tasks
    List,

    // Mark a task as done
    Done { index: usize },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut tasks = load_tasks()?;

    match cli.command {
        Commands::Add { task } => {
            tasks.push(task.clone());
            save_tasks(&tasks)?;
            println!("Added task: {}", task);
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                println!("Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}. {}", i + 1, task);
                }
            }
        }
        Commands::Done { index } => {
            if let Some(task) = tasks.get_mut(index - 1) {
                *task = format!("[Done] {}", task);
                save_tasks(&tasks)?;
                println!("Marked task {} as done.", index);
            } else {
                println!("Invalid task index.");
            }
        }
    }
    Ok(())
}

fn load_tasks() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("tasks.txt").unwrap_or_default();
    let tasks = contents.lines().map(|s| s.to_string()).collect();
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create("tasks.txt")?;
    for task in tasks {
        writeln!(file, "{}", task)?;
    }
    Ok(())
}