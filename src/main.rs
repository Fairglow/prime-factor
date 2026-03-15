use std::ops::RangeInclusive;
use clap::{Command, Arg, parser::ValuesRef};
use log::{debug, info};
use rayon::prelude::*;
use primefactor::{PrimeFactors, PrimeNumbers, DescendingPrimes};

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
        .arg(Arg::new("next")
            .short('n')
            .long("next")
            .action(clap::ArgAction::SetTrue)
            .help("Find the next prime number strictly greater than the given number(s)"))
        .arg(Arg::new("prev")
            .short('p')
            .long("prev")
            .action(clap::ArgAction::SetTrue)
            .help("Find the previous prime number strictly less than the given number(s)"))
        .get_matches();
    env_logger::init();
    info!("Welcome to Prime factorizer");

    let is_next = args.get_flag("next");
    let is_prev = args.get_flag("prev");

    let numstr_vec: ValuesRef<String> = args.get_many("number").unwrap();
    let mut range_vec: Vec<RangeInclusive<u128>> = Vec::new();
    for numstr in numstr_vec {
        if let Some((left, right)) = numstr.split_once("..=") {
            debug!("Split '{numstr}' into '{left}' and '{right}' (inclusive)");
            let beg = left.parse::<u128>().expect("invalid range start");
            let end = right.parse::<u128>().expect("invalid range end");
            assert!(beg <= end, "invalid range: start ({beg}) must be <= end ({end})");
            range_vec.push(beg..=end);
        } else if let Some((left, right)) = numstr.split_once("..") {
            debug!("Split '{numstr}' into '{left}' and '{right}' (exclusive end)");
            let beg = left.parse::<u128>().expect("invalid range start");
            let end = right.parse::<u128>().expect("invalid range end");
            assert!(beg < end, "empty or invalid range: {beg}..{end}");
            range_vec.push(beg..=(end - 1));
        } else {
            let n = numstr.parse::<u128>().expect("invalid number");
            range_vec.push(n..=n);
        }
    }
    for rng in range_vec {
        let results: Vec<_> = rng.into_par_iter().map(|n| {
            if is_next {
                let next_p = PrimeNumbers::from(n.saturating_add(1)).next().unwrap();
                return format!("Next prime after {n} is {next_p}");
            }
            if is_prev {
                if let Some(p) = DescendingPrimes::from(n.saturating_sub(1)).next() {
                    return format!("Previous prime before {n} is {p}");
                } else {
                    return format!("No prime strictly less than {n}");
                }
            }
            match n {
                0 | 1 => format!("{n} is neither prime nor composite"),
                _ => {
                    let factors = PrimeFactors::factorize(n);
                    if factors.is_prime() {
                        format!("{n} is prime!")
                    } else {
                        format!("{n} = {factors}")
                    }
                }
            }
        }).collect();
        for line in results { println!("{line}"); }
    }
}

