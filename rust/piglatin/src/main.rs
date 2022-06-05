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
        "a" | "e" | "i" | "o" | "u" => {
            // append just -hay to end if first char is a vowel
            trimmed.push_str("-hay");
        
        }
        // append first char and ay to end if first char is a consonant
        _ => {
            trimmed.remove(0); 
            trimmed.push_str("-");
            trimmed.push_str(vector);
            trimmed.push_str("ay");
        }
    }
    
 
    return trimmed;
}
