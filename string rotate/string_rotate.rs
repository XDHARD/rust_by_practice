fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    let n = n.rem_euclid(len as isize) as usize;
    let (left, right) = s.split_at(len - n);
    format!("{}{}", right, left)
}
fn main() {
    let s = String::from("abcdefgh");
    let right = rotate(s.clone(), 2);
    println!("Right: {}", right);
    let left = rotate(s.clone(), -2);
    println!("Left: {}", left);
}