const SIZE: usize = 5;
fn main() {
    let mut rombus = String::new();
    for i in 0..SIZE {
        for _ in 0..(SIZE - i - 1) {
            rombus.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            rombus.push('*');
        }
        rombus.push('\n');
    }
    for i in (0..SIZE - 1).rev() {
        for _ in 0..(SIZE - i - 1) {
            rombus.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            rombus.push('*');
        }
        rombus.push('\n');
    }
    print!("{}", rombus);
}
