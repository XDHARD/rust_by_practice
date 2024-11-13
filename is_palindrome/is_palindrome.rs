fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s == s.chars().rev().collect::<String>()
}
fn main() {
    let number = 123;
    if is_palindrome(number) {
        println!("{} є паліндромом.", number);
    } else {
        println!("{} не є паліндромом.", number);
    }
}