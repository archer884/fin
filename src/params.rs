use clap::{ App, Arg, ArgMatches };
use std::error::Error;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::path::Path;

pub fn read_args() -> Result<(Vec<String>, String), String> {
    let matches = load_matches();

    // The only things that can cause an early return here are
    // an absent dictionary and the lack of a target prefix. Here
    // we handle the prefix:
    let prefix = match matches.value_of("prefix") {
        Some(prefix) => prefix,
        None => return Err("Prefix not provided".to_string()),
    };

    // And here we handle the dictionary:
    let file = match matches.value_of("dictionary").map(|p| File::open(&Path::new(p))) {
        Some(Ok(file)) => file,
        Some(Err(e)) => return Err(format!("Unable to open file: {}", e.description())),
        None => return Err("Dictionary not provided".to_string()),
    };

    let sort = matches.is_present("sort");
    let ddup = matches.is_present("dedup");

    Ok((load_file(BufReader::new(file), sort, ddup), prefix.to_string()))
}

fn load_file<R: BufRead>(r: R, sort: bool, ddup: bool) -> Vec<String> {
    let mut vec: Vec<String> = r.lines()
        .filter_map(|l| l.map(|l| l.trim().to_string()).ok())
        .collect();

    if sort { vec.sort(); }
    if ddup { vec.dedup(); }

    vec
}

fn load_matches<'a>() -> ArgMatches<'a> {
    App::new("fin")
        .version("0.0.5")
        .author("J/A <archer884@gmail.com>")
        .about("Provides word completions")
        .arg(Arg::new("prefix")
             .help("Search prefix, e.g. 'app' for 'apple'")
             .required(true)
             .index(1))
        .arg(Arg::new("dictionary")
             .help("Dictionary input file(s)")
             .required(true)
             .index(2))
        .arg(Arg::new("sort")
             .short("s")
             .long("sort")
             .help("Ensure dictionary is sorted")
             .takes_value(false))
        .arg(Arg::new("dedup")
             .short("d")
             .long("dedup")
             .help("Ensure dictionary is deduplicated")
             .takes_value(false))
        .get_matches()
}
