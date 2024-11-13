//(1)
fn main() {
    let x = 5;

    let p = &x;

    println!("the memory address of x is {:p}", p);
}
//(2)
fn main() {
    let x = 5;
    let y = &x;


    assert_eq!(5, *y);

    println!("Success!");
}
//(3)
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}
//(4)
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success! {}", s);
}

fn push_str(s: &mut String) {
    s.push_str("world");
}
//(5)
fn main() {
    let mut s = String::from("hello, ");


    let p = &mut s;

    p.push_str("world");

    println!("Success! {}", s);
}
//(6)
fn main() {
    let c = '中';

    let r1 = &c;

    let r2 = &c;

    assert_eq!(*r1, *r2);


    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}


fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
//(7)
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;



    println!("{}", r1);

    println!("Success!");
}
//(8)
fn main() {

    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}

fn borrow_object(s: &mut String) {}
//(9)
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}
//(10)
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");


    println!("{}", r1);
}
//(11)
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;


    r1.push_str("world");
}