use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let per = &s[8..];
    let hour: i32 = s[..2].parse().unwrap();
    let mut conv_hour = hour;
    if per == "AM" {
        if hour == 12 {
            conv_hour = 0;
        }
    } else {
        if hour != 12 {
            conv_hour += 12;
        }
    }


    format!("{:02}{}", conv_hour, &s[2..8])
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
