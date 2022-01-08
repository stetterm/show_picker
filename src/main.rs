use std::env;
use rand::seq::SliceRandom;

use picker::list_io;

const FILENAME: &str = "anime_list.txt";

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_args = args.len();

    if num_args == 1 {
        panic!("Error: insufficient arguments");
    }

    let mut titles = list_io::get_list(FILENAME).unwrap();
    let choice: &str;

    if let "pick" = &(args[1].to_lowercase())[..] {
        choice = match titles.choose(&mut rand::thread_rng()) {
            Some(c) => c,
            None => { panic!("List cannot be empty"); }
        };
        println!("Watch next: {:?}", choice);
    }
}
