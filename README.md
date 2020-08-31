# Projektarbeit "Implementieren von Softwareprojekt Aufgaben in Rust"

### Autor: Daniel Daunderer

## Allgemein
Die Projektarbeit ist Teil des Bachelorstudiengangs Informatik an der Hochschule Karlsruhe und entstand im Laufe des Sommersemesters 2020. Betreut wurde die Arbeit von Professor Dr. Martin Sulzmann.  
Ziel ist die Implementierung der Aufgaben des Softwareprojektlabors in Rust, um dadurch Unterschiede zwischen Rust und C(++) zu veranschaulichen.  

Die Projektarbeit ist durch vier Aufgaben unterteilt, welche sich in einem eigenen Unterordner in `src` befinden. Neben den Lösungen der Aufgabenstellungen beinhaltet die Projektarbeit auch eine Dokumentation, welche Rust und die Lösungen näher erklärt. Neben einer Hauptdokumentation, welche als Einstieg in Rust und die Projektarbeit gedacht ist, ist jede Aufgabe mit einer eigenen Dokumentation ausgestattet, welche als Leitfaden für die Lösungen dient, zudem ist der Quellcode an mehren Stellen kommentiert. Ziel ist das dadurch selbst Personen, welche sich zuvor noch nicht mit Rust oder den Softwareprojektaufgaben befasst haben, ein Verständnis dieser übermittelt wird.

