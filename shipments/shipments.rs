use rand::Rng;
fn generate_equal_distribution(num_ships: usize, average: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..num_ships).map(|_| rng.gen_range(average - 0..=average + 0)).collect()
}
fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    if total % n != 0 {
        return None;
    }
    let average = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &load in shipments {
        balance += load as i32 - average as i32;
        moves += balance.abs() as usize;
    }
    Some(moves)
}
fn main() {
    let shipments = vec![9, 3, 7, 2, 9];
    if let Some(moves) = count_permutation(&shipments) {
        println!("Мінімальна кількість переміщень: {}", moves);
    } else {
        println!("Неможливо розподілити вантаж рівномірно.");
    }
    let random_shipments = generate_equal_distribution(5, 4);
    println!("Рівномірні вантажі: {:?}", random_shipments);
}
