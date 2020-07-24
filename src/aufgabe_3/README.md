# Aufgabe 3: String Klasse mit überladenen Operatoren
[Link zur Aufgabenstellung](https://sulzmann.github.io/SoftwareProjekt/schein.html#(5))

Der Code ist in mehrere Dateien aufgeteilt. 
- In [src/main.rs](src/main.rs) befindet sich der Code für das ausführen als Binary.
- In [src/lib.rs](src/lib.rs) befindet sich die Definition des CString Structs sowie dazugehörige unit-tests, dies ist eine Library-file welche an anderen stellen (wie [main.rs](src/main.rs)) importiert werden kann.
- In [tests/integration_test.rs](tests/integration_test.rs) befinden sich integration tests.

Ausführen von [main.rs](src/main.rs) mit dem Befehl `cargo run` \
Ausführen der unit- und integration-tests mit dem Befehl `cargo test`