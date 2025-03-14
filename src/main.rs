use parser::parser::parse_line;
use std::io::{self, Write};

fn main() {
    println!("Wpisz linię do parsowania (lub 'exit' aby zakończyć):");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: &str = input.trim();

        if input == "exit" {
            break;
        }

        let parsed: parser::parser::ParsedAction = parse_line(input);

        println!("\n=== Wynik parsowania ===");
        println!("Zdarzenia: {:?}", parsed.events);
        println!("Komentarz: {:?}", parsed.comment);
        println!("Ostrzeżenia: {:?}", parsed.warnings);
        println!("========================\n");
    }
}
