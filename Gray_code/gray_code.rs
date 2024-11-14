fn gray(bit: u8) -> Vec<String> {
    if bit == 0 {
        return vec!["".to_string()];
    }
    let previous = gray(bit - 1);
    let mut result = Vec::new();
    for code in &previous {
        result.push(format!("0{}", code));
    }
    for code in previous.iter().rev() {
        result.push(format!("1{}", code));
    }
    result
}
fn main() {
    let bit = 4;
    let gray_codes = gray(bit);
    println!("Код Грея = {}:", bit);
    for code in gray_codes {
        println!("{}", code);
    }
}