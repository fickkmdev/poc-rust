use rand::Rng;
fn main() {
    let random_number = rand::thread_rng().gen_range(1..=10);
    println!("generate Random 1-10 : {}", random_number);
}
