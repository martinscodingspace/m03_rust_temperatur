# m03_rust_temperatur

Das Programm m03_rust_temperatur.rs beinhaltet folgenden Zweck:

Programmierung eines Tools, welches Temperaturen in verschiedene
Einheiten umwandelt. Das Programm soll es dem Nutzer ermöglichen,
einen Zahlenwert und die jeweilige Einheit (z. B. Celsius,
Fahrenheit oder Kelvin) anzugeben und anschließend die Zieleinheit
zu wählen.
Das Ergebnis der Umrechnung soll anschließend formatiert
ausgegeben werden. Die Berechnungsfunktionen sollen sauber
voneinander getrennt und in einem eigenen Modul untergebracht
werden, sodass sie unabhängig getestet werden können.
Dazu gehören
zu jeder Umrechnungsfunktion passende Unit-Tests, die typische und
fehlerhafte Eingaben prüfen.
Alle Funktionen sind mit
Dokumentationskommentaren zu versehen, damit später eine automatisch
generierte Referenz entsteht.
Optional:
• Erweiterung um zusätzliche Einheiten
• Automatische Erkennung der Einheit aus der Nutzereingabe
• Ausgabe mit Nachkommagenauigkeit nach Benutzervorgabe
• Interaktive CLI-Auswahl über Menüs oder Eingabeaufforderungen

///  # Beschreibung
///  Die Umrechnung erfolgt innerhalb von fünf Temperatureinheiten
///
///  Anders Celsius (1701 - 1744) war ein schwedischer Astronom,
///  Mathematiker und Physiker, der vor allem durch das von ihm 1742
///  eingeführte Thermometersystem bekannt ist.
///
///  Daniel Gabriel Fahrenheit (1686 - 1736) war ein deutscher Physiker
///  und Erfinder von Messinstrumenten. Nach ihm wurde die Temperatureinheit
///  Grad Fahrenheit (°F) benannt.
///
///  Das Kelvin wurde nach William Thomson (1786 - 1849), dem späteren Lord Kelvin, benannt,
///  der die thermodynamische Temperaturskala vorschlug. Bis 1967 lautete der
///  Einheitenname „Grad Kelvin“, das Einheitenzeichen war °K.
///
///  René-Antoine Ferchault de Réaumur (1683 - 1757) war ein französischer Natur-
///  und Materialforscher mit einem weiten Interessen- und Arbeitsgebiet.
///  So beschäftigte er sich unter anderem mit der Temperaturmessung (Réaumur-Skala)
///  Sie wird heute nur noch sehr selten verwendet, so zum Beispiel in der
///  Süßwarenindustrie und häufiger noch bei der Alp-Käseherstellung in der Schweiz.
///
///  William John Macquorn Rankine (1820 - 1872) war ein schottischer Physiker und Ingenieur.
///  Die Rankine-Skala ist eine Temperaturskala, die wie die Kelvin-Skala beim absoluten
///  Temperaturnullpunkt ihren Nullwert hat, jedoch im Gegensatz zu dieser den Skalenabstand
///  der Fahrenheit-Skala verwendet. Die Rankine-Skala, die weniger bekannt ist als Celsius
///  wird hauptsächlich in der thermodynamischen Technik (Energiegewinnung) verwendet.
