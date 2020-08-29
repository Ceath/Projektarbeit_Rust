# Aufgabe 1: Erkennen von Mustern

[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(3))

## Hinweise
Der komplette Code befindet sich in [src/main.rs](src/main.rs).

Die Aufgabe kann mithilfe des Befehls `cargo run` ausgehend von diesem Ordner ausgeführt werden.

Es wurden 2 unterschiedliche Funktionen für `extract` implementiert.
- `extract_c` soll eine C nahe funktion darstellen, welche ohne besondere standartfunktion von Rust funktioniert.
- `extract_rust` stellt dar wie die Aufgabenstellung eigentlich in Rust mithilfe von Standardfunktionen gelöst werden sollte.

## String, str und char
Rust verwendet zur darstellung von Zeichenketten UTF-8 als Kodierung, deshalb kann ein einzelnes Zeichen länger als ein Byte groß sein, dies ist ein wichtiger unterschied zwischen dem Rust char und C char. Zudem werden Strings in Rust nicht mit einem `\0` terminiert sondern haben eine gesetzte länge.  
String ist ein Struct zum darstellen von (UTF-8)Strings in Rust, Rust besitzt aber auch andere String Typen z.B. für nur ASCII Zeichenketten.  
Bei str handelt es sich um einen Slice eines String. Sie werden verwendet um einen Teil eines Strings zu referenzieren weshalb sie in der Regel einen & Operator benötigen. Das anwenden des * Operators auf einen String liefert ein str zurück, welche den ganzen String Referenziert. Dadurch ist es möglich alle Funktionen welche für str implementiert wurde auch mit einem String auszuführen. Aus diesem Grund werden in Rust wenn immer eine Referenz einer Zeichenkette benötigt wird meist &str verwendet da eine str Referenz sowohl anhand eines str Objekts oder auch indirekt über ein String Objekt mithilfe des * Operators oder des [...] (Slice) Operators möglich ist.

## Iteratoren
Iteratoren werden in Rust häufig verwendet um über Kollektionen zu Iterieren, dabei werden die Werte Referenziert und nicht Konsumiert. Iteratoren implementieren mindestens den Trait `Iterator` und besitzten immer die Funktion `next()`, diese gibt ein Objekt des Enums `Option<T>` zurück und bewegt den Zeiger um eine stelle vorwärts. Wenn der Iterator am ende ist gibt `next()` nur noch None zurück. Das bewegen zur vorherigen Position wird dabei von den meisten Iteratoren nicht unterstüzt, deshalb müssen mit `clone()` Kopien an vorheriegen Positionen erstellt werden.  
In Aufgabe 1 wird die Funktion `chars()` von `str` verwendet um einen char Iterator über eine Zeichenkette zu erschaffen, der grund dafür ist die ähnlichkeit gegenüber dem char* aus C, da dadurch das bewegen eines char* über einen String vergleichbar dargestellt werden kann.

## Vector
Vector ist eine in oft verwendet Kollektion welche der gleichnahmigen Klasse aus C++ ähnelt. Werte können mithilfe von `push(T)` eingefügt werden, es ist aber auch möglich einen Vector mit dem Macro `vec!` zu initialisieren.

## Closures
Bei Cosures handelt es sich um Typen welche Funktionen darstelle, wie Lambdas aus C++. Entweder können vorhandene Funktionen referenziert werden oder neue Colusure Ausdrücke erstellt werden, dabei muss immer auf die Lebenszeit geachtet werden wenn Referenzen verwendet werden. Closure Definitionen können mithilfe von Templates beschrieben werden.


## Anmerkungen zum Code
- &str Objekte besitzten die Funktion `chars()`. Diese Funktion gibt ein Iterator über die `char` Objekte des Strings zurück, zudem gibt es auch noch die Funktion `bytes()` welche einen Iterator über die einzelnen Bytes des Strings zurückgibt, bei ASCII Strings geben beide Funktionen vergleichbare Vektoren zurück. Zur Kompatibilität mit UTF-8 Strings wurde in den Aufgaben `chars()` verwendet.
### extract_c 
- In `extract_c` wird mithilfe von `while let Some(current_text_char) = text_position.next()` über die einzelnen `char` Objekte von `text` iteriert. Dies ist vergleichbar mit dem Ausdruck `while (c != '\0') {c++; ...}` in C. Da in Rust Strings kein `\0` zum Terminieren verwenden wird das Ende des Iteratoren mithilfe von `None` implizit erkannt. Der Ausdruck `while let ...` ist eine vereinfachung des folgenden ausdrucks:
```
    let mut c = text_position.next();
    while c != None { 
        ...
        c = text_position.next();
    }
```
- Mit `suffix_position` wird die Position nach dem zuletzt gefunden passenden Pattern als Iterator gespeichert. Am ende von `extract_c` wird `suffix_position` als `&str` zurückgegeben, dabei handelt es sich aber nur um eine Referenz von einen bereich aus `text` und nicht um einen neuen String, 
deshalb wird eine Lifetime für den Rückgabewert benötigt.
Mit `<'a>` wird angegeben das es sich bei der Lifetime des Rückgabewertes von `extract_c` um die von `text` handelt. Beide Verweisen also das selbe Objekt (ein String), allerdings nicht zwingend auf den selben Bereich sondern eventuell nur auf einen Teil (slice/substring).
### Testrahmen
- Der Testrahmen wurde aus dem Original größtenteils übernommen, da jedoch zwei funktionen getestet werden müssen wurde er angepasst indem er ein Closure der zu testende Funktion als Parameter bekommt.

