use std::env;
use std::str::FromStr;

use corner_calculator::DirectionConvertible;

fn main() {
    //    let args: Vec<String> = env::args().collect();
    let exe: String = env::args().next().unwrap();
    let mut numbers = Vec::new();

    // executable is the first argument
    if env::args().len() != 6 {
        usage(exe);
        std::process::exit(1);
    }

    for arg in env::args().skip(1) {
        numbers.push(u16::from_str(&arg).expect("5 Numbers expected"));
    }

    if numbers[0] < 1 || numbers[0] > 9 {
        usage(exe);
        std::process::exit(1);
    }

    let newx = numbers[0].get_new_x_coord(numbers[1], numbers[3]);
    let newy = numbers[0].get_new_y_coord(numbers[2], numbers[4]);

    println!("{} px {} px", newx, newy);
}

fn usage(executable: String) {
    eprintln!("Usage: {0:?} <keypad direction|1-9> <screen width|0-{1}> <screen height|0-{1}> <window width|0-{1}> <window height|0-{1}>", executable, u16::MAX);
}
