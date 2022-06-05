use std::io;

fn main() {
    
    
    let mut input = String::new();

    println!("ENGLISH PIG LATIN TRANSLATOR");
    println!("enter a word. just one. all lowercase. no funny symbols. example: apple");

    // take input for translation
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    // convert input to piglatin
    println!("{}", to_piglatin(&mut input));
}

fn to_piglatin(input: &mut String) -> String {
    // remove whitespace from input and get first char from input
    let mut trimmed = input.trim().to_string();
    let vector = &trimmed.clone()[0..1];

    // determine if first char of input is a vowel or consonant
    match vector {
        "b" | "c" | "d" | "f" | "g" |  "j" | "k" | "l" | "m" | "n" | "p" | "q" | "r" | "s" | "t" | "v" | "x" | "z" | "h" | "w" | "y" => {
            // append first char and -hay to end if first char is a consonant
            trimmed.remove(0); 
            trimmed.push_str("-");
            trimmed.push_str(vector);
            trimmed.push_str("ay");
        
        }
        // append just -hay to end if first char is a vowel
        _ => {
            trimmed.push_str("-hay");
        }
    }
    
 
    return trimmed;
}
