//(1)
fn main() {

    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
//(2)
fn main() {
    print();
}


fn print() -> () {
    println!("Success!");
}
//(3)
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    panic!("This function never returns!");
}
//(4)
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {

            Some(42)
        }
        _ => {

            never_return_fn();
        }
    }
}

fn never_return_fn() -> ! {

    loop {

    }
}
//(5)
fn main() {

    let b = false;

    let _v = match b {
        true => 1,

        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}