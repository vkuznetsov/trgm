
mod trigrams;

use crate::trigrams::Trigrams;

use std::io::{self, BufReader, BufRead};
use std::fs::File;
use clap::{Arg, App};


fn main() {
    let matches = App::new("Trigram corrector")
        .version("0.1.0")
        .arg(Arg::with_name("dict_file")
             .short("d")
             .long("dict")
             .takes_value(true)
             .required(true)
             .help("Name of the dictionary file"))
        .get_matches();


    let dict_filename = matches.value_of("dict_file").unwrap();

    let mut trigrams = Trigrams::new();

    let file = File::open(dict_filename).expect("Dictionary file not found!");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        trigrams.add_doc(line.unwrap());
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let result = trigrams.find_doc(line.unwrap()).unwrap_or("NULL");
        println!("{}", result);
    }
}
