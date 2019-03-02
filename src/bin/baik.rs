extern crate pest;
extern crate baik;
extern crate pest_derive;
extern crate clap;

use baik::core::interpreter;
use clap::{Arg, App};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let matches = App::new("Bahasa Perograman BAIK")
                    .version("v10.0")
                    .author("Eka Tresna Irawan <anak10thn@gmail.com>")
                    .about("Bahasa Anak Indonesia untuk Komputer")
                    .arg(Arg::with_name("INPUT")
                        .help("Masukan berkas BAIK (.ina)")
                        .required(true)
                        .index(1))
                    .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    let f = File::open(filename).expect("Berkas tidak ditemukan!");
    for line in BufReader::new(f).lines() {
        interpreter(line.unwrap());
    }

    match matches.occurrences_of("v") {
        0 => println!("exit"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
}
