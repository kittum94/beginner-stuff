use std::io;

fn main() {

    println!("Please enter a number.");
    
    let mut user_input = String::new();
    let mut i: u32 = 1;

    io::stdin()
         .read_line(&mut user_input)
         .expect("Failed to read user input");
    
    let input_trim: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("you are scum"),
        };

    while i != input_trim {
        
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
        
        i = i + 1;



    }

}
