use std::env;
use std::str::FromStr;

use learn2code::gcd;

mod webapp;
use crate::webapp::webapp_main;

#[actix_web::main]
async fn main() {
    println!("Welcome to use");

    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        print_help();
    }

    let cmd = &args[1];
    match cmd.as_str() {
        "-h" | "--help" => print_help(),
        "-v" | "--version" => print_version(),
        "gcd" => gcd_cmd_handler(&args[2..]),
        "web" => webapp_cmd_handler().await,
        _ => print_help(),
    }
}

fn print_help() {
    println!("Usage: gcd num1 num2 ...numn");
}

fn print_version() {
    println!("learn2code 0.1.0");
}

fn gcd_cmd_handler(args: &[String]) {
    let mut nums = Vec::new();
    for arg in args {
        nums.push(u64::from_str(arg).expect("invalid number"));
    }

    if nums.len() == 0 {
        eprintln!("no number given");
        std::process::exit(1);
    }

    let mut d = nums[0];
    for m in &nums[1..] {
        d = gcd(d, *m);
    }

    println!(
        "gcd({}) = {}",
        format!(
            "{}",
            nums.iter()
                .map(|&n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        d
    )
}

async fn webapp_cmd_handler() {
    webapp_main().await;
}
