use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("You guessed: {}", secret_num);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Valid Number Please!");
                continue;
            }
        };
        // .expect("Please type a number!");
        // if secret_num == guess {
        //     println!("You win!");   
        // }
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
