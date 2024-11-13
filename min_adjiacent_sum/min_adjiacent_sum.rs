use rand::Rng;
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_pair = (0, 0);
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (i, i + 1);
        }
    }
    (min_sum, min_pair.0, min_pair.1)
}
fn print_data_with_min_pair(data: Vec<i32>, min_sum: i32, min_index1: usize, min_index2: usize) {
    for i in 0..data.len() {
        print!("{} ", i);
    }
    println!();
    for num in &data {
        print!("{} ", num);
    }
    println!();
    for i in 0..data.len() {
        if i == min_index1 || i == min_index2 {
            print!("\\__ ");
        } else {
            print!("__ ");
        }
    }
    println!();
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[min_index1], data[min_index2], min_sum, min_index1, min_index2);
}
fn main() {
    let data = gen_random_vector(20);
    let (min_sum, min_index1, min_index2) = min_adjacent_sum(&data);
    print_data_with_min_pair(data, min_sum, min_index1, min_index2);
}
