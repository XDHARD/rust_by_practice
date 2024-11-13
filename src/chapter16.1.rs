//(1)
fn main() {
    let s1 = "hello";
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}
//(2)
fn main() {
    print!("hello world, ");
    println!("I am");
    println!("Sunface!");
}
