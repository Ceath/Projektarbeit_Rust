# Aufgabe 2: Umdrehen ("reverse") eines Strings
[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(4))

Der komplette Code befindet sich in [src/main.rs](src/main.rs).

Ausführen mithilfe des befehls `cargo run` ausgehend von diesem ordner.

## Anmerkungen
### Allgemein
- `reverse` entspricht der Funktion `reverse(char*)` aus Teilaufgabe a) in einer C ähnlichen Art.
- `reverse_rust` stellt ein Lösungsweg auf Rust Art dar.
- `reverse_rec` entspricht der Funktion aus der Teilaufgabe b), ebenfalls C nahe geschrieben.
- Im Vergleich zur Aufgabe 1 ist in Aufgabe 2 nicht nur das Lesen von Zeicheenketten notwendig sondern auch das /schreiben/erzeugen eines neuen Strings
### reverse
- Als Parameter bekommt `reverse` einen `char` Iterator.
- In der ersten `while` Schleife wird die länge der Eingabe ermittelt. Zwar haben Iteratoren in Rust auch eine Funktion `size_hint()` welche die Anzahl an Elementen zurückgibt wurde hier Analog zu C die Länge mit einer Schleife festgelegt.
- Iteratoren können sich in Rust nur in eine Richtung bewegen, deshalb wird beim Erkennen der Länge eine Kopie von `text_pointer` verwendet und mit einer zweiten Iteration `reverse_string` befüllt. In C wäre es möglich den Pointer wieder umzudrehen, ohne ihn zu Kopieren.
- `reverse_string` ist von Typ `Vec<char>`, Rust ist in der Lage den Typ anhand des Kontext (hier die Zeile `reverse_string = vec!['\0'; size];`) zu erkenne ohne das er Konkret definiert wurde.
- Bei `reverse_string` handelt es sich um einen `Vec<char>` und nicht um einen `String` da diese nicht dafür gedacht sind einzelne `char` Werte zu bearbeiten. (Es ist zwar möglich aber nur mithilfe unsicheren Codes)
- Bei `vec!['\0'; size]` handelt es sich um ein Macro welches einen `Vec` erzeugt bei dem die ersten `size` einträge den Wert `\0` haben.
- Am Ende wird `reverse_string` zu einen String transformiert. `into_iter()` erzeugt einen Konsumierenden Iterator(bedeutet `reverse_string`wird geleert) und `collect()` Konvertiert diesen zu einen neuen String. Da String den Trait  `FromIterator<char>` implementiert hat. Alternativ könnte auch `String::from_iter(reverse_string.into_iter())` geschrieben werden.

### reverse_rust
- Iteratoren besitzten wenn sie den trait `DoubleEndedIterator` implementieren eine Funktion `rev()`. Diese gibt einen Umgedrehte Version des Iterators zurück, Konkret bedeutet das das der Iterator vom voherigen Ende sich auf die Vorherige Position zubewegt. Es ist aber dabei nicht möglich auf das Element zuzugreifen welches vor dem Aktuellen liegt.

### Part B
- `length` funktioniert analog zum erkennen der Länge aus `reverse` mithilfe eines `char Iterators`
- In `copy` wird das erste mal ein `IterMut` verwendet. Dabei handelt es sich um einen Speziellen Iterator welche erlaubt die Referenzierten Werte zu verändern. Da `next` ein `Option` zurückgibt muss `unwrap()` verwendet werden um auf den Wert zuzugreifen, sollte `input_pointer` oder `output_pointer` nicht mindestens `n` elemente beinhandeln würde dies zu einer `Panic` führen. `*` dereferenziert eine Referenz aus einen `&char` wird also ein `char`.  
- In `put_back` wird immer ein neuer `Vec<char>` mit der größe von `input` + 1 erzeugt und dieser nach dem befüllen auch zurückgegeben. Da Iteratoren nur Referenzen besitzen können und diese nicht lange genug Leben (da `output` nach funktionsaufruf zerstört wird) kann kein Iterator zurückgegeben werden. Rust würde dies statisch erkennen und nicht erlauben dies zu Kompilieren.
- Jeder aufruf von `reverse_rec` ergreift besitzt von einem `Vec<char>` in der Variable `reverse`, das bedeutet das am zum umdrehen eines Strings der länge n auch n Vektoren exestieren welche und erst bei am ende der Rekursiven Funktion zerstört werden (außer der erste welche den umgedrehten String enthält), diese funktion ist dadurch sehr kostenaufwändig

### Testrahmen
- Beim Testrahmen sind die Felder des Structs `TestCase` vom Type `&str` und nicht `String` wie es in Aufgabe 1 der fall ist. Es ist also möglich das Structs Referenzen verwenden, dafür muss aber ähnlich wie bei der Rückgabe von Referenzen von Funktionen eine Lifetime angegeben werden, um zu garantieren das die Lebensdauer eines Struct Objekts die der Felder mindestestens gleich sind. In `main()` werden mehrere Instanzen von `TestCase` erzeugt, als Argumente werden hier String Literale übergeben. In Rust werden String Literale beim Kompilieren Statisch angelegt. `"Hallo"` verweist also intern auf eine String Instanz mit Statischer lebensdauer und wenn ein `"Hallo"` an einder Stelle verwendet wird verweißt es auf die selbe String Instanz.
- Beim ersten Eintrag von `test_functions` handelt es sich um ein closure, während die beiden anderen Einträge einfach nur verweise an Funktion sind. Bei closures werden zwichen zwei | die Parameter definiert, danach folgt der Funktionsinhalt. Handelt es sich dabei um eine Funktion welche einen Wert zurückgibt kann Rust dies statisch erkennen der Rückgabetyp muss im gegensatzt zu normalen Funktions definitionen also nicht angegeben werden, gleiches gilt auch für die Parameter hier ist die Typangabe optional.