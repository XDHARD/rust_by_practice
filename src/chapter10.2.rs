//(1)
struct Array<T, const N: usize> {
    data: [T; N],
}

enum AnyArray {
    Int(Array<i32, 3>),
    Float(Array<f64, 3>),
    Int2(Array<i32, 2>),
}

fn main() {
    let arrays = [
        AnyArray::Int(Array { data: [1, 2, 3] }),
        AnyArray::Float(Array { data: [1.0, 2.0, 3.0] }),
        AnyArray::Int2(Array { data: [1, 2] }),
    ];

    println!("Success!");
}
//(2)
fn print_array<T: std::fmt::Debug>(arr: T) {
    println!("{:?}", arr);
}

fn main() {
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}
//(3)
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{

}

fn main() {
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 1]);
    check_size([(); 1].map(|_| "hello你好".to_string()));
    check_size(['中'; 1]);

    println!("Success!");
}

pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
