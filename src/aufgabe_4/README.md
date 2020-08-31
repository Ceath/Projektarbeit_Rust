# Aufgabe 4: Parser/Interpreter/Compiler für arithemtische Ausdrücke
[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(6))

## Hinweise
Wie Aufgabe 3 besteht Aufgabe 4 ebenfalls aus einem ausführbaren Teil und einer Bibliothek, dabei wurde der Code in mehrere Dateien unterteilt, wie es auch im C++ Original der Fall ist.  
Die Lösungen der Teilaufgabe "Syntax" befindet sich in `ast.rs` und die Lösungen für den Teil "Semantik" in `vm_parser.rs`.

Zudem enthält diese Aufgabe auch Unit-Tests, einige dieser verfügen über eine anschauliche Ausgabe zum Betrachten der Ergebnisse. Da `cargo test` standardmäßig ausgaben unterdrückt, muss zur Betrachtung dieser der Befehl `cargo test -- --nocapture` verwendet werden. Eventuell sind die Ausgaben in einer fehlerhaften Reihenfolge, da zum Testen mehrere Threads verwendet werden. Dies kann mit dem Befehl `cargo test -- --nocapture --test-threads=1` behoben werden.

Die Beispiele aus `main.rs` können einfach mithilfe des Befehls `cargo run` ausgeführt werden.



## Größere Bibliotheken
Wenn in Rust der komplette Code einer Bibliothek sich nicht nur in `lib.rs` befindet, sondern über mehrere Dateien und Module aufgeteilt ist, müssen diese Module in `lib.rs` als `pub` importiert werden, damit sie extern verwendbar sind.

## Derive, Eq und Debug
Mit `#[derive(TRAIT)]` können Traits implementiert werden, welche keine eigenen Funktionsdefinitionen benötigen, da alle Funktionen des Traits bereits Standardimplementationen besitzen. In Aufgabe 4 wurde `derive` mehrmals in Kombination mit dem `Debug`, `Eq` und `PartialEq` Trait verwendet.

Debug erlaubt das Verwenden eines Typs via dem Debugformat (`"{:?}"`) in einem Format-String, dies ist bei Enums hilfreich, da das Verwenden der Standardimplementation von `Debug` auf einen Enum Objekt den Namen des Wertes zurückgibt.

`Eq` und `PartialEq` dienen zum booleschen Vergleichen von Objekten desselben Typs. Enums sind in Rust standardmäßig nur per match Operator vergleichbar, ist dies jedoch gewünscht, müssen die beiden Traits `Eq` und `PartialEq` für Enums implementiert werden.

## Box
`Box<T>` gehört zu den Pointertypen von Rust. In der Regel werden Variablen/Felder auf den Stack abgelegt, weshalb vor der Laufzeit bereits die Größe dieser bekannt sein muss (dies gilt auch in C++). 

Wenn aber ein Objekt nur anhand eines Traits vermerkt werden soll, ist es nicht möglich, vor der Laufzeit die Größe des Objekts zu wissen, da eventuell mehrere Typen unterschiedlicher Größe infrage kommen könnten. In solchen Fällen ist eine `Box` hilfreich, da sie ein Objekt auf dem Heap ablegt und auf dieses verweist, etwa wie ein `unique_ptr` aus C++, welcher auf eine abstrakte Klasse angewendet wird. Dieses Verhalten ist in `ast.rs` bei `MulExpr` und `PlusExpr` vorhanden, da beide Felder besitzen, welche ein Objekt darstellen, welches den Expr Trait implementiert, ohne dabei den genauen Typ des Objekts zu kennen, es könnte sich dabei um `MulExpr`, `PlusExpr` oder `IntExpr` handeln.

## Any
`Any` ist ein Trait, welcher standardmäßig von (fast) allen Typen implementiert ist, ohne dies explizit anzugeben. `Any` dient bei der Erkennung des Typs eines Objekts und ermöglicht auch ein cast eines Objekts auf einen grundlegenden Typ. Ähnlich wie der `dynamic_cast` aus C++.

## Deref
Durch das Implementieren des `Deref` Traits wird der `*`-Operator überschrieben, dabei kann auch mit `*` ein Objekt zurückgegeben werden, welches einem anderen Typen entspricht. 

Weiterhin können alle Funktionen, welche für den dereferenzierten Typ implementiert sind, auch direkt vom einem Objekt des Ursprungstyps ausgeführt werden. Wie in C++ ist das Überschreiben des `*` Operators dadurch vor allem für Pointer wie `Box<T>` sinnvoll, ein weiterer Beispielfall ist der `String` Typ, welcher ein `Deref` auf `str` implementiert. 

