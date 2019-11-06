
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
   println!("Guess the number");
 
   let secret_num = rand::thread_rng().gen_range(1, 101);
   println!("the secret number is {}", secret_num);


   loop {
      println!("Please input your number: ");
   
      let mut guess_num = String::new();
      // read_line is an append operation
      io::stdin().read_line(&mut guess_num).ok().expect("Failed to read line.");

//       let guess: u32 = guess_num.trim().parse().ok().expect("Please pass in a number.");   
       let guess: u32 = match guess_num.trim().parse() {
             Ok(num) => num,
             Err(_) => continue
       };

         println!("You guessed: {}", guess);

         match guess.cmp(&secret_num) {
            Ordering::Less    =>   println!("Too small"),
            Ordering::Greater =>   println!("Too big"),
            Ordering::Equal   =>   {
                println!("You Win!");
	          break;
            }
         };

   }

}
