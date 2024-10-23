fn draw_tree(triangle_count: usize) {
    let max_width = 2 * triangle_count + 1;
    for t in 1..=triangle_count {
        for level in 0..t {
            let stars = 2 * level + 1;
            let spaces = (max_width - stars) / 2;
            let row: String = " ".repeat(spaces)
                .chars()
                .chain("*".repeat(stars).chars())
                .chain(" ".repeat(spaces).chars())
                .collect();

            println!("{}", row);
        }
    }
}
fn main() {
    let triangle_count = 6;
    draw_tree(triangle_count);
}
