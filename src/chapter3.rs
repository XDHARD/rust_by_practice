// (1)
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
//(2)
fn main() {
    let  mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}
//(3)
fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}
//(4)
fn main() {
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}
//(5)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}
//(6)
fn main() {
    let mut _x: i32 = 1;
    _x = 7;
    // Shadowing and re-binding
    _x += 3;


    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";

    println!("Success!");
}
//(7)
fn main() {
    let x = 1;
    println!("{}", x);
}
//(8)
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
//(9)
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
}