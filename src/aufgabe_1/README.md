# Aufgabe 1: Erkennen von Mustern
[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(3))

Der komplette Code befindet sich in [src/main.rs](src/main.rs).

Ausführen mithilfe des Befehls `cargo run` ausgehend von diesem Ordner.

Es wurden 2 unterschiedliche Funktionen für `extract` implementiert.
- `extract_c` soll eine C nahe funktion darstellen, welche ohne besondere standartfunktion von Rust funktioniert.
- `extract_rust` stellt dar wie die Aufgabenstellung eigentlich in Rust mithilfe von Standardfunktionen gelöst werden sollte.

## Anmerkungen
- In C sind `char*` verweise auf einen `char`, welcher meist Teil eines Arrays ist. Die Position des Verweises kann mithilfe von Operatoren wie +, - und [] verändert werden, zudem ist es möglich den Wert des verweisenden `char` zu änderen. Die letzte Position eines `char` Arrays ist mithilfe des Wertes `\0` erkennbar. 
- Rust stellt in den Meisten Fällen Strings im UTF-8 Format dar, in C stellt ein `char`ein Byte und dadurch ein einziges ASCII Symbol dar. In Rust ist es deshalb möglich das ein `char` Objekt aus mehreren Bytes besteht.
- &str Objekte besitzten die Funktion `chars()`. Diese Funktion gibt ein iterator über die `char` Objekte des Strings zurück, zudem gibt es auch noch die Funktion `bytes()` welche einen Iterator über die einzelnen Bytes des Strings zurückgibt, bei ASCII Strings geben beide Funktionen identsiche Werte zurück. Zur Kompitabilität mit UTF-8 Strings wurde in den Aufgaben `chars()` verwendet.
- Iteratoren verfügen in Rust mindestens immer über eine Funktion `next()` diese gibt ein Objekt des Enums `Option<T>` zurück und bewegt den Zeiger um eine stelle vorwärts. `T` ist der Datentyp der Collection zu der der Iterator gehört. Im Falle von `.chars()` handelt es sich dabei um einen `char`. Zeigt der Iterator auf ein gültiges Objekt wird Some(t) zurückgegeben,  ansonsten wird `None` zurückgegeben.
- Iteratoren können mit `clone()` kopiert werden. Dies entspricht dem Kopieren eines Pointers in C, das Referenzierte Objekt wird dabei nicht kopiert.
### extract_c 
- In `extract_c` wird mithilfe von `while let Some(current_text_char) = text_position.next()` über die einzelnen `char` Objekte von `text` iteriert. Dies ist vergleichbar mit dem Ausdruck `while (c != '\0') {c++; ...}` in C. Da in Rust Strings kein `\0` zum Terminieren verwenden wird deshalb das Ende des Iteratoren mithilfe von `None` erkannt. Der Ausdruck `while let ...` ist eine vereinfachung des folgenden ausdrucks:
```
    let mut c = text_position.next();
    while c != None { 
        ...
        c = text_position.next();
    }
```
- Mit `suffix_position` wird die Position nach dem zuletzt gefunden passenden Pattern als Iterator gespeichert. Am ende von `extract_c` wird `suffix_position` als `&str` zurückgegeben, dabei handelt es sich aber nur um eine Referenz von einen bereich aus `text` und nicht um einen neuen String.  
Werden  Referenzen in Rust von Funktionen zurückgegeben, muss die Lifetime des Rückgabe angegeben werden damit Garantiert werden kann das die Referenz auch nach dem Funktionaufruf noch auf ein existierendes Objekt verweist.
Es ist also nicht möglich ein Objekt in einer Funktion zu konstruieren und dann nur eine Referenz desen zurückzugeben, da das Objekt am ende des Funktionsaufruf zertört wird. Darum ist es nur möglich Referenzen zurückzugeben welche entweder zu einem Parameter gehören oder Statisch sind. Mit `<'a>` wird angegeben das es sich bei der Lifetime des Rückgabewertes von `extract_c` um die von `text` handelt. Beide Verweisen also das selbe Objekt (ein String), allerdings nicht zwingend auf den selben Bereich sondern eventuell nur auf einen Teil.
### Testkonstrukt

