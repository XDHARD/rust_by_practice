//(1)
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"),
    };

    println!("Success!");
}
//(2)
struct Unit;
trait SomeTrait {

}

impl SomeTrait for Unit { }

fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
}


fn do_something_with_unit(u: Unit) { }

//(3)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let v = Point(0, 127, 255);
    check_color(Color(v.0, v.1, v.2));

    println!("Success!");
}

fn check_color(p: Color) {
    let Color(x, y, z) = p;
    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255);
}
//(4)
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let age = 18;
    let mut p = Person {
        name: String::from("sunface"),
        age,
    };


    p.age = 30;


    p.name = String::from("sunfei");

    println!("Success!");
}
//(5)
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("Success!");
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name,
    }
}
//(6)
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
}

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        username: u.username,
        active: u.active,
        sign_in_count: u.sign_in_count,
    }
}
//(7)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("{:#?}", rect1);
}
//(8)
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let _name = &f.name;


    println!("{}, {}, {:?}", f.name, f.data, f);
}