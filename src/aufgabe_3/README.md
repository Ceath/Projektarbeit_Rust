# Aufgabe 3: String Klasse mit überladenen Operatoren
[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(5))

## Hinweise

Der Code ist in mehrere Dateien aufgeteilt. 
- In [src/main.rs](src/main.rs) befindet sich der Code für das ausführen als Binary.
- In [src/lib.rs](src/lib.rs) befindet sich die Definition des CString Structs sowie dazugehörige unit-tests, dies ist eine Library-file welche an anderen stellen (wie [main.rs](src/main.rs)) importiert werden kann.
- In [tests/integration_test.rs](tests/integration_test.rs) befinden sich integration tests.

Ausführen von [main.rs](src/main.rs) mit dem Befehl `cargo run` \
Ausführen der Unit- und Integration-Tests mit dem Befehl `cargo test`

## Unit und Integration Tests
Rust unterstützt nativ das schreiben und ausführen von Unit- und Integration-Tests.  
Unit-Tests werden in Rust in die Datei geschrieben in welcher sich der zu testende Code befindet und bekommen meist ein eigenes Module `tests`. Durch die angabe `#[cfg(test)]` weiß der Rust Compiler das diese Module nur für Tests gebaut werden muss. Das Module selbst besteht aus Testfunktionen welche durch die Annotation `#[test]` erkennbar sind, in diesen können mithilfe mehrerer Assert funktionen Testfälle geschrieben werden.  

Integration-Tests befindet sich dagegen in eigenen Dateien im `tests` Ordner, hier erkennt der Compiler ebenfalls aufgrund der Zeile `#[cfg(test)]` das es sich um einen Test handelt.  
Sowohl Integration-, Unit- und auch Dokumentation-tests(hier nicht behandelt) können mit Cargo einfach via `cargo test` ausgeführt werden, dabei werden alle Testfälle durchlaufen und falls ein Assert nicht besteht oder eine (ungewünschte) Panic verursacht wird fällt der Testfall durch, ansonsten wird er bestanden.

## Display
Mithilfe des Display Traits ist es möglich Typen als Strings darzustellen. Dies geht funktioniert dann entweder mit der  `to_string()` methode oder in Format Strings welche z.B. beim `println!` oder `format!` Macro verwendet werden.  
Format Strings sind Strings in denen es möglich ist  `{}` durch Parameter zu ersetzen, ähnlich wie bei `printf` von C nur das kein genauer Typ angegeben werden muss.

## Unterteilung von Ausführbaren Code und Bibliotheken
Rust Projekte können ausführbare Anwendungen(.exe(Windows)) oder Bibliotheken(.dll oder .lib bei c(++)) sein, es ist auch möglich beide in einem Projekt gemeinsam zu verwenden. Zum erkenne welche Dateien für welchen zweck gedacht sind werden vorgegebene Dateinamen verwendet. In main.rs steht der Code für die executable während in lib.rs der Code für die Bibliothek steht.  

Aufgabe 3 ist in main.rs und lib.rs unterteilt, dadurch ist es möglich sowohl den Code auszuführen als auch den Inhalt von lib.rs in anderen Crates als abhängigkeit zu verwenden. Zudem wird dadurch auch das verwenden von Unit- und Integration-Tests ermöglicht (Hauptgrund für die Unterteilung in Aufgabe 3).

## Anmerkungen zum Code

### [main.rs](src/main.rs)
- [main.rs](src/main.rs) besteht aus mehren anschaulichen Beispielen des CString structs. Die `use` anweisung in der ersten Zeile referenziert dabei den inhalt einer Crate namens aufgabe_3, dabei handelt es sich um den inhalt aus [lib.rs](src/lib.rs) welcher als abhänigkeit importiert wird.

### [lib.rs](src/lib.rs)
- In Rust exestieren Konstruktoren nicht in der Selben form wie in C++, da eigentlich Instanzen von Structs mithilfe der zuweisung der einzelnen felder geschieht. Trotzdem können funktionen welche die Funktionalität von Konstruktoren besitzten geschrieben werden. Dabei handelt es sich dann um Statische Funktionen welche anhand Parameter Instanzen erzeugen, dies ermöglicht auch das zuweisen der Werte für private Felder.
- Das Überladen von Funktionen ist in Rust nur beschränkt möglich (via Traits), deshalb muss für jeden Konstruktor ein einzigartiger Name verwendet werden
- Der Unterschied zwischen dem Kopierkonstruktor `new_string` und dem Move-Konstruktor `new_move` ist anhand des Namens aber auch anhand der Parameter zu erkennen. Ein Kopierkonstruktor benötigt lediglich eine Referenz auf das zu Kopierende objekt, während der Move-Konstruktor eine CString Instance direkt ohne Referenz benötigt was in Rust immer bei nicht Kopierbaren Objekten darauf hinweist das das Objekt konsumiert wird. Rust würde das fehlerhafte aufrufen dieser Funktionen aber beim Kompilieren erkenne und mögliche Fehler dadurch verhindern.
- Anstelle des Kopierkonstruktors `new_string` wäre es auch möglich entweder den Trait `Clone` oder `Copy` für CString zu implementieren. `Clone` implementiert einfach eine Funktion `clone()` welche in Rust in den Regel als Kopierkonstruktor verwendet wird. `Copy` wird dagegen verwendet wenn ein Typ bit-kopierbar ist. Dabei wird das Verhalten beim verwenden des `=` Operators verändert, anstelle einer Move-Semantik stellt er dann eine Kopier-Semantik dar, die Instanze wird also immer Kopiert und nicht Konsumiert. Beispiel:
    ```
    let s1 = CString::new_empty();
    let s2 = s1; /* Wenn CString `Copy` implementiert ist danach s1 noch verfügbar und s2 eine Bit-Kopie von s1. S1 wäre dann noch gültig*/
    println!("{}", s1); /* Nur Möglich mit `Copy` ansonsten würde Rust nicht Kompilieren da s1 nach s2 bewegt wurde */
    ```
    Normalerweise implementieren nur Primärdatentypen wie `i32`, `char`, `f32` etc. den `Copy` Trait, da er für Komplexe Datentypen nach dem Rust Idiom selten Sinnvoll ist. `String` in Rust implementiert deshalb ebenfalls nicht den `Copy` Trait sondern stadtdessen `Clone`.
- Operatoren werden in Rust durch das Implementieren von Traits überschrieben.