#![feature(box_syntax, slice_patterns)]

extern crate clap;
mod prefix;
mod params;

pub fn main() {
    let (words, target) = match params::read_args() {
        Ok((words, target)) => (words, target),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    match prefix::get_descendants(&target, &words[..]) {
        Some(descendants) => for d in descendants { println!("{}", d); },
        None => println!("No matches found."),
    }
}
