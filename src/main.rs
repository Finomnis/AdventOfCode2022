use anyhow::{anyhow, Result};
use clap::Parser;
use std::{fs, path::PathBuf};

pub mod helpers;

// DAILY: Add new solutions here
solutions! {
    (day01, task1, task2)
    (day02, task1, task2)
    (day03, task1, task2)
    (day04, task1, task2)
}

// DAILY: Add new reference solutions here
reworked_solutions! {}

// DAILY: Add new renderers here
renderers! {}

// Command line arguments
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    /// The day of the challenge, can be 1-25
    #[arg()]
    pub day: u8,

    /// The task on the day, can be 1 or 2
    #[arg()]
    pub task: u8,

    /// The path to the challenge input data
    #[arg()]
    pub data: PathBuf,

    /// Run the reworked solution of mine
    #[arg(short, long)]
    pub reworked: bool,

    /// Render the task visually, if available
    #[arg(long)]
    pub render: bool,
}

fn main() -> Result<()> {
    let opts = Options::parse();

    let input_file_path = opts.data;
    let data = fs::read_to_string(&input_file_path).map_err(|err| {
        anyhow!(
            "Unable to open '{}': {}",
            &input_file_path.into_os_string().into_string().unwrap(),
            err
        )
    })?;

    if opts.render {
        let artifacts = run_renderer(opts.day, opts.task, &data)?;
        println!("─ Rendering artifacts: ─────────────────────────");
        for artifact in artifacts {
            println!("{}", artifact);
        }
        println!("────────────────────────────────────────────────");
        return Ok(());
    }

    let result = match opts.reworked {
        true => run_reworked_solutions(opts.day, opts.task, &data)?,
        false => run_solution(opts.day, opts.task, &data)?,
    };

    println!("─ Result: ──────────────────────────────────────");
    println!("{}", result);
    println!("────────────────────────────────────────────────");

    Ok(())
}
