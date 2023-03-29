#![allow(dead_code)]
#![allow(unused_must_use)]

use clap::{Parser, Subcommand};
use file_sorter_x::{create_files, custom_sort, sort_files, update_filesorterx};
use std::time::SystemTime;

mod tests;

/*
Made by Xanthus
Check out my other works at https://github.com/Xanthus58
Email me at 'Xanthus58@protonmail.com'
You can see more information on my website https://xanthus58.github.io/Xanthus58/
*/

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sorts the files in the current directory
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Sort {
        /// Verbose mode
        #[arg(short, long, default_value_t = false)]
        verbose: bool,

        /// Generates a log file
        #[arg(short, long, default_value_t = false)]
        log: bool,
    },
    Create {
        /// The amount of files to create
        #[arg(short, long)]
        amount: u32,
    },
    Customsort {
        /// The input directory
        #[arg(short, long)]
        input: String,

        /// The output directory
        #[arg(short, long)]
        output: String,

        /// The file extension to sort
        #[arg(short, long)]
        extension: String,
    },
    Update {},
}

fn main() {
    let start = SystemTime::now();
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Sort { verbose, log }) => {
            sort_files(verbose, log);
            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();
            println!("Time taken: {:?}", duration);
        }
        Some(Commands::Customsort {
            input,
            output,
            extension,
        }) => {
            custom_sort(input, output, extension);
        }
        Some(Commands::Create { amount }) => {
            create_files(amount + 1);
            let end = SystemTime::now();
            let duration = end.duration_since(start).unwrap();
            println!("Time taken: {:?}", duration);
        }
        Some(Commands::Update { .. }) => {
            update_filesorterx().expect("Failed to update FileSorterX");
        }
        None => todo!(),
    }
}
