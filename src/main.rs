use std::ops::RangeInclusive;
use clap::{Command, Arg, parser::ValuesRef};
use log::{debug, info};
use rayon::prelude::*;
use primefactor::PrimeFactors;

const APPNAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let args = Command::new(APPNAME)
        .version(VERSION)
        .author(AUTHORS)
        .about("Prime Factorizer")
        .arg(Arg::new("verbosity")
            .short('v')
            .long("verbose")
            .num_args(0..)
            .help("Increase the level of verbosity"))
        .arg(Arg::new("number")
            .num_args(1..)
            .required(true)
            .number_of_values(1)
            .value_name("NUMBER")
            .help("One or more numbers or ranges (inclusive)"))
        .get_matches();
    let verbosity = args.get_count("verbosity") as usize;
    stderrlog::new()
        .module(module_path!())
        .verbosity(verbosity)
        .init()
        .unwrap();
    info!("Welcome to Prime factorizer");

    let numstr_vec: ValuesRef<String> = args.get_many("number").unwrap();
    let mut range_vec: Vec<RangeInclusive<u128>> = Vec::new();
    for numstr in numstr_vec {
        let mut no_range = true;
        for range_sep in ["-", ".."].iter() {
            if let Some(pos) = numstr.find(range_sep) {
                let after = pos + range_sep.len();
                debug!("Split '{}' into '{}' and '{}'",
                    numstr, &numstr[..pos], &numstr[after..]);
                let beg = numstr[..pos].parse::<u128>().unwrap();
                let end = numstr[after..].parse::<u128>().unwrap();
                range_vec.push(beg..=end);
                no_range = false;
                break;
            }
        }
        if no_range {
            let n: u128 = numstr.parse::<u128>().unwrap();
            range_vec.push(n..=n);
        }
    }
    for rng in range_vec {
        let par_iter: Vec<_> = rng.into_par_iter().map(|n| {
            let factors = PrimeFactors::from(n);
            if factors.is_prime() {
                format!("{} is prime!", n)
            } else {
                format!("{} = {}", n, factors)
            }
        }).collect();
        for outstr in par_iter { println!("{}", outstr); }
    }
}

