//(1)
fn main() {

    let arr: [i32; 5] = [1, 2, 3, 4, 5];


    assert!(arr.len() == 5);

    println!("Success!");
}
//(2)
fn main() {

    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];


    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success!");
}
//(3)
fn main() {

    let mut list: [i32; 100] = [0; 100];
    list[0] = 1;

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}
//(4)
fn main() {

    let _arr = ['1', '2', '3'];

    println!("Success!");
}
//(5)
fn main() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0];

    assert!(ele == 'a');

    println!("Success!");
}
//(6)
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];


    let name0 = names.get(0).unwrap();


    let _name1 = &names[1];

    println!("Success!");
}
