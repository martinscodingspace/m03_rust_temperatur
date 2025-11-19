//! Das Modul temperatur.rs beinhaltet folgenden Zweck:
//!
//! Zurverfügungstellung der erforderlichen Berechnungsfunktionen für die
//! • Umwandlung der Temperaturen in verschiedene Einheiten
//! • Formatierung der Ergebnisse der Umrechnungen
//! • unabhängig voneinander getestete Berechnungsfunktionen
//! optional:
//! • Erweiterung um zusätzliche Einheiten
//! • Automatische Erkennung der Einheit aus der Nutzereingabe
//! • Ausgabe mit Nachkommagenauigkeit nach Benutzervorgabe
//! • Interaktive CLI-Auswahl über Menüs oder Eingabeaufforderungen

use std::process::Command;
use std::io::{self, Write};
use colored::Colorize;

pub fn degree() -> f64 {
    // nur bei korrekter Eingabe soll die Schleife verlassen werden
    // ansonsten Wiederholung
    loop {
        print!  ("===     Eingabe in Grad   :  ");
        io::stdout().flush().unwrap();
        let mut eingabe = String::new();
        io::stdin().read_line(&mut eingabe).unwrap();

        match eingabe.trim().parse::<f64>() {
            Ok(val) => return val,
            Err(_) => println!("Bitte eine gültige Zahl eingeben."),
        }
    }
}

/// Liest eine Temperatureinheit (C/F/K).
pub fn scale_in() -> String {
    // nur bei korrekter Eingabe soll die Schleife verlassen werden
    // ansonsten Wiederholung
    loop {
        println!("============================================================");
        println!("===     Einheiten         :  (C / F / K / RE / RA)       ===");
        println!("============================================================");
        print!("===     Eingabe der Quell-Temperatureinheit   :  ");
        io::stdout().flush().unwrap();
        let mut eingabe = String::new();
        io::stdin().read_line(&mut eingabe).unwrap();
        println!("============================================================");
        let einheit = eingabe.trim().to_uppercase();

        // Großschreibung für einheitliche Erkennung
        if ["C", "F", "K", "RE", "RA"].contains(&einheit.as_str()) {
            return einheit;
        } else {
            let f1 = "============================================================";
            let f2 = "===     Ungültige Eingabe  -  Bitte Wiederholen !!!      ===";
            let f3 = "============================================================";
            println!("{}", f1.red());
            println!("{}", f2.red());
            println!("{}", f3.red());
        }
    }
}

/// Liest eine Temperatureinheit (C/F/K).
pub fn scale_out() -> String {
    // nur bei korrekter Eingabe soll die Schleife verlassen werden
    // ansonsten Wiederholung
    loop {
        print!("===     Eingabe der Ziel -Temperatureinheit   :  ");
        io::stdout().flush().unwrap();
        let mut eingabe = String::new();
        io::stdin().read_line(&mut eingabe).unwrap();
        println!("============================================================");
        let einheit = eingabe.trim().to_uppercase();

        // Großschreibung für einheitliche Erkennung
        // der Accent d'Aigu muss ebenfalls erkannt werden können   ;-)
        if ["C", "F", "K", "RE", "RA"].contains(&einheit.as_str()) {
            return einheit;
        } else {
            println!("Ungültige Einheit.");
        }
    }
}



//*************************************************************************
//*************************************************************************
/// Temperatur-Umwandlungsfunktionen.
/// Jede Funktion gibt `Result<f64, String>` zurück, falls Eingaben ungültig

pub fn cel2fah(c: f64) -> Result<f64, String> {
    if c < -273.15 {
        return Err("Celsius kann nicht kleiner sein als -273,15 C".into());
    }
    Ok(c * 1.8 + 32.00)
}

pub fn cel2kel(c: f64) -> Result<f64, String> {
    if c < -273.15 {
        return Err("Celsius kann nicht kleiner sein als -273,15 C".into());
    }
    Ok(c + 273.15)
}

pub fn cel2rea(c: f64) -> Result<f64, String> {
    if c < -273.15 {
        return Err("Celsius kann nicht kleiner sein als -273,15 C".into());
    }
    Ok(c * 0.8)
}

pub fn cel2ran(c: f64) -> Result<f64, String> {
    if c < -173.15 {
        return Err("Celsius kann nicht kleiner sein als -273,15 C".into());
    }
    Ok(c * 1.8 + 491.67)
}

pub fn fah2cel(f: f64) -> Result<f64, String> {
    if f < -459.67 {
        return Err("Fahrenheit kann nicht kleiner sein als -459,67 F".into());
    }
    Ok((f - 32.0) / 1.8)
}

pub fn fah2kel(f: f64) -> Result<f64, String> {
    if f < -459.67 {
        return Err("ahrenheit kann nicht kleiner sein als -459,67 F".into());
    }
    Ok((f + 459.7) * 5.0 / 9.0)
}

pub fn fah2rea(f: f64) -> Result<f64, String> {
    if f < -459.67 {
        return Err("Fahrenheit kann nicht kleiner sein als -459,67 F".into());
    }
    Ok((f - 32.0) / 2.25)
}

