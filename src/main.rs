//A command-line tool to play hello_somebody

use clap::Parser;
use prj1::{hello, delete_zero, coin, mean, median, mode, variance, std, chi_square};

#[derive(Parser)]
#[clap(version = "1.0", author = "Wanqian", about = "Data")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Wanqian")]
    Hello {
        #[clap(short, long)]
        name: String,
    },
    Clean {
        #[clap(short, long)]
        v: String,
    },
    Coin {
        #[clap(short, long)]
        probability: f64,
    },
    Mean {
        #[clap(short, long)]
        v: String,
    },
    Median {
        #[clap(short, long)]
        v: String,
    },
    Mode {
        #[clap(short, long)]
        v: String,
    },
    Variance {
        #[clap(short, long)]
        v: String,
    },
    Std {
        #[clap(short, long)]
        v: String,
    },
    Chi {
        #[clap(short, long)]
        v: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        // call hello function
        Some(Commands::Hello { name }) => {
            let result = hello(&name);
            println!("{}", result);
        }
        None => println!("Something went wrong!"),
        // call delete_zero function
        Some(Commands::Clean { v }) => {
            // convert string to vector
            let v: Vec<i32> = v
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let result = delete_zero(&mut v.clone());
            println!("{:?}", result);
        }
        // call coin function
        Some(Commands::Coin { probability }) => {
            let result = coin(probability);
            println!("{}", result);
        }
        // call mean function
        Some(Commands::Mean { v }) => {
            // convert string to vector
            let v: Vec<i32> = v
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let result = mean(&v);
            println!("{}", result);
        }
        // call median function
        Some(Commands::Median { v }) => {
            // convert string to vector
            let v: Vec<i32> = v
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let result = median(&v);
            println!("{}", result);
        }
        // call mode function
        Some(Commands::Mode { v }) => {
            // convert string to vector
            let v: Vec<i32> = v
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let result = mode(&v);
            println!("{:?}", result);
        }
        // call variance function
        Some(Commands::Variance { v }) => {
            // convert string to vector
            let v: Vec<i32> = v
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let result = variance(&v);
            println!("{}", result);
        }
        // call std function
        Some(Commands::Std { v }) => {
            // convert string to vector
            let v: Vec<i32> = v
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let result = std(&v);
            println!("{}", result);
        }
        // call chi_square function
        Some(Commands::Chi { v }) => {
            // convert string to vector
            let v: Vec<i32> = v
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            let result = chi_square(&v);
            println!("{}", result);
        }
    }
}