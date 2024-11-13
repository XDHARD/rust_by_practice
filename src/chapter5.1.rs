//(1)
fn main() {
    let x = String::from("Hello world");
    let y = x.clone();

    println!("{}, {}", x, y);
}
//(2)
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}


fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
//(3)
fn main() {
    let s = give_ownership();
    println!("{}", s);
}


fn give_ownership() -> String {
    let s = String::from("Hello world");


    let _s = s.as_bytes().to_vec();

    s
}
//(4)
fn main() {
    let s = String::from("Hello World");

    print_str(&s);

    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s);
}
//(5)
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
//(6)
fn main() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("{}", s1);
}
//(7)
fn main() {
    let mut x = Box::new(5);

    let y = &mut x;

    **y = 4;

    assert_eq!(*x, 4);

    println!("Success!");
}
//(8)
fn main() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;


    println!("{:?}", (&t.1));
}
//(9)
fn main() {
    let t = (String::from("hello"), String::from("world"));


    let (s1, s2) = (t.0.clone(), t.1.clone());

    println!("{:?}, {:?}, {:?}", s1, s2, t);
}