pub fn fah2ran(f: f64) -> Result<f64, String> {
    if f < -459.67 {
        return Err("Fahrenheit kann nicht kleiner sein als -459,67 F".into());
    }
    Ok(f + 459.67)
}

pub fn kel2cel(k: f64) -> Result<f64, String> {
    if k < 0.0 {
        return Err("Kelvin kann nicht negativ sein".into());
    }
    Ok(k - 273.15)
}

pub fn kel2fah(k: f64) -> Result<f64, String> {
    if k < 0.0 {
        return Err("Kelvin kann nicht negativ sein".into());
    }
    Ok(k * 1.8 -459.67)
}

pub fn kel2rea(k: f64) -> Result<f64, String> {
    if k < 0.0 {
        return Err("Kelvin kann nicht negativ sein".into());
    }
    Ok((k - 273.15) / 1.25)
}

pub fn kel2ran(k: f64) -> Result<f64, String> {
    if k < 0.0 {
        return Err("Kelvin kann nicht negativ sein".into());
    }
    Ok(k *1.8)
}

pub fn rea2cel(r: f64) -> Result<f64, String> {
    if r < -218.52 {
        return Err("Réaumur kann nicht kleiner sein als -218,52 Ré".into());
    }
    Ok(r * 1.25)
}

pub fn rea2fah(r: f64) -> Result<f64, String> {
    if r < -218.52 {
        return Err("Réaumur kann nicht kleiner sein als -218.52 Ré".into());
    }
    Ok(r * 2.25 + 32.0)
}

pub fn rea2kel(r: f64) -> Result<f64, String> {
    if r < -218.52 {
        return Err("Réaumur kann nicht kleiner sein als -218,52 Ré".into());
    }
    Ok(r * 1.25 + 273.15)
}

pub fn rea2ran(r: f64) -> Result<f64, String> {
    if r < -218.52 {
        return Err("Réaumur kann nicht kleiner sein als -218.52 Ré".into());
    }
    Ok(r * 2.25 + 491.67)
}

pub fn ran2cel(ra: f64) -> Result<f64, String> {
    if ra < 0.0 {
        return Err("Rankine kann nicht negativ sein".into());
    }
    Ok(ra * 5.0 / 9.0 -273.15)
}

pub fn ran2fah(ra: f64) -> Result<f64, String> {
    if ra < 0.0 {
        return Err("Rankine kann nicht negativ sein".into());
    }
    Ok(ra - 459.67)
}

pub fn ran2kel(ra: f64) -> Result<f64, String> {
    if ra < 0.0 {
        return Err("Rankine kann nicht negativ sein".into());
    }
    Ok(ra * 5.0 / 9.0)
}

pub fn ran2rea(ra: f64) -> Result<f64, String> {
    if ra < 0.0 {
        return Err("Rankine kann nicht negativ sein".into());
    }
    Ok((ra - 491.67) / 2.25)
}

//***********************************************************
//***********************************************************

pub fn clear_screen() {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("clear")
            .status()
            .unwrap();
    }
}



