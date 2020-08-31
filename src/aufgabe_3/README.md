# Aufgabe 3: String Klasse mit überladenen Operatoren
[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(5))

## Hinweise

Der Code ist in mehrere Dateien aufgeteilt. 
- In [src/main.rs](src/main.rs) befindet sich der Code für das Ausführen als Binary.
- In [src/lib.rs](src/lib.rs) befindet sich die Definition des `CString` Structs sowie dazugehörige Unit-Tests, dies ist eine Bibliotheksdatei, welche an anderen stellen (wie [main.rs](src/main.rs)) importiert werden kann.
- In [tests/integration_test.rs](tests/integration_test.rs) befinden sich Integration-Tests.

Ausführen von [main.rs](src/main.rs) mit dem Befehl `cargo run` \
Ausführen der Unit- und Integration-Tests mit dem Befehl `cargo test`

## Unit und Integration Tests
Rust unterstützt nativ das Schreiben und Ausführen von Unit- und Integration-Tests.  
Unit-Tests werden in Rust in die Datei geschrieben, in welcher sich der zu testende Code befindet, und bekommen meist ein eigenes Module. Durch die Angabe `#[cfg(test)]` weiß der Rust Compiler, dass dieses Modul nur für Tests kompiliert werden muss. Das Modul selbst besteht aus Testfunktionen, welche durch die Annotation `#[test]` erkennbar sind, in diesen können mithilfe mehrerer `assert` Funktionen Testfälle geschrieben werden.  

Integration-Tests befindet sich dagegen in eigenen Dateien im `tests` Ordner, hier erkennt der Compiler ebenfalls aufgrund der Zeile `#[cfg(test)]` das es sich um einen Test handelt.  
Sowohl Integration-, Unit- und auch Documentation-Tests(hier nicht behandelt) können mit Cargo einfach via `cargo test` ausgeführt werden, dabei werden alle Testfälle durchlaufen, und falls ein `assert` nicht besteht oder eine (unerwünschte) Panic verursacht wird, fällt der Testfall durch, ansonsten wird er bestanden.

## Display
Mithilfe des `Display` Traits ist es möglich, Typen als `String` darzustellen. Dies funktioniert dann entweder mit der `to_string()` Funktion oder in einem Format String, welche z. B. im `println!` oder `format!` Macro verwendet werden.

Format Strings sind Stringkonstanten, in denen es möglich ist `{}` durch Parameter zu ersetzen, ähnlich wie bei `printf` von C, nur das kein genauer Typ angegeben werden muss.

## Unterteilung von Ausführbaren Code und Bibliotheken
Rust Projekte können ausführbare Anwendungen(`.exe`(Windows)) oder Bibliotheken(`.dll` oder `.lib` bei c(++)) sein, es ist auch möglich, beide in einem Projekt gemeinsam zu verwenden. Zum Erkennen, welche Dateien für welchen Zweck gedacht sind, werden vorgegebene Dateinamen verwendet. In `main.rs` steht der Code für die executable, während in `lib.rs` der Code für die Bibliothek steht.  

Aufgabe 3 ist in `main.rs` und `lib.rs` unterteilt, dadurch ist es möglich, sowohl den Code auszuführen als auch den Inhalt von `lib.rs` in anderen Crates als externe Abhängigkeit zu verwenden. Zudem wird dadurch auch das Verwenden von Unit- und Integration-Tests ermöglicht (Hauptgrund für die Unterteilung in Aufgabe 3).

## Anmerkungen zum Code

### [main.rs](src/main.rs)
- [main.rs](src/main.rs) besteht aus mehren anschaulichen Beispielen des `CString` Structs. Die `use` Anweisung in der ersten Zeile referenziert dabei den Inhalt einer Bibliothek namens `aufgabe_3`, dabei handelt es sich um den Inhalt aus [lib.rs](src/lib.rs), welcher als Abhängigkeit importiert wird.

### [lib.rs](src/lib.rs)
- In Rust existieren Konstruktoren nicht in der gleichen Form wie in C++, da eigentlich Instanzen von Structs mithilfe einer zuweisung der einzelnen Felder erzeugt werden. Trotzdem können Funktionen, welche die Funktionalität von Konstruktoren besitzen, geschrieben werden. Dabei handelt es sich dann um statische Funktionen, welche anhand von Parametern eine Instanz erzeugen. Dies ermöglicht auch das Zuweisen der Werte von private Felder.
- Das Überladen von Funktionen ist in Rust nur beschränkt möglich (via Traits), deshalb muss für jeden Konstruktor ein einzigartiger Name verwendet werden.
- Der Unterschied zwischen dem Kopierkonstruktor `new_string` und dem Move-Konstruktor `new_move` ist anhand des Namens aber auch anhand der Parameter zu erkennen. Ein Kopierkonstruktor benötigt lediglich eine Referenz auf das zu kopierende Objekt, während der Move-Konstruktor eine `CString` Instanz ohne Referenz benötigt, was in Rust immer bei nicht kopierbaren Objekten darauf hinweist, dass das Objekt konsumiert wird. Rust würde das fehlerhafte Aufrufen dieser Funktionen aber beim Kompilieren erkennen und mögliche Fehler dadurch verhindern.
- Anstelle des Kopierkonstruktors `new_string` wäre es auch möglich, entweder den Trait `Clone` oder `Copy` für `CString` zu implementieren.  
`Clone` implementiert einfach eine Funktion `clone()`, welche in Rust als Kopierkonstruktor verwendet wird.  
`Copy` wird dagegen verwendet, wenn ein Typ Bit-Kopierbar ist. Dabei wird das Verhalten beim Verwenden des `=` Operators verändert. Anstelle eines Move-Operators stellt dieser dann eine Kopier-Operator dar, die Instanz wird also immer kopiert und ohne das die Ownership übergeben wird.  
Beispiel:
```
let s1 = CString::new_empty();
let s2 = s1; /* Wenn CString `Copy` implementiert ist danach s1 noch verfügbar und s2 eine Bit-Kopie von s1. S1 wäre dann noch gültig*/
println!("{}", s1); /* Nur Möglich mit `Copy` ansonsten würde Rust nicht Kompilieren da s1 nach s2 bewegt wurde */
```
Normalerweise implementieren nur Primärdatentypen wie `i32`, `char`, `f32` etc. den `Copy` Trait, da er für komplexe Datentypen nach dem Rust-Idiom selten sinnvoll ist. `String` in Rust implementiert deshalb ebenfalls nicht den `Copy` Trait, sondern stadtdessen `Clone`.
- Operatoren werden in Rust durch das Implementieren von Traits überschrieben.