In jeder Aufgabe befindet sich ein Link zur Aufgabenstellung, da dieser jedoch auf den aktuellen Commit eines externen Repositories verweist, ist es möglich, das die Aufgabenstellung dort inzwischen verändert wurden.  
Eine archivierte Version aller Aufgabenstellungen ist über diesen [Link](https://htmlpreview.github.io/?https://github.com/sulzmann/SoftwareProjekt/blob/5dc75c9d4f84bfe9e8396fab6785276f25fd5cbe/schein.html) erreichbar.

Zum Erstellen der Projektarbeit wurde "IntelliJ IDEA" und "Visual Studio Code" verwendet, der Code wurde mit der Rust Version 1.45.0 entwickelt. Da Rust sich noch in Entwicklung befindet, ist es möglich, das Teile des Quellcodes nicht mit neueren Versionen kompatible sind oder die Dokumentation veraltet ist.  
Die Informationen über Rust stammen größtenteils aus dem online frei verfügbaren Buch "The Rust Programming Language" (https://doc.rust-lang.org/book/) sowie der offiziellen Rust Dokumentation (https://doc.rust-lang.org/std/).

## Rust
Rust ist eine von Mozilla seit 2010 sich in Entwicklung befindende Plattformübergreifende Programmiersprache, welche sich dabei auf das Schreiben von sicheren, schnellen und parallel laufbaren Code fokussiert. Dabei kombiniert Rust Aspekte von mehreren unterschiedlichen Programmiersprachen, besitzt jedoch viel Besonderheiten, wodurch sich Rust doch stark von den meisten gängigen Programmiersprachen unterscheidet. 

Eine dieser Besonderheit ist das Verwalten von Speicher-Ressourcen anhand eines Ownership Systems oder auch die Abwesenheit von Null-Pointer. Weiterhin zeichnet sich Rust durch einen schlauen Analyzer aus, welcher prüft, ob die meisten Rust-Idiome eingehalten werden und dadurch einige typische Laufzeitfehler reduziert. Jedoch ist es explizit auch möglich, mit Rust unsicheren Code zu schreiben, dadurch ist es z. B. möglich, C Bibliotheken zu verwenden. Für die Projektarbeit wurde jedoch nur sicherer Rust Code verwendet.

## Variablendeklaration
Variablen werden mit dem Syntax `let NAME` deklariert. Auch wenn in Rust einer Variable nur einen Typ zugewiesen ist, muss im Gegensatz zu C der Typ nicht immer explizit angegeben werden, solange der Typ nachfolgend eindeutig schlussfolgbar ist. Ebenso müssen Variablen nicht direkt initialisiert werden. Es ist aber auch möglich, den Typ bei der Deklaration anzugeben, dies dient nicht nur der Lesbarkeit, sondern ist auch in einigen Fällen notwendig, wenn der Typ mehrdeutig interpretierbar ist.  

In Rust sind alle Variablen, solange nicht speziell angegeben unveränderbar (const), wenn eine Variable veränderbar (mutable) sein soll, wird das Schlüsselwort `mut` bei der Deklaration benötigt. Damit geht Rust den gegenseitigen Weg von C(++), wo jede Variable, welche nicht mit `const` deklariert wurde veränderbar ist. Rust prüft das Einhalten dieser Regeln statisch, bevor dem Kompilieren mit dem Analyzer.  
Beispiele:  
```
let word: String; // Variable mit Typangabe

let word; // Typerkennung anhand des Kontext
word = String::new();

let mut word = String::new(); // Veränderbare Variable
word.push_str("Hello, World");
```

## Speicherverwaltung und Ownership
Zur Speicherverwaltung verlässt sich Rust weder auf das manuelle Freigeben von Ressourcen wie in C(++) oder auf einen Garbage-Collector wie Java, C#, Python etc. Stattdessen werden Ressourcen mithilfe von Ownerships verwaltet. 

Jedes Objekt in Rust gehört einer Variable an, welche das Objekt besitzt und sobald die Variable am Ende ihres Blocks (Scope, meist anhand von `{}` Klammern erkennbar) erreicht, werden alle Ressourcen des Objekts freigegeben. Bei Typen, welche aus mehren Objekten bestehen, wie z. B. Structs werden auch diese Objekte freigegeben, vorausgesetzt das Struct ist der Besitzer der Objekte.

Dadurch müssen Ressourcen nicht manuelle freigegeben werden und es gibt keine periodischen Performanceeinbrüche aufgrund eines Garbage-Collectors. Das Ownership Prinzip von Rust lässt sich leicht in C(++) vorstellen, wenn ein Programm geschrieben wird, bei dem jede Variable auf dem Stack abgelegt ist und der Heap sowie Pointer (fast) nicht verwendet werden.

Um das Kopieren von anspruchsvolleren Objekten(Struct, Enum) zu vermeiden, ist der `=`-Operator in Rust ein Move- und kein Kopieroperator (es gibt jedoch Ausnahmen wie die Primärdatentypen (i32, f32, char etc.)). Der Ausdruck `var2 = var1;` bedeutet also, dass die Variable `var2` besitzt des Objekts von `var1` ergreift, dies führt auch dazu, das `var1` jetzt kein Objekt mehr besitzt, wodurch `var1` nicht mehr verwendet werden kann (bis die Variable wieder besitzt eines Objekt ergreift).  
Dies gilt z. B. auch beim Übergeben des Objekts als Parameter einer Funktion oder beim einfügen in eine Kollektion(Vector, HashSet etc.). Wenn jedoch das Objekt übergeben werden soll, ohne dabei den Besitzer zu wechseln, werden Referenzen verwendet und es wird vom Leihen (borrow) des Objekts gesprochen. 

Referenzen verhalten sich in Rust ähnlich wie in C++, sie sind anhand eines `&` erkennbar. Aus Sicherheitsgründen unterscheidet Rust zwischen immutable und mutable Referenzen. Nur mutable Referenzen, welche mit `&mut` deklariert werden, erlauben das Verändern des Inhalts des referenzierten Objekts. Dabei können immer nur entweder eine beliebige Anzahl an immutable Referenzen oder eine einzige mutable Referenz gleichzeitig existieren, dies verhindert ein Data Race bei Multithread Programmen und ist eine von vielen (Code)Sicherheitsfeatures von Rust. Dazu ist eine mutable Referenz nur möglich, wenn das referenzierte Objekt als mutable instanziiert wurde.  

Rust prüft das Einhalten all dieser Regeln vorm Kompilieren mit dem Analyzer, es ist also nicht möglich, das während der Laufzeit ein Fehler auftritt weil die Ressourcen eines Objekts bereits an anderer Stelle freigegeben wurden oder der Besitzer gewechselt wurde, gleiches gilt auch für die Regeln der Referenzen.  
Dabei werden eindeutige Fehlernachrichten verwendet, welche u. a. anzeigen, in welcher Zeile ein Objekt bewegt oder fehlerhaft geliehen wurde.  
Beispiel:  
````
{ // Block

    let mut var1 = String::new(); // var1 besitzt den neu erstellten String
    let mut var2 = var1; // Der Besitz des Strings wurde an var2 übergeben, var1 ist nun nicht mehr verwendbar.
    { /* Neuer innerer Block*/

        let ref1 = &var2;
        let ref2 = &var2; // Möglich da beide Referenzen immutable sind
        /*let ref3 = &mut var2;*/ /* Nicht möglich da ref1 und ref2 noch existieren, würde nicht kompilieren */

    } // ref1 und ref2 werden aufgelöst/freigegeben
    let ref3 = &mut var2; /* Hier möglich da keine anderen Referenzen mehr existieren */

} /* var2 und ref3 werden aufgelöst, dabei wird der Speicher des Strings wieder freigegeben */

let var3 = var2; /* Fehler da var2 nicht mehr existiert -> würde nicht Kompilieren */
````

### Struct
In Rust existieren keine Klassen wie in C++, stattdessen werden die aus C bekannten Structs verwendet, um mehrere Datentypen zu einem Objekt zu kombinieren. Anders als in C können in Rust für Structs jedoch auch Funktionen definiert werden und es ist auch möglich, den Feldvariablen Sichtbarkeiten zuzuweisen. Ein wichtiger Unterschied zu C++ Klassen ist allerdings die fehlende Vererbung zwischen Structs.  
Structs können folgenderweise definiert werden.  
```
struct Person {
    pub name: String, /* Öffentlich zugreifbar */
    mut age: u16 /* Privat */
} 
```

Eine Instanz wird erzeugt, indem jedem Feld ein Wert zugewiesen wird. Als Beispiel für `Person` (ohne berücksichtigung der Sichtbarkeit).
```
let p = Person { 
    name: "Max Mustermann".to_string(),
    age: 30 
    }
```

## Trait
Zur Darstellung, dass mehrere unterschiedliche Struct Typen eine ähnliche Funktion erfüllen, werden in Rust Trait Definitionen verwendet.  
Traits sind vergleichbar mit einem Interface oder auch abstrakte Klassen aus C++ (ohne Felder!). In ihnen können öffentliche Funktionen deklariert werden, dabei ist das Definieren des Inhalts der Funktion nicht notwendig, im Gegensatz zu Interfaces aus anderen Sprachen allerdings trotzdem möglich.

Wichtig dabei zu beachten ist, dass es sich bei einem Trait nicht um ein Objekt handelt, es gibt also keine eigenständigen Traitinstanzen sondern nur Objektinstanzen, welche als ein Trait behandelt werden. Sollen also mehrere unterschiedliche Typen, welche ein gemeinsames Trait besitzen, zusammen verarbeitet werden (Polymorphismus) ist dies nur mithilfe von Referenzen und Pointer möglich.

## Enum
Enums sind ebenfalls aus C bekannt, in Rust wurden diese jedoch stark erweitert und dienen als wichtiger Bestandteil der Sprache. In C können Enums nur Integer darstellen, in Rust ist es allerdings möglich, jeden Typ in einem Wert darzustellen oder auch auf einen Typ zu verzichten. Dadurch ähneln sie von der Definition her den Structs mit ihren Feldvariablen, jedoch mit dem wichtigen Unterschied, dass einem Enum immer nur ein Wert zugewiesen werden kann, wodurch sie einer Kombination von Enum und Union aus C ähneln. 

Es ist auch möglich, mehrere Parameter für die Werte eines Enums zu definieren. Ein Beispiel dafür ist der häufig verwendete `Option<T>` Enum, welcher zum Überprüfen des Vorhandenseins eines Objekts dient. Dieser besitzt zwei mögliche Werte `Some(T)` und `None`, bei `T` handelt es sich dabei um ein Template Parameter, welcher mithilfe von matching Operatoren abgefragt werden kann.

Weiterhin ist es in Rust auch möglich, Funktionen für Enums zu definieren, welche als erweiterungen dienen wie z. B. die `unwrap()` Funktion von `Option<T>` welche im Fall eines `Some(T)` `T` zurückgibt, aber bei `None` ein Panic verursacht.

## Matching
Zum besseren Verarbeiten der unterschiedlichen Fälle von Enums bietet Rust mehrere matching Operatoren an.  
Bei `match` handelt es sich um eine schlauere Erweiterung der `switch` Anweisung aus C, welche es ermöglicht, nicht nur zwischen den Werten eines Enums zu unterscheiden, sondern auch zwischen den Parameter Werten. Bei einer `match` Unterscheidung müssen immer alle Möglichkeiten behandelt werden.  
Beispiel `match` für `Option<i32>`:
```
match o {
    Some(1) => 1, /* Nur wenn der Wert Some(1) ist */
    Some(t) => t*2, /* Nur wenn der Wert Some mit einem beliebigen Parameter ist */
    None => -1 // Alle Möglichkeiten behandelt
}
```
Wenn nur ein einziger Fall geprüft werden soll, kann die `if let` Anweisung oder auch eine `while let` Schleife verwendet werden, welche die enthaltenen Anweisungen nur ausführt, falls die Bedienung auf den Wert zutrifft.  
Einfaches Beispiel für `if let`:
```
if let Some(v) = e { /* v wurde davor noch nicht definiert, e ist ein Enum Objekt */
    println!("Der Wert ist {}", v);
}
```
Um sicherzustellen das z. B. ein `Option<T>` korrekt verarbeitet wird ist Rust in der Lage alle Enum Fallunterscheidungen statisch zu überprüfen. Dadurch ist es z. B. im Fall von `Option<T>` deutlich schwerer ein Fehler aufgrund der abwesenheit eines wertes zu verursachen (Null-Pointer-Exception in C(++)).

## Funktionen und impl
In Rust können Funktionen alleinstehend geschrieben werden oder an einen Typ (Struct oder Enum) gebunden werden. Beim Deklarieren von Funktionen müssen immer die Typen der Parameter und der Rückgabetyp angegeben werden. Jede Funktion wird unabhängig vom Rückgabewert (auch bei keiner Rückgabe) mit dem Schlüsselwort `fn` definiert.  

Für Funktionen gelten ebenfalls die Regeln mit der Besitzergreifung, weshalb Parameter, bei denen keine Besitzergreifung notwendig ist, als Referenzen übergeben werden. Wenn eine Referenz von einer Funktion zurückgegeben werden soll, muss eine Lifetime verwendet werden. Auf das Einhalten dieser Bedienungen prüft Rust statisch mit dem Analyzer.  

Ein Rückgabewert ist entweder durch eine `return` oder anhand eines fehlenden `;` in der letzten Zeile der Funktion erkennbar.   
Wenn Funktionen für einen Typ geschrieben werden, geschieht dies nicht im Körper des Typs, sondern in einem eigenen Block, welcher mit dem Syntax `impl TYPNAME` beginnt. `impl` wird ebenfalls für das Implementieren von Traits für ein Typ verwendet, mit dem Syntax `impl TRAIT for TYPNAME`.

Wenn eine Funktion in einem `impl` Block auf die Felder einer Instanz zugreifen muss, wird ein `self` als Parameter benötigt(vergleichbar mit Python), welches meist an erster Stelle steht. Dabei muss auf die Verwendung von `&` und `mut` geachtet werden. \
Wenn nur `self` verwendet wird, wird die Instanz nach dem Funktionsaufruf konsumiert, ist dies nicht gewünscht, muss `&self` verwendet werden um die Instanz als Referenz zu übergeben. Das verändern von Feldern (welche mit `mut` deklariert wurden) ist nur mit einem `mut self` möglich (auch mit `&` anwendbar), dadurch ist es nur möglich, diese Funktion aufzurufen, wenn die Instanz mit einem `mut` deklariert wurde. Dieses Verhalten ähnelt den `const` Klassenfunktionen aus C++, welche dort benötigt werden, um `const` Instanzen das Aufrufen von Klassenfunktionen zu erlauben. Wenn kein `self` als Parameter übergeben wird, handelt es sich um eine statische Funktion, welche auch ohne Instanz aufrufbar ist.  


## Lifetimes
Lifetimes werden in Rust für Referenzen in Funktionen und für Felder von Structs benötigt. Lifetimes werden an der gleichen stelle wie Templates definiert, unterscheiden lassen sie sich anhand des `'` Zeichens.

Als Beispiel eine Funktion mit der benannten Lifetime `a` wobei angegeben wird, das die Rückgabe und der Parameter `s` dasselbe Objekt Referenzieren:  
`fn funktion<'a>(s: &'a str) -> &'a str {...}`

Wenn eine Referenz auf ein Objekt von einer Funktion zurückgegeben wird, muss (statisch) garantiert werden, das das Objekt außerhalb der Funktion existiert, d. b. das Referenzierten Objekt wird entweder bereits von einem Parameter referenziert oder ist statisch definiert, in dem Fall wird dann die spezielle Lifetime `'static` verwendet.  

Die gleichen Bedienungen gelten auch für Structs, da garantiert werden muss, dass alle Felder mindestens solange wie die Instanz des Structs leben.  
Aufgrund von Lifetimes ist also nicht möglich, eine Referenz auf ein Objekt zu besitzen, welches nicht mehr existiert, auch dies prüft Rust mit dem Analyzer.

## Schleifen
Rust unterstützt unterschiedliche Schleifenarten.  
`loop` ist eine einfache Endlosschleife, `while` prüft nach jeder Iteration eine boolesche Bedingung und bricht ab, wenn diese negativ ist, und `for` iteriert über eine Kollektion oder Iterator und ähnelt dadurch der `foreach` Schleife aus C++( `for (auto ele: collection)` ). Eine zählende Schleife gibt es nicht, dafür kann eine `for` Schleife in Kombination mit einem Range Operator verwendet werden.  
Beispiel:
```
for i in 0..10 { // Von einschließlich 0 bis exklusive 10
    ...
}
```

## Exceptions
In Rust gibt es zwei arten von Fehlern, `Panic` und `Result<T,E>`.  
Eine Panic ist ein Fehler während der Laufzeit, welcher zur sofortigen Terminierung des Programms(bzw. des aktuellen Threads) führt, dabei wird als letztes eine (oft benutzerdefinierbare) Fehlernachricht ausgegeben. Eine Panic kann nicht abgefangen werden, es gibt also kein `try ... catch` wie in C++, da eine Panic immer dann entsteht, wenn ein unbehandelbarer Fehler auftritt und Rust einfach nicht weiß, wie weitergehend gehandelt werden soll. Als Beispiel geschieht dies bei einem ungültigen Zugriff mit dem `[]` (Index) Operator auf einen Vektor oder auch das Aufrufen der Funktion `unwrap()` von `Option<T>` und `Result<T,E>`, bei einem `None` oder `Err(E)` Wert.

Wenn Fehler stattdessen behandelbar sein sollen, wird dafür `Result<T,E>` verwendet. Dabei handelt es sich um einen Enum mit zwei möglichen Werten `Ok(T)` und `Err(E)`, welcher den Erfolg eines fehleranfälligen Codes beschreibt. Jede Funktion, in welcher ein Fehler entstehen könnten, z. B. aufgrund eines ungültigen Parameters sollte deshalb ein `Result` zurückgeben. Diese Fehler sollen dann mithilfe von matching Operatoren behandelt werden, analog zu `Option<T>`.  

Viele Typen bieten Funktionen an, welche entweder eine Panic verursachen oder ein `Result` zurückgeben, z. B. `Vec` besitzt zwei Funktionen zum Reservieren von Kapazitäten, `reserve()` und `try_reserve()`. In Fällen, in denen sichergestellt ist, dass das Reservieren zu keinem Fehler führt, kann `reserve()` verwendet werden, falls nicht, sollte `try_reserve()` benutzt werden, da es ein `Result` zurückgibt, um den Fehler zu Verabeiten.

## Cargo
Cargo ist ein mitgelieferter Paketmanager für Rust, welche das verwenden von externen Abhängigkeiten, das erstellen und Verteilen von Paketen (Crates) und das Ausführen und Testen dieser ermöglicht.

Mit dem Befehl `Cargo new NAME` wird eine neue Crate in einem neuen Ordner erstellt. Diese besteht zuerst aus einem `src` Ordner mit einer `main.rs` Datei, welche Code für ein einfaches "Hello, World" Programm beinhaltet, sowie einer `Cargo.toml` Datei, welche Informationen über die Crate verwaltet (Name, Autor, Version etc.) und wo auch externe Crates als Abhängigkeiten eingetragen werden können. Diese können entweder lokal oder aus dem Internet (https://crates.io) bezogen werden. 

Mit `cargo build`, `cargo run` und `cargo test` ist es sehr einfach, Crates zu kompilieren, auszuführen (bei ausführbaren Crates) und zu Testen. Cargo findet dabei selbst heraus, welche Dateien zum kompilieren benötigt werden, es wird also keine Konfiguration wie mit Make/CMake benötigt.  
Via `cargo check` kann auch der Code der Crate mithilfe des Rust Analyzer überprüft werden, bevor er kompiliert wird.

Alle Aufgaben in diesem Projekt wurden mithilfe von Cargo erstellt, externe Abhängigkeiten wurden dabei in keiner Aufgabe benötigt.

## Module
Größere Projekte in Rust können in Module unterteilt werden. Dies hilft der Übersicht des Programmes und ermöglicht das Festlegen von Modul internen Sichtbarkeiten. Module sind anhand des `mod` Schlüsselworts erkennbar, zudem stellt jede Datei und jeder Ordner in `src` (ausnahme `lib.rs` und `main.rs`) implizit ein Module dar.