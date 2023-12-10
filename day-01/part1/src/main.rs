use std::{env, fs};

fn get_first_last_digit(line: &str) -> (u32, u32) {
    let (mut first, mut last) = (0, 0);
    for c in line.chars() {
        if c.is_numeric() {
            let num = c.to_string().parse().unwrap();
            if first == 0 {
                first = num;
            }
            last = num;
        }
    }

    return (first, last);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Input data file is missing.");
        println!("usage: {} input-file.txt", args[0]);
        std::process::exit(1);
    }

    let input_file_name = &args[1];
    let contents = fs::read_to_string(&input_file_name)
        .expect(&format!("unable to open file {:?}", &input_file_name));

    let mut calibration_value: u32 = 0;
    for line in contents.split('\n') {
        let (first, last) = get_first_last_digit(line);
        calibration_value += first * 10 + last;
    }

    println!("Calibration value: {calibration_value}");
}
