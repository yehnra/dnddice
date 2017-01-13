// TODO: confirm that arguments can be converted to i32
//       currently, running `dnddice foo bar' produces a panic
// TODO: add limits on quantity and sides of dice
// TODO: test importing --csv to a spreadsheet program
extern crate time;
extern crate rand;
use std::env;
use time::{now, strftime};
use rand::{thread_rng, Rng};

static LABEL: &'static str = "[dnddice] ";
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        output_help();
    } else if args.len() > 4 {
        // maximum amount of args exceeded
        println!("{}Too many arguments.", LABEL);
        output_help();
        return;
    } else {
        let (arg_list, csv_mode, help_mode, version_mode) = parse_args(&args);
        if !args.contains(&"--csv".to_string()) && args.len() > 3 {
            println!("{}Too many arguments without --csv.", LABEL);
            output_help();
            return;
        }

        if help_mode {
            output_help();
        } else if version_mode {
            output_version();
        } else {
            roll(arg_list[0], arg_list[1], csv_mode);
        }
    }
}

fn roll(dice_quantity: i32, dice_sides: i32, csv: bool) {
    let time_now = now();
    let current_time = strftime("%A, %B %e - %l:%M:%S %P", &time_now).unwrap();
    let mut rng = thread_rng();
    let mut rng_value: i32;
    let mut running_total = 0;
    let high = dice_sides + 1;
    let low = 1;

    if csv {
        println!("roll_number,roll_result,running_total");
    } else {
        println!();
        println!("[{}]", current_time);
        println!("{}Rolling {} d{}:", LABEL, dice_quantity, dice_sides);
        println!("-------------------------");
    }

    for i in 0..dice_quantity {
        rng_value = rng.gen_range(low, high);
        running_total += rng_value;
        if csv {
            println!("{},{},{}", i + 1, rng_value, running_total);
        } else {
            println!("{}\t| Rolled a {}!", i + 1, rng_value);
        }
    }

    if !csv {
        println!("-------------------------");
        println!("{}Total: {}", LABEL, running_total);
    }

    println!();
}

fn parse_args(args: &Vec<String>) -> (Vec<i32>, bool, bool, bool) {
    let mut arg_list = Vec::new();
    let mut csv_mode = false;
    let mut help_mode = false;
    let mut version_mode = false;

    for i in 1..(args.len()) {
        if args[i] == "--help" {
            help_mode = true;
            break;
        } else if args[i] == "--version" {
            version_mode = true;
            break;
        } else if args[i] == "--csv" {
            csv_mode = true;
        } else {
            arg_list.push(args[i].parse::<i32>().unwrap());
        }
    }

    (arg_list, csv_mode, help_mode, version_mode)
}

// version info
fn output_version() {
    println!("\n[dnddice {}]", VERSION);
    println!("Copyright (C) 2017 Baerlabs");
    println!("License GPLv3: GNU GPL version 3 <http://gnu.org/licenses/gpl.html>");
    println!("dnddice is free to use, study, change, and/or redistribute.");
    println!("There is no warranty, to the extent permitted by law.");
    println!("\nWritten by yehnra");
    println!();
}

// help info
fn output_help() {
    println!("\n{}", LABEL);
    println!("Usage: dnddice [NUMBER_OF_DICE] [NUMBER_OF_DICE_SIDES]");
    println!("Example: dnddice 2 20         # rolls 2d20's");
    println!("Roll some dice.");
    println!("  --csv\t\t\toutput in .csv format, use `>' to pipe output to a file");
    println!("  --help\t\tdisplay this help and exit");
    println!("  --version\t\toutput version information and exit");
    println!();
}
