fn invert_the_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}
fn main () {
    let text = "Привіт Світ";
    let swap = invert_the_case(text);
    println!("Swapped: {}", swap);
}