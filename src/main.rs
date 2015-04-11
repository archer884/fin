#![feature(box_syntax, slice_patterns)]

use std::error::Error;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::path::Path;

pub fn main() {
    let (words, target) = match read_args() {
        Ok((words, target)) => (words, target),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match get_descendants(&target, &words[..]) {
        Some(descendants) => for d in descendants { println!("{}", d); },
        None => println!("No matches found."),
    }
}

fn get_descendants<'a>(target: &str, words: &'a [String]) -> Option<&'a [String]> {
    let idx = words.binary_search(&target.to_string()).unwrap_or_else(|e| e);
    match words[idx].starts_with(target) {
        true => Some(&words[idx..(idx + words[idx..].iter().take_while(|w| w.starts_with(target)).count())]),
        false => None,
    }
}

fn read_args() -> Result<(Vec<String>, String), String> {
    let args: Vec<_> = std::env::args().collect();
    match &args[1..] {
        [ref path, ref target] => {
            match File::open(&Path::new(path)).map(|f| BufReader::new(f)) {
                Ok(file) => Ok((load_words(file), target.clone())),
                Err(e) => Err(e.description().to_string())
            }
        },
        _ => Err(format!("USAGE:\t{} <dictionary> <prefix>", args[0])),
    }
}

fn load_words<R: BufRead>(r: R) -> Vec<String> {
    BufReader::new(r).lines()
        .filter_map(|l| l.map(|l| l.trim().to_string()).ok())
        .collect()
}
