use std::{env, fs};

fn decode_string_to_number(line: &str) -> String {
    //Not the best approach in terms of time and space but good enough to do the job

    let transformations = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut res = String::from("");
    let mut source = String::from(line);
    while source.len() > 0 {
        for tr in transformations {
            if source.starts_with(tr.0) {
                res.push(tr.1);
                break;
            }
        }

        let c = source.remove(0);
        res.push(c);
    }

    return res;
}

fn get_first_last_digit(line: &str) -> (u32, u32) {
    let decoded = decode_string_to_number(&line);

    let (mut first, mut last) = (0, 0);
    for c in decoded.chars() {
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
