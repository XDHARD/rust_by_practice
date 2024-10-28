pub fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }
    for i in 2..=(*n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let max_test_number = 10000;// XD
    for number in 1..=max_test_number {
        if is_prime(&number) {
            println!("{} : не є простим числом.", number);
        } else {
            println!("{} : є простим числом.", number);
        }
    }
}