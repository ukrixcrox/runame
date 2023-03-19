// runame is a file renamer
// usage:
// runame [current filename] [new filename]
use clap::Parser;
use std::fs::rename;
use colored::Colorize;
use std::io::{self, Write};

/// Simple file renamer
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args{

    ///current filename
    current_filename: String,

    ///new filename
    new_filename: String,

    /// interactive
    #[arg(short='i', long)]
    interactive:bool,

    /// verbose output
    #[arg(short='v', long)]
    verbose:bool,
    
}

fn main() {
    let args = Args::parse();

    if args.interactive{
        print!("Rename {} to {}? [y/n]> ",args.current_filename.bold().blue(), args.new_filename.bold().green());
        io::stdout().flush().unwrap_or_else(|e| eprintln!("Error: {}", e));

        let mut input_buffer = String::new();
        io::stdin().read_line(&mut input_buffer).unwrap();

        match input_buffer.as_str().trim() {

            "y" => {
                rename(&args.current_filename, &args.new_filename).unwrap_or_else(|e| println!("Error: {}", e));
                println!("Renamed {} to {}", args.current_filename.bold().blue(), args.new_filename.bold().blue());
            },

            "n" => {
                println!("{}", "Stopping operation".bold().blue());
            },

            _ => {
                println!("{}", "No valid option has been given!".bold().red());
            },

        }
        std::process::exit(0x0100);
    }

    if args.verbose && !args.interactive{
        rename(&args.current_filename, &args.new_filename).unwrap_or_else(|e| println!("Error: {}", e));
        println!("Renamed {} to {}", args.current_filename.bold().blue(), args.new_filename.bold().green());
        std::process::exit(0x0100);
    }

    rename(&args.current_filename, &args.new_filename).unwrap_or_else(|e| println!("Error: {}", e));
}


