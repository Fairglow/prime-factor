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
        .arg(Arg::new("number")
            .num_args(1..)
            .required(true)
            .value_name("NUMBER")
            .help("One or more numbers or ranges (e.g., 42, 100..200, 100..=200)"))
        .get_matches();
    env_logger::init();
    info!("Welcome to Prime factorizer");

    let numstr_vec: ValuesRef<String> = args.get_many("number").unwrap();
    let mut range_vec: Vec<RangeInclusive<u128>> = Vec::new();
    for numstr in numstr_vec {
        if let Some((left, right)) = numstr.split_once("..=")
            .or_else(|| numstr.split_once(".."))
        {
            debug!("Split '{numstr}' into '{left}' and '{right}'");
            let beg = left.parse::<u128>().expect("invalid range start");
            let end = right.parse::<u128>().expect("invalid range end");
            assert!(beg <= end, "invalid range: start ({beg}) must be <= end ({end})");
            range_vec.push(beg..=end);
        } else {
            let n = numstr.parse::<u128>().expect("invalid number");
            range_vec.push(n..=n);
        }
    }
    for rng in range_vec {
        let results: Vec<_> = rng.into_par_iter().map(|n| match n {
            0 | 1 => format!("{n} is neither prime nor composite"),
            _ => {
                let factors = PrimeFactors::factorize(n);
                if factors.is_prime() {
                    format!("{n} is prime!")
                } else {
                    format!("{n} = {factors}")
                }
            }
        }).collect();
        for line in results { println!("{line}"); }
    }
}

