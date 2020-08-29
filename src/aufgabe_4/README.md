# Aufgabe 4: Parser/Interpreter/Compiler für arithemtische Ausdrücke
[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(6))

## Hinweise
Wie Aufgabe 3 besteht Aufgabe 4 ebenfalls aus einem Ausführbaren Teil und einer Bibliothek, dabei wurde der Code in mehrere Dateien unterteilt, wie es auch im C++ Original der Fall ist.  
Die Lösungen der Teilaufgabe Syntax befindet sich in ast.rs und die Lösungen für den Teil Semantik in vm_parser.rs.

Diese Aufgabe enthält ebenfalls Unit-Tests, einige dieser verfügen auch über eine anschauliche Ausgabe zum betrachten des Ergebnisses. Da `cargo test` standartmäßig ausgaben unterdrückt, muss zu betrachtung dieser der Befehl `cargo test -- --nocapture` verwendet werden. Eventuell sind die Ausgaben in einer fehlerhaften Ordnung da zum Testen mehrere Threads verwendet werden. Dies kann mit dem Befehl `cargo test -- --nocapture --test-threads=1` behoben werden.

Die Beispiel aus main.rs können einfach mithilfe des Befehls `cargo run` ausgeführt werden.



## Größere Bibliotheken
Wenn in Rust der komplette Code einer Bibliothek sich nicht nur in lib.rs befindet sondern über mehrere Dateien und Module aufgeteilt ist, müssen diese Module in lib.rs als `pub` importiert werden dammit sie extern verwendbar sind.

## Derive, Eq und Debug
Mit derive können in Rust Traits implementiert werden welche keine eigene Funktionsdefinitionen benötigen, da alle Funktionen des Traits bereits standardimplementationen besitzen. In Aufgabe 4 wurden derive mehrmals in Kombination mit dem Debug, Eq und PartialEq Trait verwendet.  
Debug erlaubt das verwenden eines Types als Debug Format (`{:?}`) in einem Format-String, dies ist vorallem bei Enums hilfreich da die Standardimplementation von Debug auf einen Enum Wert den Name des Wertes zurückgibt.  
Eq und PartialEq dienen zum vergleichen von Objekten des selben Types. Enums sind in Rust standardmäßig nicht ohne match operator vergleichbar, ist dies jedoch gewünscht müssen die Beiden Traits Eq und PartialEq für Enums verwendet werden.

## Box
`Box<T>` gehört zu den Pointer Typen von Rust. In der Regel werden Variablen/Felder auf den Stack abgelegt weshalb vor der Laufzeit bereits die Größe dieser bekannt sein muss (diese Regel existiert auch in C++). Wenn aber ein Objekt nur anhand eines Traits gespeichert werden soll ist es nicht möglich vor der Laufzeit die Größe des Objekts zu wissen, da eventuell mehrere Typen unterschiedlicher Größe infrage kommen könnten. In solchen fällen ist eine Box hilfreich da sie ein Objekt auf dem Heap Speichert und auf dieses verweist in etwa wie ein Unique Pointer aus C++ welcher auf eine Abstrakte Klasse angewendet wird. Dieses Verhalten ist in ast.rs bei MulExpr und PlusExpr zu erkennen da beide Felder besitzten, welche ein Objekt darstellen welches den Expr Trait implementiert ohne dabei den genauen Typ des Objekts zu kennen, es könnte sich dabei um MulExpr, PlusExpr oder IntExpr handeln.

## Any
`Any` ist ein Trait welcher standardmäßig von (fast) allen typen implementiert ist ohne dies Explizit anzugeben. `Any` dient dabei bei der erkennung des Typs eines Objekts und ermöglicht auch ein cast eines Objekts auf einen grundliegenden Typ, wie der `dynamic_cast` aus C++.

## Deref
Durch das Implementieren des Deref Traits wird der * Operator auf einen Typ Überschrieben, dabei kann ein Objekt Zurückgegeben werden welches einem anderen Typen entspricht. Weiterhin können alle Funktionen welche für den Dereferenzierten Typ implementiert sind auch direkt vom einem Objekt des ursprungstyps ausgeführt werden. Wie in C++ ist das überschreiben des * Operators dadurch vorallem für Pointer sinnvoll, ein weitere Beispielanwendung ist der String Typs welcher ein Deref auf str implementiert. Da Rust das vererben von Struct Typen untereinader nicht unterstützt können mithilfe von Deref zumindestens Öffentliche Funktionen und Felder einfach zugänglich gemacht werden, vorausgesetzt das eigentliche ableitende Struct besitz ein Objekt der Superklasse wie es beim Tokenizer der fall ist (obwohl Deref eigentlich nie für Tokenizer benötigt wird, sollte dadurch die Vererbung zwischen Tokenize und Tokenizer aus dem Originalcode dargestellt werden).


