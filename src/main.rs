use rand::Rng;




fn get_computer_choice() -> i32 {
    let num = rand::thread_rng().gen_range(0..3);
    num



}









fn main() {
    let number = get_computer_choice();
    println!("{}", number);
}
