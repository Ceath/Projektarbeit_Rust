# Aufgabe 2: Umdrehen ("reverse") eines Strings
[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(4))

## Hinweise

Der komplette Code befindet sich in [src/main.rs](src/main.rs).

Ausführen mithilfe des befehls `cargo run` ausgehend von diesem ordner.

Es existieren drei unterschiedliche Lösungen für `reverse`.
- `reverse()` entspricht der Funktion `reverse(char*)` aus Teilaufgabe a) mit einem Algorithmus welcher ählnich in C umgesetzt werden könnte.
- `reverse_rust()` stellt ein Lösungsweg auf Rust Art dar.
- `reverse_rec()` entspricht der Funktion aus der Teilaufgabe b), ebenfalls C nahe geschrieben.

Im Vergleich zur Aufgabe 1 ist in Aufgabe 2 nicht nur das Lesen von Zeicheenketten notwendig sondern auch das veränder und erzeugen eines Strings.


## IterMut und Rev
IterMut und Rev sind besondere Arten von Iteratoren, welche in Aufgabe 2 verwendet wurden.  
IterMut gibt mit next() ein Option<&mut T> zurück, dadurch ist es möglich das element zu verändern auf welches next() verweist. Da es sich um eine Referenz handelt muss diese jedoch zuerst mit dem `*`-Operator dereferenziert werden.  
Rev ist ein umgekehrter Iterator. Wenn ein Iterator den Trait `DoubleEndedIterator` implementiert ist es möglich mithilfe der `rev()` funktion den Iterator umzudrehen. Dabei wird vom letzten Element ausgehend auf das aktuelle Element verwiesen, Elemente über welche zuvor beim Originalen Iterator mit next() iteriert wurde bleiben dadurch weiterhin unzugänglich.

## Verändern von Strings
In Rust ist das ändern eines chars an einer beliebigen Position im String nicht so einfach möglich wie in C(++), grund dafür ist die UTF-8 Kodierung, da String intern als Byte Vector verwaltet werden, aber ein Zechen aus mehreren Bytes bestehen kann. Deshalb wurde in Aufgabe 2 und 3 anstellen eines Strings ein `Vec<char>` verwendet um das verändern der Zeichen einfach zu ermöglichen.

## Anmerkungen zum Code
### reverse
- In der ersten `while` Schleife wird die länge der Eingabe ermittelt. Zwar haben Iteratoren in Rust auch eine Funktion `size_hint()` welche die Anzahl an Elementen zurückgibt wurde hier Analog zu C die Länge mit einer Schleife festgelegt.
- Iteratoren können sich in Rust nur in eine Richtung bewegen, deshalb wird beim Erkennen der Länge eine Kopie von `text_pointer` verwendet und mit einer zweiten Iteration `reverse_string` befüllt. In C wäre es möglich den Pointer wieder umzudrehen, ohne ihn zu Kopieren.
- `reverse_string` ist von Typ `Vec<char>`, Rust ist in der Lage den Typ anhand des Kontext (hier die Zeile `reverse_string = vec!['\0'; size];`) zu erkenne ohne das er Konkret definiert wurde.
- Bei `vec!['\0'; size]` handelt es sich um ein Macro welches einen `Vec<char>` erzeugt bei dem die ersten `size` einträge den Wert `\0` haben.
- Am Ende wird `reverse_string` zu einen String transformiert. `into_iter()` erzeugt einen Konsumierenden Iterator(bedeutet `reverse_string`wird geleert) und `collect()` Konvertiert diesen zu einen neuen String, da String den Trait  `FromIterator<char>` implementiert hat. Alternativ könnte auch `String::from_iter(reverse_string.into_iter())` geschrieben werden.

### reverse_rust
- Da `Chars` den Trait `DoubleEndedIterator` implementiert kann der Iterator mit `rev()` umgedreht werden und anschließend mit `collect()` einen neuen String erzeugen, dabei wird der Original String nicht konsumiert.

### Part B
- `length` funktioniert analog zum erkennen der Länge aus `reverse` mithilfe eines `char Iterators`
- In `copy` wird das erste mal ein `IterMut` verwendet. Dabei handelt es sich um einen Speziellen Iterator welche erlaubt die Referenzierten Werte zu verändern. Da `next` ein `Option` zurückgibt muss `unwrap()` verwendet werden um auf den Wert zuzugreifen, sollte `input_pointer` oder `output_pointer` nicht mindestens `n` elemente beinhandeln würde dies zu einer `Panic` führen. `*` dereferenziert eine Referenz aus einen `&char` wird also ein `char`.  
- In `put_back` wird immer ein neuer `Vec<char>` mit der größe von `input` + 1 erzeugt und dieser nach dem befüllen auch zurückgegeben. Da Iteratoren nur Referenzen besitzen können und diese nicht lange genug Leben (da `output` nach funktionsaufruf zerstört wird) kann kein Iterator zurückgegeben werden. Rust würde dies statisch erkennen und nicht erlauben dies zu Kompilieren.
- Jeder aufruf von `reverse_rec` ergreift besitzt von einem `Vec<char>` in der Variable `reverse`, das bedeutet das am zum umdrehen eines Strings der länge n auch n Vektoren exestieren welche erst am ende der Rekursiven Funktion freigegeben werden (außer der erste welche den umgedrehten String enthält), diese funktion ist dadurch sehr kostenaufwändig.

### Testrahmen
- Beim Testrahmen sind die Felder des Structs `TestCase` vom Type `&str` und nicht `String` wie es in Aufgabe 1 der fall ist. Dafür wird dann allerdings eine Lifetime für die Referenzen benötigt. In `main()` werden mehrere Instanzen von `TestCase` erzeugt, als Argumente werden hier Stringkonstanten übergeben. In Rust werden Stringkonstanten beim Kompilieren Statisch angelegt. `"Hallo"` verweist also intern auf eine String Instanz mit Statischer lebensdauer und wenn ein `"Hallo"` an einer anderen Stelle verwendet wird verweißt es auf die selbe String Instanz.
- Beim ersten Eintrag von `test_functions` handelt es sich um ein Closure, während die beiden anderen Einträge einfach nur verweise an Funktion sind. Bei Closures werden zwischen zwei | die Parameter definiert, danach folgt der Funktionsinhalt. Handelt es sich dabei um eine Funktion mit Rückgabewert kann Rust dies implizit erkennen, der Rückgabetyp muss im gegensatzt zu normalen Funktionen also nicht angegeben werden, gleiches gilt auch für die Parameter hier ist die Typangabe ebenfalls optional.