## Anmerkungen
- ### `ast.rs (Teilaufgabe Syntax)`
    - In Rust ist es wie in C++ möglich ein Alias für einen Typ festzulegen, dafür wird das Schlüsselwort `type` benötigt.
    - Aus Expr wurde ein Trait um als Interface zu dienen, da Expr im C++ Original eine Abstrakte Klasse ist diese einfach mit einem Trait in Rust darzustellen
    - Zum einfachen unterscheiden der Unterschiedlichen Expression wurde hier ein Enum als Hilfe verwendet, es wäre aber auch mithilfe von `Any` möglich, wenn auch etwas aufwendiger.
    - In Rust ist es üblich die Unit-Tests in dieselbe Datei zu schreiben in der sich der zu testende Code befindet. Deshalb befindet sich in jeder Datei aus Aufgabe 4 (ausnahme lib.rs und main.rs) die Unit-Tests für das entsprechende Modul.

- ### `tokenizer.rs`
    - Der `TokenT` enum wurde erweitert indem er alle möglichen Zahlenwerte erlaubt, dies wird ermöglicht da in Rust die Einträge von enums auch Parameter beinhalten können.
    - Für Tokenize wurde der Iterator Trait implementiert aufgrund der Funktionalen ähnlichkeit von Tokenize zu einem Iterator und da dadurch auch das Verwenden von von Iterator Funktionen wie collect möglich ist ohne diese zu definieren.

- ### `parser.rs`
    - Da in Rust bereits ein passender Typ zum Beschreiben des Vorhandenseins eines Wertes beschreibt existiert, wird hier `Option<ExprBox>` verwendet anstelle des vorgegebenen Option Typ aus der Aufgabenstellung.
    - Da viele der Internen Funktionen von Parser ein `&mut self` als Parameter besitzten, muss jede Instanz von Parser mit `mut` deklariert werden, damit ein String zu einer VM umgewandelt werden kann. 
    
- ### `vm.rs`
    - Aufgrund der freiheit von Rust Enums mit parametern zu definieren, wird kein value Feld in Code benötigt und stattdessen der Wert in PUSH vermerkt
    - Da das Parsen eines Strings zu einer Expression nicht immer möglich ist wurde dafür der `TryFrom` Trait implementiert welcher das Abfangen von Fehlern beim Parsen erlaubt.
    - Da es jedoch immer möglich ist eine Expression zu einer VM zu parsen wurde dafür der `From` Trait implementiert.
    - Der ? Operator ist beim behandeln von `Result` Fehlern hilfreich, wenn eine Anweisung welche ein `Result` zurückgibt mit einem ? endet, bedeutet das falls die Aussage ein `Err` zurückliefert diese direkt von der aktuellen Funktion zurückgegeben wird. Dafür muss die Funktion natürlich ein Result als Ruckgabetyp besitzten, wie es bei `fmt()` der Fall ist.
    - Da das eigentliche Parsen von Strings und Expressions zu VMs in anderen Dateien definiert sind und die beiden `From` implementationen nur als Shortcut dienen existieren sich für diese hier keine Unit-Tests

- ### `vm_parser.rs (Teilaufgabe Semantik)`
    - Der `VMParser` orientiert sich am `Parser`. Zum unterscheiden der Typen von `Expr` wird Any benötigt, da zugriff für die Felder von MulExpr und PlusExpr und IntExpr benötigt wird. Für diesen Zugriff werden die Expr Objekte zu ihrem Primärtyp umgewandelt, mithilfe der `downcast_ref` Funktion von `Any` und matching Operatoren.
    - Während `Parser` besitzt des Eingabe Strings ergreift und diesen dadurch Konsumiert, Verzichtet VMParser auf diese besitzergreifung um die ExprBox nicht zu zerstören.
    - Ähnlich wie bei `Parser` wird zum parsen einer ExprBox zu einer VM vorausgesetzt das die VMParser Instanz mit `mut` deklariert wurde. 

- ### `main.rs`
    - In main.rs befinden sich anschauliche Beispiel für Expr, Tokenize und dem VMParser. Für einige dieser Beispiel existieren ebenfalls Unit-Tests in ihren entsprechendenden Dateien.
