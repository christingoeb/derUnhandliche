use std::io::{self, Write}; // Importiere für Eingabe und Ausgabe

fn main() {
    // Begrüßung und Erklärung
    println!("Willkommen beim Rust Taschenrechner!");
    println!("Geben Sie zwei Zahlen und einen Operator (+, -, *, /) ein.");

    // Erhalte die erste Zahl
    let num1 = read_number("Erste Zahl: ");

    // Erhalte den Operator
    let operator = read_operator("Operator (+, -, *, /): ");

    // Erhalte die zweite Zahl
    let num2 = read_number("Zweite Zahl: ");

    // Berechne das Ergebnis basierend auf dem Operator
    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                println!("Fehler: Division durch Null ist nicht erlaubt!");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Ungültiger Operator!");
            return;
        }
    };

    // Ergebnis ausgeben
    println!("Das Ergebnis ist: {}", result);
}

// Funktion zum Lesen einer Zahl
fn read_number(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Flush zum sofortigen Anzeigen der Eingabeaufforderung

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Versuch die Eingabe in eine Gleitkommazahl umzuwandeln
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Ungültige Eingabe, bitte eine gültige Zahl eingeben."),
        }
    }
}

// Funktion zum Lesen des Operators
fn read_operator(prompt: &str) -> char {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        // Überprüfen, ob der Operator gültig ist
        if trimmed == "+" || trimmed == "-" || trimmed == "*" || trimmed == "/" {
            return trimmed.chars().next().unwrap();
        } else {
            println!("Ungültiger Operator, bitte einen der folgenden eingeben: +, -, *, /");
        }
    }
}
