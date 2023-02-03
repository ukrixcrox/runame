// runame is a file renamer
// usage:
// runame [current filename] [new filename]
use clap::Parser;
use std::fs::rename;
use colored::Colorize;

/// Simple file renamer
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args{
    ///current filename
    current_filename: String,
    ///new filename
    new_filename: String,

    /// verbose output
    #[arg(short='v', long)]
    verbose:bool,
    
}

fn main() {
    let args = Args::parse();
    rename(&args.current_filename, &args.new_filename).unwrap_or_else(|e| println!("Error: {}", e));

    if args.verbose{
        println!("Renamed {} to {}", args.current_filename.bold().blue(), args.new_filename.bold().green());
    }
}


