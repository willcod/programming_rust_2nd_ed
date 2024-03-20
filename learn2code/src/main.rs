use std::env;
use std::str::FromStr;

use learn2code::utils::gcd;

mod webapp;
use crate::webapp::webapp_main;

use learn2code::mandelbrot::parse_complex;
use learn2code::mandelbrot::parse_pair;
use learn2code::mandelbrot::render;
use learn2code::mandelbrot::write_image;

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
        "mandelbrot" => mandelbrot_cmd_handler(&args[2..]),
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

fn mandelbrot_cmd_handler(args: &[String]) {
    if args.len() != 4 {
        eprintln!("Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT");
        eprintln!("Example: mandelbrot mandel.png 1000x750 -1.20,0.35 -1,0.20");
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[1], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[2]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[3]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[0], &pixels, bounds).expect("error writing image");
}
