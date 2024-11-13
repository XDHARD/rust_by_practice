//(1)
fn main() {
    let v: &str = "hello";
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}
//(2)
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut config: Option<&mut Config> = None;

fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    Some(Box::leak(c))
}


fn main() {
    unsafe {
        config = init();

        println!("{:?}",config)
    }
}
//(3)
fn main() {
    let static_string = "I'm in read-only memory";
    println!("static_string: {}", static_string);

    println!("static_string reference remains alive: {}", static_string);
}
//(5)
use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

fn print_it1( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}


fn print_it2<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    const i:i32 = 5;
    print_it(i);

    print_it(&i);

    print_it1(&i);

    print_it2(&i);
}
use std::fmt::Display;
//(6)
fn main() {
    let mut string = "First".to_owned();

    string.push_str(string.to_uppercase().as_str());
    print_a(&string);
    print_b(&string);
    print_c(&string);
    print_d(&string);
    print_e(&string);
    print_f(&string);
    print_g(&string);
}

fn print_a<T: Display + 'static>(t: &T) {
    println!("{}", t);
}

fn print_b<T>(t: &T)
where
    T: Display + 'static,
{
    println!("{}", t);
}

fn print_c(t: &dyn Display) {
    println!("{}", t)
}

fn print_d(t: &impl Display) {
    println!("{}", t)
}

fn print_e(t: &(dyn Display + 'static)) {
    println!("{}", t)
}

fn print_f(t: &(impl Display + 'static)) {
    println!("{}", t)
}

fn print_g(t: &String) {
    println!("{}", t);
}