use std::io;
// getting input/output library into scope from Std (standard library)
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!(" The secret number is {secret_number}");
loop{
    println!("Please input your guess.");
    
//  let to create a variable example: let name = "nikhil" this var is imutable by default
// if I put mut after let then it will be mutable

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read the line");

    let guess: u32 = guess.trim().parse().expect("Please type a Number!");

    println!("you Guessed: {guess}");

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too chota"),
        Ordering::Greater => println!("Bohut Badda"),
        Ordering::Equal => println!("Jiit Gye Bhaii"),
    }
}
}



