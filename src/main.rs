use std::env;
use std::process;
use ibmparse::Config;

// splitting your binary project
// split your program into a main.rs and lib.rs
// move program logic into lib.rs
// cli parsing logic can remain in main.rs if it's small


// Group config fields

fn main() {
    // main responsibilities:
    // - calling the CLI parsing logic
    // - setting up any configuration
    // - calling a run in lib.rs
    // - handling the error if run returns an error

    // need to annotate types when using Collect
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err) ;
        process::exit(1);
    });



    if let Err(e) = ibmparse::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    } 
}

