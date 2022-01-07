use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_args = args.len();

    if let 0 = num_args {
        panic!("Error: insufficient arguments");
    }

    
}
