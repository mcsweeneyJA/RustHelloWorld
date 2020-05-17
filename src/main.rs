use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number");
    //local to current thread and seeded by OS.
    let hidden_num = rand::thread_rng().gen_range(1, 101);


    loop{
        println!("Input your guess");

        //create a string to store user input in
        //::new implies that new is an associated function of string
        let mut guess = String::new();

        io::stdin()
            //readline takes a string that we wish to store
            //the users input inside
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {} ", guess);

        match guess.cmp(&hidden_num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won!!!");
                break;
            }
        }
    }


}
