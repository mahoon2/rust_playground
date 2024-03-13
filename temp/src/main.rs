use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    // let ret = rand::thread_rng().gen_range(1..=100);

    // loop {
    //     println!("\nPlease input your guess.");
    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
        
    //     // println!("You guessed: {guess}");

    //     match guess.cmp(&ret) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    let mut tup = (1, 1.1, true);
    tup.0 = 3;

    let (x, y, z) = tup;
    print!("{x}")
}