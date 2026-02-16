mod output;

use std::io;
use std::process;

use clap::Parser;
use kount_cli::Args;

use output::{print_json, print_summary, print_table};

fn run() -> i32 {
    let args = Args::parse();
    let config = args.to_config();
    let result = kount_count::count(&config);

    let print_result = if args.json {
        print_json(&result, args.top)
    } else if args.summary {
        print_summary(&result, args.top)
    } else {
        print_table(&result, args.top)
    };

    if let Err(e) = print_result {
        if e.kind() == io::ErrorKind::BrokenPipe {
            return 0;
        }
        eprintln!("error: {e}");
        return 1;
    }

    0
}

fn main() {
    process::exit(run());
}
