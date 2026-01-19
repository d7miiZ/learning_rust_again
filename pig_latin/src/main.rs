use std::io;

fn main() {
    let consonant_list = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'];
    let vowel_list = ['a', 'e', 'i', 'o', 'u'];

    println!("Enter a sentence:");
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");

    let words = sentence.split_whitespace();
    for word in words {
        let first_letter = word.chars().next().unwrap();
        if consonant_list.contains(&first_letter.to_ascii_lowercase()) {
            let rest_of_word = word.chars().skip(1).collect::<String>();
            print!("{rest_of_word}-{first_letter}ay ");
        } else if vowel_list.contains(&first_letter.to_ascii_lowercase()) {
            print!("{word}-hay ");
        } else {
            print!("{word} ");
        }
    }

    println!();
}