/// Hinweise zu den Tests
/// Aufgrund von Rundungsfehlern bei Fließkommazahlen
/// muss in manchen Fällen bei den Tests ab einer
/// bestimmten Nachkommastelle gerundet werden
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tt_cel2fah() {
        assert_eq!(cel2fah(0.0), Ok(32.0));
        assert_eq!(cel2fah(100.0), Ok(212.0));
    }

    #[test]
    fn tt_cel2kel() {
        assert_eq!(cel2kel(0.0), Ok(273.15));
        assert_eq!(cel2kel(20.0), Ok(293.15));
    }

    #[test]
    fn tt_cel2rea() {
        assert_eq!(cel2rea(100.0), Ok(80.0));
        assert_eq!(cel2rea(0.0), Ok(0.0));
    }

    #[test]
    fn tt_cel2ran() {
        assert_eq!(cel2ran(0.0), Ok(491.67));

        // Problem Gleitkommazahlenvergleich bzgl. Rundungsfehler
        // Ergebnis aus Berechnung abgerundet, benötigt integrierten Trait
        let erg_fn = cel2ran(50.0).unwrap();
        let num_nachkommastellen = 4;
        // Multiplizieren mit 10^2 für Zwischenschritt
        let multipliziert = erg_fn * 10f64.powi(num_nachkommastellen);
        // Runden vor dem nächsten Schritt
        let gerundet = multipliziert.round();
        // Wieder durch 10^2 dividieren
        let endergebnis = gerundet / 10f64.powi(num_nachkommastellen);
        assert_eq!(endergebnis, 581.67);

    }

    #[test]
    fn tt_fah2cel() {
        assert_eq!(fah2cel(32.0), Ok(0.0));
        assert_eq!(fah2cel(-22.0), Ok(-30.0));
    }

    #[test]
    fn tt_fah2kel() {
        // Problem Gleitkommazahlenvergleich bzgl. Rundungsfehler
        // Ergebnis aus Berechnung abgerundet, benötigt integrierten Trait
        let erg_fn = fah2kel(0.0).unwrap();
        let num_nachkommastellen = 2;
        // Multiplizieren mit 10^2 für Zwischenschritt
        let multipliziert = erg_fn * 10f64.powi(num_nachkommastellen);
        // Runden vor dem nächsten Schritt
        let gerundet = multipliziert.round();
        // Wieder durch 10^2 dividieren
        let endergebnis = gerundet / 10f64.powi(num_nachkommastellen);
        assert_eq!(endergebnis, 255.39);
    }

    #[test]
    fn tt_fah2rea() {
        assert_eq!(fah2rea(50.0), Ok(8.0));
    }

    #[test]
    fn tt_fah2ran() {
        assert_eq!(fah2ran(-59.67), Ok(400.0));
    }

     #[test]
    fn tt_kel2cel() {
        // assert_eq!(kel2cel(100.0), Ok(-173.15));
        // Problem Gleitkommazahlenvergleich bzgl. Rundungsfehler
        // Ergebnis aus Berechnung abgerundet, benötigt integrierten Trait
        let erg_fn = kel2cel(100.0).unwrap();
        let num_nachkommastellen = 2;
        // Multiplizieren mit 10^2 für Zwischenschritt
        let multipliziert = erg_fn * 10f64.powi(num_nachkommastellen);
        // Runden vor dem nächsten Schritt
        let gerundet = multipliziert.round();
        // Wieder durch 10^2 dividieren
        let endergebnis = gerundet / 10f64.powi(num_nachkommastellen);
        assert_eq!(endergebnis, -173.15);
    }

    #[test]
    fn tt_kel2fah() {
        assert_eq!(kel2fah(66.0), Ok(-340.87));
    }

    #[test]
    fn tt_kel2rea() {
        assert_eq!(kel2rea(273.15), Ok(0.0));
    }

    #[test]
    fn tt_kel2ran() {
        assert_eq!(kel2ran(100.00), Ok(180.0));
    }

    #[test]
    fn tt_rea2cel() {
        assert_eq!(rea2cel(1.00), Ok(1.25));
    }

    #[test]
    fn tt_rea2fah() {
        assert_eq!(rea2fah(1.00), Ok(34.25));
    }

    #[test]
    fn tt_rea2kel() {
        assert_eq!(rea2kel(10.00), Ok(285.65));
    }

    #[test]
    fn tt_rea2ran() {
        assert_eq!(rea2ran(1.00), Ok(493.92));
    }

    #[test]
    fn tt_ran2cel() {
        // assert_eq!(ran2cel(1.00), Ok(-272.59));
        // Problem Gleitkommazahlenvergleich bzgl. Rundungsfehler
        // Ergebnis aus Berechnung abgerundet, benötigt integrierten Trait
        let erg_fn = ran2cel(1.0).unwrap();
        let num_nachkommastellen = 2;
        // Multiplizieren mit 10^2 für Zwischenschritt
        let multipliziert = erg_fn * 10f64.powi(num_nachkommastellen);
        // Runden vor dem nächsten Schritt
        let gerundet = multipliziert.round();
        // Wieder durch 10^2 dividieren
        let endergebnis = gerundet / 10f64.powi(num_nachkommastellen);
        assert_eq!(endergebnis, -272.59);
    }

    #[test]
    fn tt_ran2fah() {
        assert_eq!(ran2fah(491.67), Ok(32.0));
    }

    #[test]
    fn tt_ran2kel() {
        // assert_eq!(ran2kel(40.0), Ok(22.222));
        // Problem Gleitkommazahlenvergleich bzgl. Rundungsfehler
        // Ergebnis aus Berechnung abgerundet, benötigt integrierten Trait
        let erg_fn = ran2kel(40.0).unwrap();
        let num_nachkommastellen = 3;
        // Multiplizieren mit 10^2 für Zwischenschritt
        let multipliziert = erg_fn * 10f64.powi(num_nachkommastellen);
        // Runden vor dem nächsten Schritt
        let gerundet = multipliziert.round();
        // Wieder durch 10^2 dividieren
        let endergebnis = gerundet / 10f64.powi(num_nachkommastellen);
        assert_eq!(endergebnis, 22.222);
    }

    #[test]
    fn tt_ran2rea() {
        // assert_eq!(ran2rea(50.00), Ok(-196.30));
        // Problem Gleitkommazahlenvergleich bzgl. Rundungsfehler
        // Ergebnis aus Berechnung abgerundet, benötigt integrierten Trait
        let erg_fn = ran2rea(50.0).unwrap();
        let num_nachkommastellen = 1;
        // Multiplizieren mit 10^2 für Zwischenschritt
        let multipliziert = erg_fn * 10f64.powi(num_nachkommastellen);
        // Runden vor dem nächsten Schritt
        let gerundet = multipliziert.round();
        // Wieder durch 10^2 dividieren
        let endergebnis = gerundet / 10f64.powi(num_nachkommastellen);
        assert_eq!(endergebnis, -196.30);
    }

}
