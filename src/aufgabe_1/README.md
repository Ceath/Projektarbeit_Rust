# Aufgabe 1: Erkennen von Mustern

[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(3))

## Hinweise
Der komplette Code befindet sich in [src/main.rs](src/main.rs).

Die Aufgabe kann mithilfe des Befehls `cargo run` ausgehend von diesem Ordner ausgeführt werden.

Es wurden 2 unterschiedliche Funktionen für `extract` implementiert.
- `extract_c` stellt eine C nahe Funktion dar, welche ohne besondere Standartfunktion von Rust funktioniert.
- `extract_rust` stellt dar, wie die Aufgabenstellung einfach und kompakt in Rust mithilfe von Standardfunktionen gelöst werden kann.

## String, str und char
Rust verwendet zur Darstellung von Zeichenketten UTF-8 als Codierung, deshalb kann ein einzelnes Zeichen länger als ein Byte groß sein. Dies ist ein wichtiger Unterschied zwischen dem Rust `char` und C `char`. Zudem werden Strings in Rust nicht mit einem `\0` terminiert, sondern besitzen eine längenwert. 

`String` ist ein Struct zum Darstellen von (UTF-8)Zeichenketten in Rust, Rust besitzt aber auch andere String Typen z. B. für Zeichenketten im ASCII Format oder auch C Zeichenketten. 

Bei `str` handelt es sich um einen Slice eines `String`s. Es wird verwendet, um einen Teil eines `String`s zu referenzieren, weshalb es in der Regel einen `&` Operator benötigt. Das Anwenden des `*` Operators auf einen `String` liefert ein `str` zurück, welches den ganzen `String` referenziert. Dadurch ist es möglich, alle Funktionen, welche für `str` implementiert wurde, auch mit einem `String` auszuführen. Aus diesem Grund werden in Rust, wenn eine Referenz einer Zeichenkette benötigt wird, meist `&str` verwendet, da eine `str` Referenz sowohl anhand eines `str` Objekts oder auch indirekt über ein `String` Objekt mithilfe des `*` Operators oder des [...] (Slice) Operators möglich ist.

## Iteratoren
Iteratoren werden in Rust häufig verwendet, um über Kollektionen zu iterieren, dabei werden die Werte referenziert und keine Ownership übergeben. Iteratoren implementieren mindestens den Trait `Iterator` und besitzen dadurch immer die Funktion `next()`, diese gibt ein Objekt des Enums `Option<T>` zurück und bewegt den Zeiger um eine stelle vorwärts. Wenn der Iterator am Ende ist, gibt `next()` nur noch None zurück. Das bewegen zur vorherigen Position, wird dabei von den meisten Iteratoren nicht unterstützt, deshalb müssen mit `clone()` Kopien einer vorherigen Position erstellt werden.  

In Aufgabe 1 wird die Funktion `chars()` von `str` verwendet, um einen `char` Iterator über eine Zeichenkette zu erschaffen, der Grund hierfür ist die Ähnlichkeit gegenüber dem `char*` aus C, da dadurch das Bewegen eines `char*` über einen `String` vergleichbar dargestellt werden kann.

## Vec
`Vec<T>` ist eine in oft verwendete Kollektion, welche der `Vector<T>` Klasse aus C++ entspricht. Werte können mithilfe von `push(T)` eingefügt werden, es ist aber auch möglich, einen Vektor mit dem Macro `vec!` zu initialisieren.

## Closures
Bei Cosures handelt es sich um Typen, welche Funktionen darstellen, wie Lambdas aus C++. Entweder können vorhandene Funktionen referenziert werden oder neue Colusure Ausdrücke erstellt werden, dabei muss immer auf die Lebenszeit geachtet werden, wenn Referenzen verwendet werden. Closure Definitionen können mithilfe von Templates beschrieben werden.


## Anmerkungen zum Code
- `&str` Objekte besitzen die Funktion `chars()`. Diese Funktion gibt ein Iterator über die `char` Objekte des Strings zurück, zudem gibt es auch noch die Funktion `bytes()`, welche einen Iterator über die einzelnen Bytes des Strings zurückgibt. Bei ASCII Strings geben beide Funktionen vergleichbare Vektoren zurück. Zur Kompatibilität mit UTF-8 Strings wurde in den Aufgaben `chars()` verwendet.
### extract_c 
- In `extract_c` wird mithilfe von `while let Some(current_text_char) = text_position.next()` über die einzelnen `char` Objekte von `text` iteriert. Dies ist vergleichbar mit dem Ausdruck `while (c != '\0') {c++; ...}` in C (`c` ist vom typ `char*`). Da in Rust Strings kein `\0` zum Terminieren verwenden, wird das Ende des Iterators mithilfe von `None` implizit erkannt. Der Ausdruck `while let ...` ist eine Vereinfachung des folgenden Ausdrucks:
```
    let mut c = text_position.next();
    while c != None { 
        ...
        c = text_position.next();
    }
```
- Mit `suffix_position` wird die Position nach dem zuletzt gefunden passenden Pattern als Iterator gespeichert. Am Ende von `extract_c` wird `suffix_position` als `&str` zurückgegeben, dabei handelt es sich aber nur um eine Referenz von einen Bereich aus `text` und nicht um einen neuen String, 
deshalb wird eine Lifetime für den Rückgabewert benötigt.
Mit `<'a>` wird angegeben, dass es sich bei der Lifetime des Rückgabewertes von `extract_c` um die von `text` handelt. Beide verweisen also dasselbe Objekt (ein String), allerdings nicht zwingend auf denselben Bereich, sondern eventuell nur auf einen Teil (Slice/Substring).

### Testrahmen
Der Testrahmen wurde aus dem Original größtenteils übernommen, da jedoch zwei Funktionen getestet werden müssen, wurde er angepasst, indem er ein Closure der zu testenden Funktion als Parameter bekommt.

