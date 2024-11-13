//(1)
use std::mem::size_of_val;

fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}
//(2)
fn main() {
    let c1 = "中";
    let c1_char = c1.chars().next().unwrap();
    print_char(c1_char);
}

fn print_char(c: char) {
    println!("{}", c);
}
//(3)
fn main() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}
//(4)
fn main() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!");
}
//(5)
fn main() {
    let _v: () = ();

    let v = ();
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}
//(6)
use std::mem::size_of_val;

fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}
