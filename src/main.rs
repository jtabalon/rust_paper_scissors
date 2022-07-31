use rand::Rng;




fn get_computer_choice() -> &'static str {
    let num: i32 = rand::thread_rng().gen_range(0..3);

    // let hand: &str = match num {
    //     0 => String::from("Rock"),
    //     1 => String::from("Paper"),
    //     2 => String::from("Scissors"),
    // };

    let hand: &str = match num {
        0 => "Rock",
        1 => "Paper",
        2 => "Scissors",
        _ => "Rock"
    };
    hand
}

fn main() {
    let hand = get_computer_choice();
    println!("{}", hand);
}
