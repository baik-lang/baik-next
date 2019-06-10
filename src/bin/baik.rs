extern crate pest;
extern crate baik;
extern crate pest_derive;
extern crate clap;

use baik::core::interpreter;
use clap::{Arg, App};
use std::fs;

fn main() -> std::io::Result<()>{

    let matches = App::new("Bahasa Perograman BAIK")
                    .version("v10.0")
                    .author("Eka Tresna Irawan <anak10thn@gmail.com>")
                    .about("Bahasa Anak Indonesia untuk Komputer")
                    .arg(Arg::with_name("INPUT")
                        .help("Masukan berkas BAIK (.ina)")
                        .required(true)
                        .index(1))
                    .get_matches();

    let path = matches.value_of("INPUT").unwrap();
    let f = fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("Unable to open file '{}': {}", path, e));
    interpreter(&f);
    

    // match matches.occurrences_of("v") {
    //     0 => println!("exit"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }
    Ok(())
}
