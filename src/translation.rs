use std::collections::HashMap;
use std::sync::OnceLock;
use std::env;

static TRANSLATIONS: OnceLock<HashMap<&'static str, HashMap<&'static str, &'static str>>> = OnceLock::new();

pub fn init_translations() -> &'static HashMap<&'static str, HashMap<&'static str, &'static str>> {
    TRANSLATIONS.get_or_init(|| {
        let mut translations: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

        // English (en)
        let mut en: HashMap<&str, &str> = HashMap::new();
        en.insert("Enter a line to parse (or type 'exit' to quit)", "Enter a line to parse (or type 'exit' to quit)");
        en.insert("Invalid team value", "Invalid team value");
        en.insert("Invalid player's number", "Invalid player's number");
        en.insert("Invalid skill type", "Invalid skill type");
        en.insert("Invalid eval type", "Invalid eval type");
        en.insert("Invalid zone", "Invalid zone");
        en.insert("Invalid subzone", "Invalid subzone");
        en.insert("Not enough characters", "Not enough characters");
        en.insert("Exiting...", "Exiting...");
        en.insert("Parsing Result", "Parsing Result");
        en.insert("Events", "Events");
        en.insert("Comment", "Comment");
        en.insert("Warnings", "Warnings");

        // Polish (pl)
        let mut pl: HashMap<&str, &str> = HashMap::new();
        pl.insert("Enter a line to parse (or type 'exit' to quit)", "Wprowadź linię do sparsowania (lub wpisz 'exit', aby wyjść)");
        pl.insert("Invalid team value", "Nieprawidłowa wartość drużyny");
        pl.insert("Invalid player's number", "Nieprawidłowy numer zawodnika");
        pl.insert("Invalid skill type", "Niepoprawny rodzaj umiejętności");
        pl.insert("Invalid eval type", "Niepoprawna wartość eval");
        pl.insert("Invalid zone", "Niepoprawna strefa");
        pl.insert("Invalid subzone", "Niepoprawna podstrefa");
        pl.insert("Not enough characters", "Zbyt mało znaków");
        pl.insert("Exiting...", "Zamykanie...");
        pl.insert("Parsing Result", "Wynik Parsowania");
        pl.insert("Events", "Zdarzenia");
        pl.insert("Comment", "Komentarz");
        pl.insert("Warnings", "Ostrzeżenia");

        translations.insert("en", en);
        translations.insert("pl", pl);

        translations
    })
}

pub fn tr(text: &str) -> String {
    let lang: String = env::var("LANG").unwrap_or_else(|_| "en".to_string());
    let lang: &str = if lang.starts_with("pl") { "pl" } else { "en" }; 

    let translations: &HashMap<&str, HashMap<&str, &str>> = init_translations();
    translations
        .get(lang)
        .and_then(|map: &HashMap<&str, &str>| map.get(text))
        .map(|s: &&str| s.to_string())
        .unwrap_or_else(|| text.to_string()) 
}