Da Rust das Vererben von Struct Typen Untereinader nicht unterstützt, können mithilfe von `Deref` zumindest öffentliche Funktionen und Felder einfach zugänglich gemacht werden, vorausgesetzt das eigentliche ableitende Struct besitz ein Objekt der Superklasse, wie es beim `Tokenizer` der Fall ist (Deref wird eigentlich nie für `Tokenizer` benötigt wird, jedoch sollte dadurch die Vererbung zwischen `Tokenize` und `Tokenizer` aus dem Originalcode dargestellt werden).


## Anmerkungen zum Code
### [ast.rs](src/ast.rs) (Teilaufgabe Syntax)
- In Rust ist es wie in C++ möglich, ein Alias für einen Typ festzulegen, dafür wird das Schlüsselwort `type` benötigt.
- Aus `Expr` wurde ein Trait, welches als Interface dient, da `Expr` im C++ Original eine abstrakte Klasse ist, ist einfach diese mit einem Trait in Rust darzustellen.
- Zum einfachen Unterscheiden den unterschiedlichen Expressions wurde hier ein Enum als Hilfe verwendet, es wäre aber auch mithilfe von `Any` möglich, wenn auch etwas aufwendiger.
- In Rust ist es üblich, die Unit-Tests in dieselbe Datei zu schreiben, in der sich der zu testende Code befindet. Deshalb befindet sich in jeder Datei aus Aufgabe 4 (Ausnahme `lib.rs` und `main.rs`) die Unit-Tests für das deren Modul.

### [tokenizer.rs](src/tokenizer.rs)
- Der `TokenT` Enum wurde erweitert, indem er alle möglichen Zahlenwerte erlaubt. Dies wird ermöglicht, da in Rust die Werte eines Enums auch Parameter besitzen können.
- Für `Tokenize` wurde der `Iterator` Trait implementiert aufgrund der funktionalen Ähnlichkeit von `Tokenize` zu einem Iterator und da dadurch auch das Verwenden von Iterator Funktionen wie `collect()` möglich ist, ohne diese zu definieren.

### [parser.rs](src/parser.rs)
- Da in Rust bereits ein passender Typ zum Beschreiben des Vorhandenseins eines Wertes existiert, wird hier `Option<ExprBox>` verwendet anstelle des vorgegebenen `Optional` Typ aus der Aufgabenstellung.
- Da viele der internen Funktionen von Parser ein `&mut self` als Parameter besitzen, muss jede Instanz von Parser mit `mut` deklariert werden, damit ein String zu einer VM umgewandelt werden kann. 
    
### [vm.rs](src/vm.rs)
- Aufgrund der Freiheit von Rust, Enums mit Parametern zu definieren, wird kein `value` Feld in Code benötigt, sondern stattdessen der Wert in `PUSH` vermerkt.
- Da das Parsen eines Strings zu einer `Expr` nicht immer möglich ist, wurde dafür der `TryFrom` Trait implementiert, welcher das Abfangen von Fehlern beim Parsen erlaubt.
- Da es jedoch immer möglich ist, eine `Expr` zu einer `VM` zu parsen, wurde dafür der `From` Trait implementiert.
- Der `?` Operator ist beim Behandeln von `Result` Fehlern hilfreich. Wenn eine Anweisung, welche ein `Result` zurückgibt, mit einem `?` endet, wird dadurch ein `Err` dieser Anweisung abgefangen und an die aktuelle Funktion als Rückgabewert weitergeleitet. Dafür muss die Funktion natürlich ein `Result` als Rückgabetyp besitzen, wie es bei `fmt()` der Fall ist.
- Da das eigentliche Parsen von `String`s und `Expr` zu `VM`s in anderen Dateien definiert ist und die beiden `From` Implementationen nur als Shortcut dienen, existieren für diese hier keine Unit-Tests.

### [vm_parser.rs](src/vm_parser.rs) (Teilaufgabe Semantik)
- Der `VMParser` orientiert sich am `Parser`. Zum Unterscheiden der Typen von `Expr` wird `Any` benötigt, da zugriff für die Felder von `MulExpr`, `PlusExpr` und IntExpr benötigt wird. Für diesen Zugriff werden die `Expr` Objekte zu ihrem Ursprungstyp mithilfe der `downcast_ref` Funktion von `Any` und matching Operatoren umgewandelt.
- Während `Parser` besitz des Eingabestrings ergreift und diesen dadurch konsumiert, verzichtet `VMParser` auf diese Besitzergreifung, um die `ExprBox` nicht zu zerstören.
- Ähnlich wie bei `Parser` wird zum Parsen einer `ExprBox` zu einer VM vorausgesetzt, das die `VMParser` Instanz mit `mut` deklariert wurde. 

### [main.rs](src/main.rs)
- In `main.rs` befinden sich anschauliche Beispiel für `Expr`, `Tokenize` und dem `VMParser`. Für einige dieser Beispiele existieren ebenfalls Unit-Tests in ihren entsprechendenden Dateien.
