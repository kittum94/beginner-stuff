use std::io;

fn main() {
    
    let mut input = String::new();

    println!("ENGLISH PIG LATIN TRANSLATOR");
    println!("enter a word. just one. all lowercase. no funny symbols. example: apple");

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");


    println!("{}", to_piglatin(&mut input));
}

fn to_piglatin(input: &mut String) -> String {
    let mut trimmed = input.trim().to_string();
    let vector = &trimmed.clone()[0..1];

    match vector {
        "b" | "c" | "d" | "f" | "g" |  "j" | "k" | "l" | "m" | "n" | "p" | "q" | "r" | "s" | "t" | "v" | "x" | "z" | "h" | "w" | "y" => {
            trimmed.remove(0); 
            trimmed.push_str("-");
            trimmed.push_str(vector);
            trimmed.push_str("ay");
        
        }
        _ => {
            trimmed.push_str("-hay");
        }
    }
    

    return trimmed;
}
