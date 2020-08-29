# Projektarbeit "Implementieren von Softwareprojekt Aufgaben in Rust"

## Allgemein
Die Projektarbeit entstand im laufe des Sommersemesters 2020 und wurde von Professor Dr Martin Sulzmann betreut und ist Teil des Studiengangs Bachelor für Informatik an der Hochschule Karlsruhe. Ziel ist die Implementierung der Aufgaben des Softwareprojekt Labors in Rust um dadurch unterschiede zwischen Rust und C(++) zu veranschaulichen.  

Die Projektarbeit ist durch vier Aufgaben unterteilt welche sich in einem Unterordner in `src` befinden. Neben den Lösungen der Aufgabenstellungen beinhaltet die Projektarbeit auch eine Dokumentation welche Rust und die Lösungen näher erklärt. Neben der Allgemeinen Dokumentation über Rust und die Projektarbeit (diese Datei) wurde der Code jeder Aufgabe Kommentiert und zusätzlich existiert eine README.md Datei für jede Aufgabe um den Code zu erläutern.

In jeder Aufgabe befindet sich ein Link zur Aufgabenstellung, da dieser jedoch auf den aktuellen Commit eines externen Repositories verweist ist es möglich das die Aufgabenstellung dort inzwischen verändert wurden.  
Eine archivierte Version aller Aufgabenstellungen ist über diesen [Link](https://htmlpreview.github.io/?https://github.com/sulzmann/SoftwareProjekt/blob/5dc75c9d4f84bfe9e8396fab6785276f25fd5cbe/schein.html) erreichbar.

## Rust
Rust ist eine von Mozilla seit 2010 in entwicklung befindente Programmiersprache, welche sich dabei auf das Schreiben von sicheren, schnellen und Parallel laufbaren Code fokussiert. Dabei kombiniert Rust Aspekte von mehreren unterschiedlichen Programmiersprachen, besitzt jedoch viel besonderheiten wodurch sich Rust doch stark von den meisten gänigen Programmiersprachen unterscheidet. Eine dieser Besonderheit ist das verwalten von Speicher-Ressourcen anhand eines Ownership Systems oder auch die Abwesenheit von null-Pointer. Weiterhin zeichnet sich Rust durch einen Schlauen Analyser aus welcher prüft ob die meisten Rust Idiome eingehalten werden und dadurch einige Typesche laufzeitfehler reduziert. Jedoch ist es explizit auch möglich mit Rust unsicheren Code zu schreiben, dadurch ist es z.B. auch möglich C bibliotheken zu verwenden.

## Variablendeklaration
Variablen werden mit dem Syntax `let NAME` deklariert. Auch wenn in Rust einer Variable nur einen Typ zugewiesen ist muss im gegensatz zu C der Typ nicht immer explizit angegeben, werden sollange auf den Typ nachfolgend eindeutig geschlussfolgert werden kann, ebenso müssen Variablen nicht direkt Initialisiert werden. Es ist aber auch möglich den Typ bei der Deklaration anzugeben, dies dient nicht nur der lesbarkeit sonder ist auch in einigen Fällen notwendig wenn der Typ mehrdeutig Interpretiert werden kann.  
In Rust sind alle Variablen, solange nicht speziell angegeben unveränderbar(const), wenn eine Variable veränderbar (mutable) sein soll wird das Schlüsselwort `mut` bei der Deklaration benötigt, damit geht Rust den gegenseitigen Weg von C(++) wo jede Variable welche nicht mit `const` deklariert wurde veränderbar ist. Rust prüft das einhalten dieser Regeln statisch vorm Kompilieren mit dem Analyser. 
Beispiele:  
```
let word: String; // Variable mit Typangabe

let word; // Typerkennung anhand des Kontext
word = String::new();

let mut word = "Hello".to_string(); // Veränderbare Variable
word = "World".to_string();
```

## Speicherverwaltung und Ownership
Zur Speicherverwaltung verlässt sich Rust weder auf das Manuelle Freigeben von Ressourcen wie in C(++) oder auf einen Garbage-Collector wie Java, C#, Python etc. Stattdessen werden Ressourcen mithilfe von Ownerships verwaltet. Jedes Objekt in Rust gehört einer Variable an, diese Variable besitzt das Objekt und sobald die Variable am ende ihres Blocks(Scope, meist anhand von `{}` Klammern erkennbar) erreicht werden alle Ressourcen des Objekts freigegeben, bei Typen welche aus mehren Objekten bestehen wie z.B. Structs werden auch diese Objekte freigegeben, vorausgesetzt das Struct ist der Besitzer der Objekte.  
Dadurch müssen Ressourcen nicht manuelle freigegeben werden und es gibt keine Periodischen Performance einbrüche aufgrund eines Garbage-Collectors. Das Ownership Prinzip von Rust lässt sich leicht in C(++) vorstellen wenn ein Programm geschrieben wird bei dem jede Variable auf dem Stack abgelegt ist und der Heap sowie Pointer (fast) nicht verwendet wird.  
Um das Kopieren von anspruchsvolleren Objekten(Struct, Enum) zu vermeiden ist der `=`-Operator in Rust ein Move und kein Kopieroperator (es gibt jedoch Ausnahmen wie die Primären Datentypen (i32, f32, char etc.)), der ausdruck `var2 = var1;` bedeutet also das die Variable `var2` besitzt des Objekts von `var1` ergreift, dies führt auch dazu das `var1` jetzt kein Objekt mehr besitzt wodurch `var1` nicht mehr verwendet werden kann (bis die Variable wieder besitzt eines Objekt ergreift).  
Dies gilt z.B. auch beim Übergeben des Objekts als Parameter einer Funktionen oder beim einfügen in eine Collection(Vector, HashSet etc.). Wenn aber das Objekt übergeben werden soll ohne dabei den Besitzer zu wechseln werden Referenzen verwendet und es wird vom Leihen (borrow) des Objekts gesprochen. Referenzen verhalten sich in Rust ähnlich wie in C, sie sind anhand eines `&` erkennbar. Aus sicherheitsgründen unterscheidet Rust zwischen immutable und mutable Referenzen, nur mutable Referenzen welche mit `&mut` dargestellt werden erlauben das verändern des Inhalts des Referenzierten Objekts. Dabei können immer nur entweder eine Beliebige anzahl an immutable Referenzen oder eine einzige mutable Referenz gleichzeitig existieren, dies verhindert ein Data Race bei Multithread Programmen und ist eine von vielen (Code)Sicherheitsfeatures von Rust. Dazu ist eine mutable Referenz nur möglich wenn das Referenziert Objekt als mutable instanziiert wurde.  
Rust prüft das einhalten all dieser Regeln vorm Kompilieren mit dem Analyser, es ist also nicht möglich das wärend der Laufzeit ein Fehler auftritt weil die Ressourcen eines Objekts bereits an anderer Stelle freigegeben wurden oder der besitzer gewechselt wurde. Gleiches gilt für die Regeln für die Referenzen. Dabei werden eindeutige Fehlernachrichten verwendet welche u.a. anzeigen in welcher Zeile ein Objekt bewegt oder fehlerhaft geliehen wurde.  
Beispiel:  
````
{ // Block

    let mut var1 = String::new(); // var1 besitzt den neu erstellten String
    let mut var2 = var1; // Der Besitz des Strings wurde an var2 übergeben, var1 ist nun nicht mehr verwendbar.
    { /* Neuer innerer Block*/

        let ref1 = &var2;
        let ref2 = &var2; // Möglich da beide Referenzen immutable sind
        /*let ref3 = &mut var2;*/ /* Nicht möglich da ref1 und ref2 noch exisitieren, würde nicht kompilieren */

    } // ref1 und ref2 werden aufgelöst/freigegeben
    let ref3 = &mut var2; /* Hier möglich da ref1 und ref2 nicht mehr existieren */

} /* var2 und ref3 werden aufgelöst, dabei wird der Speicher des Strings wieder freigegeben */

let var3 = var2; /* Fehler da var2 nicht mehr existiert -> würde nicht Kompilieren */
````

### Struct
In Rust existieren keine Klassen wie in C++, stattdessen werden die aus C bekannten Structs verwendet um mehrere Datentypen zu einem Objekt zu kombinieren. Anders als in C können in Rust für Structs jedoch auch Funktionen definiert werden und ist auch möglich den Feld Variablen Sichtbarkeiten zuzuweisen. Ein Wichtiger Unterschied zu C++ Klassen ist allerdings die fehlende Vererbung zwischen Structs.  
Structs können folgenderweise definiert werden.  
```
struct Person {
    pub name: String,
    mut age: u16
} 
```
Eine Instanz wird erzeugt indem jedem Feld ein Wert zugewiese wird. Als Beispiel für `Person` (ohne berücksichtigung der Sichtbarkeit)  
```
let p = Person { 
    name: "Max Mustermann".to_string(),
    age: 30 
    }
```

## Trait
Zur Darstellung das mehrere unterschiedliche Struct Typen eine ähnliche Funktion erfüllen, werden in Rust Trait definitionen verwendet. Traits sind vergleichbar mit einem Interface oder auch Abstrakte Klassen aus C++ (ohne Felder!) in denen Öffentliche Funktionen deklariert werden können, dabei ist das Definieren des Inhalts der Funktion nicht notwendig. Im gegensatz zu Interfaces aus anderen Sprachen allerdings trotzdem möglich.  
Wichtig dabei zu beachten ist das es sich bei einem Trait nicht um ein Objekt handelt, es gibt also keine eigenständigen Trait instanzen sondern nur Objekt(Struct) Instanzen welche als ein Trait behandelt werden, sollen also mehrere unterschiedliche Typen welche ein gemeinsames Trait besitzen zusammen verarbeitet werden (polymorphismus) ist dies nur mithilfe von Referenzen und Pointer möglich.

## Enum
Enums sind ebenfalls aus C bekannt, in Rust wurden diese jedoch stark erweitert und dienen als wichtiger bestandteil des Sprache. In C können Enums nur Integer darstellen, in Rust ist es allerdings möglich jeden Typen in einem Wert darzustellen oder auch auf einen Typen zu verzichten. Dadurch ähneln sie von der Definition her den Structs mit ihren Feld Variablen jedoch dem Wichtigen unterschied das einem Enum immer nur ein Wert zugewiesen werden kann, wodurch sie einer Kombination von Enums und Unions aus C ähneln.  
Es ist auch möglich mehrere Parameter für die Werte eines Enums zu definieren. Ein Beispiel dafür ist der häufig verwendete `Option<T>` Enum, welcher zum Überprüfen des vorhandenseins eines Objekts dient. Dieser besitzt zwei mögliche Werte `Some(T)` und `None`, bei `T` handelt es sich dabei um ein Template Parameter welcher mithilfe von matching Operatoren abgefragt werden kann.  
Weiterhin ist es in Rust auch möglich Funktionen für Enums zu definieren welche als erweiterungen dienen wie z.B. die `unwrap()` Funktion von `Option<T>` welcher im Fall eines `Some(T)` `T` zurückgibt aber bei `None` ein Panic verursacht.

## Matching
Zum besseren verarbeiten der Unterschiedlichen Fälle von Enums bietet Rust mehrere matching Operatoren an.  
Bei `match` handelt es sich um einen schlauere erweiterung der switch Anweisung aus C welcher es ermöglicht nicht nur zwischen den Werten eines Enums zu unterscheiden sondern auch zwischen den Parameter Werten. Bei einer `match` unterscheidung müssen immer alle möglichkeiten abgearbeitet werden. Beispiel `match` für `Option<i32>`:
```
match o {
    Some(1) => 1, /* Nur wenn der Wert Some(1) ist */
    Some(t) => t*2, /* Nur wenn der Wert Some mit einem beliebigen Parameter ist */
    None => -1
}
```
Wenn nur ein einziger Fall geprüft werden soll kann die `if let` Anweisung oder auch eine `while let` Schleife verwendet werden, welche die enthaltenen anweisungen nur ausführt falls die Bedienung auf den Wert zutrifft.  
Einfaches Beispiel für `if let`:
```
if let Some(v) = e { /* v wurde davor noch nicht definiert, e ist ein Enum Objekt */
    println!("Der Wert ist {}", v);
}
```
Um Sicherzustellen das z.B. ein `Option<T>` korrekt verarbeitet wird ist Rust inderlage alle Enum fallunterscheidungen statisch zu überprüfen. Dadurch ist es z.B. im Fall von `Option<T>` deutlich schwerer ein Fehler aufgrund der abwesenheit eines wertes zu verursachen (Null-Pointer Exception in C(++)).

## Funktionen und impl
In Rust können Funktionen alleinstehend geschrieben werden oder an einen Typ (struct oder Enum) gebunden werden. Beim Deklarieren von Funktionen müssen immer die Typen der Parameter und der Rückgabetyp angegeben werden. Jede Funktion wird unabhängig vom Rückgabewert (auch bei keiner Rückgabe) mit dem Schlüsselwort `fn` definiert.  
Für Funktionen gilt ebenfalls die Regeln mit der Besitzergreifung weshalb Parameter bei denen keine Besitztergreifung notwendig ist meist als Referenzen übergeben werden sollten. Wenn eine Referenz von einer Funktion zurückgegeben werden soll, müssen Lifetimes verwendet werden. Auf die einhaltung dieser Bedienungen prüft Rust Statisch mit dem Analyser.  
Ein Rückgabewert ist entweder durch eine `return` oder anhand des fehlnden `;` in der letzten Zeile der Funktion erkennbar.   
Wenn Funktionen für einen Typ geschrieben werden geschieht dies nicht im Körper des Typs sondern in einem eigenen Block welcher mit dem Syntax `impl TYPNAME` beginnt. `impl` wird ebenfalls für das Implementieren von Traits für ein Typ verwendet, mit dem Syntax `impl TRAIT for TYPNAME`.
Wenn eine Funktion in einem `impl` Block auf die Felder einer Instanz zugreifen muss, wird ein `self` als Parameter benötigt(vergleichbar mit Python), welches meist an erster Stelle steht. Dabei muss auf die verwendung von `&` und `mut` geachtet werden, wenn nur `self` verwendet wird würde die instanz nach dem Funktionsaufruf konsumiert werden, ist dies nicht gewünscht muss `&self` verwendet werden damit eine Referenz übergeben wird. Das verändern von Feldern (welche mit `mut` deklariert wurden) ist nur mit einem `mut self` möglich (auch mit `&` anwendbar), dadurch ist es nur möglich diese Funktion aufzurufen wenn die Instanz mit einem `mut` deklariert wurde. Dieses Verhalten ähnelt den `const` Klassenfunktionen aus C++ welche benötigt werden um `const` Instanzen das aufrufen von Klassenfunktionen zu erlauben. Wenn kein `self` als Parameter übergeben wird handelt es sich um eine Statische Funktion welche auch ohne Instanz aufrufbar ist.  


## Lifetimes
Lifetimes werden in Rust für Referenzen in Funktionen und Felder von Structs benötigt. Lifetimes werden an der gleichen stelle wie Templates definiert, unterscheiden lassen sie sich anhand des `'` Zeichens.  
Als Beispiel eine Funktion mit der benannten Lifetime `a` wobei angegeben wird das die Rückgabe und der Paramter s dasselbe Objekt Referenzieren:  
`fn funktion<'a>(s: &'a str) -> &'a str {...}`   
Wenn eine Referenz auf ein Objekt von einer Funktion zurückgegeben wird muss (statisch) garantiert werden das das Objekt ausserhalb der Funktion existiert d.b. das Referenzierten Objekt wird entweder bereits von einem Parameter Referenziert oder ist statisch definiert, dafür wird die spezielle Lifetime `'static` verwendet.  
Das Gleiche gilt für Structs da garantiert werden muss das alle Felder mindestens solange wie eine Instanz des Structs leben.  
Aufgrund von Lifetimes ist also nicht möglich eine Referenz auf ein Objekt zu besitzten welches nicht mehr existiert, auch dies stellt Rust Statisch sicher.

## Schleifen
Rust unterstüzt unterschiedliche Schleifenarten. `loop` ist eine einfache endlosschleife, `while` prüft nach jeder iteration eine Bedienung und bricht ab wenn diese Negativ ist und `for` iteriert über eine Kollektion oder Iterator und ähnelt dadurch der `foreach` schleife aus C++( `for (auto ele: collection)` ). Eine Zählende Schleife gibt es nicht, dafür kann eine `for` Schleife in kombination mit einem Range Operator verwendet werden. Beispiel:
```
for i in 0..10 { // Von einschließlich 0 bis exklusive 10
    ...
}
```

## Exceptions
In Rust gibt es zwei arten von Fehlern, Panic und `Result<T,E>`.  
Eine Panic ist ein fehler während der Laufzeit welcher zur Sofortigen terminierung des Programms(bzw des aktuellen threads) führt, dabei wird als letztes eine (oft benutzerdefinierbare) Fehlernachricht ausgegeben. Eine Panic kann nicht abgefangen werden, es gibt also kein `try ... catch` wie in C++, da eine Panic immer dann entsteht wenn ein unbehandelter Fehler auftritt und Rust einfach nicht weiß wie fortführend gehandelt werden soll. Als Beispiel ist dies bei einen ungültigen zugriff mit dem [] Operator auf einen Vektor oder auch die Funktion `unwrap()` von `Option<T>` und `Result<T,E>`.  
Wenn Fehler stattdessen behandelt werden sollen wird dafür `Result<T,E>` verwendet. Dabei handelt es sich um einen Enum mit zwei möglichen Werten Ok(T) und Err(E) welche den Erfolg eines Fehleranfälligen Codes beschreibt. Jede Funktion in welcher ein Fehler entstehen könnten z.B. aufgrund eines ungültigen Parameters sollte deshalb ein Result zurückgeben. Diese Fehler sollen dann mithilfe von matching Operatoren behandelt werden, analog zu Option<T>.  
Viele Typen bieten Funktionen an welche entweder eine Panic verursachen oder ein `Result` zurückgeben, z.B. `Vec` besitzt zwei Funktionen zum Reservieren von Kapazitäten, `reserve()` und `try_reserve()`. In fällen in denen sichergestellt ist das das Reservieren zu keinem Fehler führ kann `reserve()` verwendet werden, falls nicht sollte `try_reserve()` benutzt werden da es ein `Result` zurückgibt um den Fehler zu verabeiten.

## Cargo
Cargo ist ein mitgelieferter Paketmanager für Rust welche das verwenden von externen Abhängigkeiten, das erstellen und Verteilen von Paketen (Crates) und das ausführen/testen dieser ermöglicht. Mit dem Befehl `Cargo new NAME` wird eine neue Crate in einem neuen Ordner erstellt, diese besteht zuerst aus einem `src` Ordner mit einer `main.rs` Datei welche Code für ein einfaches "Hello, World" programm beinhaltet sowie einer `Cargo.toml` Datei welche informationen über die Crate verwaltet (Name, Autor, Version etc.) und wo externe Crates als Abhängigkeiten eingetragen werden können, diese können entweder lokal oder aus dem Internet (https://crates.io) bezogen werden.  
Mit `cargo build`, `cargo run` und `cargo test` ist es sehr einfach Crates zu erstellen, auszuführen (bei ausführbaren Crates) und zu Testen. Cargo findet dabei selbst heraus welche Dateien zum erstellen benötigt werden, es wird also keine Konfiguration wie mit Make/CMake benötigt.  
Mit `cargo check` kann auch der Code der Crate mithilfe des Rust Analyser überprüft werden, bevor er kompiliert wird.

Alle Aufgaben in diesem Projekt wurden mithilfe von Cargo erstellt, externe Abhängigkeiten wurden dabei in keiner Aufgabe benötigt.

## Module
Größere Projekte in Rust können in Module unterteilt werden. Dies hilft der Übersicht des Programmes und ermöglicht das Festlegen von Module internen Sichtbarkeiten. Module sind anhand des `mod` Schlüsselworts erkennbar zudem stellt bei Bibliotheksprogrammen jede Datei/ jeder Ordner in `src` (ausnahme lib.rs und main.rs) ein Module dar.