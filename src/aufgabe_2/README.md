# Aufgabe 2: Umdrehen ("reverse") eines Strings
[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(4))

## Hinweise
Der komplette Code befindet sich in [src/main.rs](src/main.rs).

Ausführen mithilfe des Befehls `cargo run` ausgehend von diesem Ordner.

Es existieren drei unterschiedliche Lösungen für `reverse`.
- `reverse()` entspricht der Funktion `reverse(char*)` aus Teilaufgabe a) mit einem Algorithmus, welcher ähnlich in C umgesetzt werden könnte.
- `reverse_rust()` stellt ein Lösungsweg auf Rust Art dar.
- `reverse_rec()` entspricht der Funktion aus der Teilaufgabe b), ebenfalls C nahe geschrieben.

Im Vergleich zur Aufgabe 1 ist in Aufgabe 2 nicht nur das Lesen von Zeichenketten notwendig, sondern auch das Verändern und Erzeugen von Strings.


## IterMut und Rev
`IterMut` und `Rev` sind besondere Arten von Iteratoren, welche in Aufgabe 2 verwendet werden.  

`IterMut` gibt mit `next()` ein `Option<&mut T>` zurück, dadurch ist es möglich, das Element zu verändern, welches `next()` verweist. Da es sich um eine Referenz handelt, muss für eine Zuweisung, dieses jedoch zuerst mit dem `*`-Operator dereferenziert werden.

`Rev` ist ein umgekehrter Iterator. Wenn ein Iterator den Trait `DoubleEndedIterator` implementiert, ist es möglich, mit der `rev()` Funktion einen umgedrehten Iterator `Rev` zu erzeugen. Dabei wird vom letzten Element ausgehend auf das aktuelle Element verwiesen. Elemente, über die zuvor bereits mit `next()` iteriert wurde, bleiben dadurch weiterhin unzugänglich.

## Verändern von Strings
In Rust ist das Verändern eines `chars` an einer beliebigen Position im `String` nicht so einfach möglich wie in C(++). Grund dafür ist die UTF-8 Codierung, da `String`s intern als ein `Vec<u8>` verwaltet werden, aber ein Zechen aus mehreren Bytes bestehen kann. Deshalb wurde in Aufgabe 2 und 3 anstellen von `String` ein `Vec<char>` verwendet, um das Verändern der Zeichen auf einfach Art zu ermöglichen.

## Anmerkungen zum Code
### reverse
- In der ersten `while` Schleife wird die Länge der Eingabe ermittelt. Zwar haben Iteratoren in Rust auch eine Funktion `size_hint()`, welche die Anzahl an Elementen zurückgibt. Jedoch wurde hier bewusst die Länge mit einer Schleife ermittelt, analog zu einem C Algorithmus.
- Iteratoren können sich in Rust nur in eine Richtung bewegen, deshalb wird beim Erkennen der Länge eine Kopie von `text_pointer` verwendet und mit einer zweiten Iteration `reverse_string` befüllt. In C wäre es möglich, den Pointer wieder umzudrehen, ohne ihn zu Kopieren.
- `reverse_string` ist von Typ `Vec<char>`, Rust ist in der Lage, den Typ anhand des Kontexts (hier die Zeile `reverse_string = vec!['\0'; size];`) zu erkennen, ohne das er konkret definiert wurde.
- Bei `vec!['\0'; size]` handelt es sich um ein Macro welches einen `Vec<char>` erzeugt, bei dem die ersten `size` Einträge den Wert `\0` haben.
- Am Ende wird `reverse_string` zu einen String transformiert. `into_iter()` erzeugt einen konsumierenden Iterator(bedeutet `reverse_string`wird geleert) und `collect()` konvertiert diesen zu einen neuen `String`. Dies ist möglich, da `String` den Trait  `FromIterator<char>` implementiert hat. Alternativ könnte auch `String::from_iter(reverse_string.into_iter())` geschrieben werden.

### reverse_rust
- Da `Chars` den Trait `DoubleEndedIterator` implementiert, kann der Iterator mit `rev()` umgedreht werden und anschließend mit `collect()` einen neuen `String` erzeugen, dabei wird der original `String` nicht konsumiert.

### Part B
- `length` funktioniert analog zum Erkennen der Länge aus `reverse` mithilfe eines `char Iterators`.
- In `copy` wird zum ersten Mal ein `IterMut` verwendet. Da `next` ein `Option` zurückgibt, muss `unwrap()` verwendet werden um auf den Wert zuzugreifen. Sollte `input_pointer` oder `output_pointer` nicht mindestens `n` Elemente besitzen, würde dies zu einer `Panic` führen. `*` dereferenziert eine Referenz aus einen `&char` wird dadurch ein `char`.  
- In `put_back` wird immer ein neuer `Vec<char>` mit der Größe von (`input` + 1) erzeugt und dieser nach dem Befüllen auch zurückgegeben. Da Iteratoren nur Referenzen besitzen können und diese nicht lang genug Leben (da `output` nach Funktionsaufruf zerstört wird), kann kein Iterator zurückgegeben werden. Rust würde dies statisch erkennen und nicht erlauben, dies zu Kompilieren.
- Jeder Aufruf von `reverse_rec` ergreift besitzt von einem `Vec<char>` in der Variable `reverse`, das bedeutet, das zum Umdrehen eines Strings der länge `n` auch `n` Vektoren existieren, welche erst am Ende der rekursiven Funktion freigegeben werden (außer der erste, welcher den umgedrehten String enthält), diese Funktion ist dadurch sehr kostenaufwendig.

### Testrahmen
- Beim Testrahmen sind die Felder des Structs `TestCase` vom Typ `&str` und nicht `String`, wie es in Aufgabe 1 der Fall ist. Dafür wird dann allerdings eine Lifetime für die Referenzen benötigt. In `main()` werden mehrere Instanzen von `TestCase` erzeugt, als Argumente werden hier Stringkonstanten übergeben. In Rust werden Stringkonstanten beim Kompilieren statisch angelegt. `"Hallo"` verweist also intern auf eine Stringinstanz mit statischer Lebensdauer und wenn ein `"Hallo"` an einer anderen Stelle verwendet wird, verweist dies auf dieselbe Stringinstanz.
- Beim ersten Eintrag von `test_functions` handelt es sich um ein Closure Objekt, während die beiden anderen Einträge einfach nur verweise an Funktionen sind. Bei Closures werden zwischen zwei `|` die Parameter definiert, danach folgt der Funktionsinhalt. Handelt es sich dabei um eine Funktion mit Rückgabewert, kann Rust dies implizit erkennen, der Rückgabetyp muss im Gegensatz zu normalen Funktionen also nicht angegeben werden. Gleiches gilt auch für die Parameter, hier ist die Typangabe ebenfalls optional.