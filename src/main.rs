use parser::parser::{parse_line, ParsedAction}; 
use parser::translation::{init_translations, tr};

use std::env;
use std::io::{self, Write, stdin};

fn main() {
    // default - Polish language
    let _lang: String = env::var("LANG").unwrap_or_else(|_| "en".to_string());

    init_translations();
    
    println!("{}:", tr("Enter a line to parse (or type 'exit' to quit)"));

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        let input: &str = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("{}", tr("Exiting..."));
            break;
        }

        let parsed_line: ParsedAction = parse_line(input);

        update_with_parsed_line(parsed_line);
    }
}

fn update_with_parsed_line(parsed: ParsedAction) {
    println!("\n=====  {}  =====", tr("Parsing Result"));
        println!("{} {:?}", tr("Events"), parsed.events);
        println!("{} {:?}", tr("Comment"), parsed.comment);
        println!("{} {:?}", tr("Warnings"), parsed.warnings);
        println!("============================\n");
}
