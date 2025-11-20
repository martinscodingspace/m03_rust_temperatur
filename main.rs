//! Das Programm m03_rust_temperatur.rs beinhaltet folgenden Zweck:
//!
//! Programmierung eines Tools, welches Temperaturen in verschiedene
//! Einheiten umwandelt. Das Programm soll es dem Nutzer ermöglichen,
//! einen Zahlenwert und die jeweilige Einheit (z. B. Celsius,
//! Fahrenheit oder Kelvin) anzugeben und anschließend die Zieleinheit
//! zu wählen.
//! Das Ergebnis der Umrechnung soll anschließend formatiert
//! ausgegeben werden. Die Berechnungsfunktionen sollen sauber
//! voneinander getrennt und in einem eigenen Modul untergebracht
//! werden, sodass sie unabhängig getestet werden können.
//! Dazu gehören
//! zu jeder Umrechnungsfunktion passende Unit-Tests, die typische und
//! fehlerhafte Eingaben prüfen.
//! Alle Funktionen sind mit
//! Dokumentationskommentaren zu versehen, damit später eine automatisch
//! generierte Referenz entsteht.
//! Optional:
//! • Erweiterung um zusätzliche Einheiten
//! • Automatische Erkennung der Einheit aus der Nutzereingabe
//! • Ausgabe mit Nachkommagenauigkeit nach Benutzervorgabe
//! • Interaktive CLI-Auswahl über Menüs oder Eingabeaufforderungen


/********************************************************************
***  IHK Rust Developer 2025/2026                                 ***
***  m03_rust_temperatur                                          ***
***  Fälligkeit 21.11.2025 23:59                                  ***
***  written by Martin Hildebrand                                 ***
***  2025 ©  ALL RIGHTS RESERVED                                  ***
*********************************************************************
***  Das Repository befindet sich hier:                           ***
***  https://github.com/martinscodingspace/m03_rust_temperatur    ***
*********************************************************************
***  Diese Software steht unter folgender LIZENZ                  ***
***  GNU General Public License 3                                 ***
***  http://www.gnu.org/licenses/gpl-3.0.de.html                  ***
********************************************************************/

mod temperatur;

use std::io::{self, Write};
use colored::Colorize;
use crate::temperatur::*;
use crate::temperatur::degree;
use crate::temperatur::scale_in;
use crate::temperatur::scale_out;

fn main() {

    // Aufgrund der Größe und Komplexität ist bei diesem Programm eine
    // Löschung der vorigen Bildschirmeingaben sinnvoll
    // In Anlehnung des in Pascal integrieren "clrscn" (Clear-Screen)
    // eerfüllt die folgende Routine diesen Bedarf
    loop {
        clear_screen();

        println!("============================================================");
        println!("===             Programm m03_rust_temperatur             ===");
        println!("===     Umrechnung in eine andere Temperatureinheit      ===");
        println!("============================================================");
        println!("===     Celsius      < C >                               ===");
        println!("===     Fahrenheit   < F >                               ===");
        println!("===     Kelvin       < K >                               ===");
        println!("===     Réaumur      < RE >                              ===");
        println!("===     Rankine      < RA >                              ===");
        println!("============================================================");

        let val = degree();
        let src = scale_in();
        let trg = scale_out();

        // Das Herzstück des Programm mit Ergebnis-Ausgabe verbleibt in main-Datei
        match convert(val, &src, &trg) {
            Ok(res) => {

                println!("===     Ergebnis:      {:.2} {}  =  {:.2} {}", val, src.blue().bold(), res, trg.blue().bold());
                // println!("===     Folgende Werte    :  (C / F / K / RE / RA)       ===");
                println!("============================================================");
            }
            Err(err) => {
                println!("Fehler: {}", err);
            }

        }

        let footer = "============================================================";

        println!("{}", footer.purple());
        print!("===    Erneute Eingabe?   PRESS  < J >  :  ");
        io::stdout().flush().unwrap();
        let mut another = String::new();
        io::stdin().read_line(&mut another).expect("Fehler!");
        println!("{}", footer.purple());
        if (another.trim() != "j") && (another.trim() != "J") {
            break;
        }
    } // Ende Loop
    // Programm-ENDE

    let footer1 = "============================================================";
    let footer2 = "===                 Programm - ENDE                      ===";
    let footer3 = "===  Vielen Dank für die Nutzung von m03_rust_temperatur ===";
    let footer4 = "============================================================\n\n";
    println!("{}", footer1.blue());
    println!("{}", footer2.blue());
    println!("{}", footer3.blue());
    println!("{}", footer4.blue());
}



/// Dispatcher-Funktion, welche die richtige Umwandlung aufruft.
fn convert(gradzahl: f64, src: &str, trg: &str) -> Result<f64, String> {
    // keine neue Temperatureinheit
    if src == trg {
        return Ok(gradzahl);
    }

    // alle möglichen Varianten der fünf berühmtesten Temperatureinheiten
    match (src, trg) {
        ("C", "F") => cel2fah(gradzahl),
        ("C", "K") => cel2kel(gradzahl),
        ("C", "RE") => cel2rea(gradzahl),
        ("C", "RA") => cel2ran(gradzahl),

        ("F", "C") => fah2cel(gradzahl),
        ("F", "K") => fah2kel(gradzahl),
        ("F", "RE") => fah2rea(gradzahl),
        ("F", "RA") => fah2ran(gradzahl),

        ("K", "C") => kel2cel(gradzahl),
        ("K", "F") => kel2fah(gradzahl),
        ("K", "RE") => kel2rea(gradzahl),
        ("K", "RA") => kel2ran(gradzahl),

        ("RE", "C") => rea2cel(gradzahl),
        ("RE", "F") => rea2fah(gradzahl),
        ("RE", "RE") => rea2kel(gradzahl),
        ("RE", "RA") => rea2ran(gradzahl),

        ("RA", "C") => ran2cel(gradzahl),
        ("RA", "F") => ran2fah(gradzahl),
        ("RA", "K") => ran2kel(gradzahl),
        ("RA", "RE") => ran2rea(gradzahl),

        _ => Err("Ungültige Einheitenkombination".into()),
    }
}
