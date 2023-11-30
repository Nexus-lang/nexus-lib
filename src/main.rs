use std::env;

use clutils::file_handler::FileHandler;
use nexus::NexusExtensions;

// nexus run <File>

// nexus bundle Vec<Files>

fn main() {
    let args: Vec<String> = env::args().collect();

    let first_arg = match args.get(1) {
        Some(arg) => arg,
        None => todo!(),
    };

    let second_arg = match args.get(2) {
        Some(arg) => arg,
        None => todo!(),
    };

    match first_arg.as_str() {
        "run" => {
            let src = FileHandler::new_with_extension(second_arg, Box::new(NexusExtensions::NX)).expect("Failed to handle file");
            nexus::run_interpreter(src)
        },
        _ => todo!(),
    };
}
