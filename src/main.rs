use clap::{ArgAction, Parser, Subcommand};
use cryptography::prime_factors;
use std::ops::Range;
use fmtastic::{Subscript, Superscript};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "number_theory")]
#[command(author="Ajeesh T. Vijayan", version="0.0.1", about="Number Theory Calculator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Operations,
}

#[derive(Debug, Subcommand)]
enum Operations {
    #[command(arg_required_else_help = true)]
    PrimRoots {
        #[arg(short = 's', long = "start", value_name = "START NUMBER")]
        start: u64,

        #[arg(short = 'e', long = "end", value_name = "END NUMBER")]
        end: u64,
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Operations::PrimRoots { start, end } => {
            let r = Range {
                start,
                end,
            };

            let mut nums_with_prim_roots: Vec<u64> = Vec::new();
            let mut nums_without_no_prim_roots: Vec<u64> = Vec::new();

            for i in r {
                let prim_roots_i = cryptography::primitive_roots_trial_n_error(i);
                if prim_roots_i.len() > 0 {
                    nums_with_prim_roots.push(i);
                } else {
                    nums_without_no_prim_roots.push(i);
                }
            }

            println!("Numbers with at least one primitive roots: {:?}", nums_with_prim_roots);
            for i in nums_with_prim_roots {
                let p_factors = prime_factors(i);
                let mut form: String = String::new();
                for (factor, exp) in p_factors {
                    form.push_str(&format!("{}{} x ", factor, Superscript(exp)));
                }
                let mut form = form.trim_end().to_string();
                form.pop();

                println!("{} = {}", i, form);
            }

            println!("");
            println!("Numbers with no primitive roots: {:?}", nums_without_no_prim_roots);
            for i in nums_without_no_prim_roots {
                let p_factors = prime_factors(i);
                let mut form: String = String::new();
                for (factor, exp) in p_factors {
                    form.push_str(&format!("{}{} x ", factor, Superscript(exp)));
                }
                let mut form = form.trim_end().to_string();
                form.pop();

                println!("{} = {}", i, form);
            }
        }
    }
}

