pub mod custom_string {
    use std::ops::{Index, IndexMut, AddAssign};
    use std::fmt::{Display, Formatter, Result};

    pub struct CString {
        pub size: usize,
        // Wie in Aufgabe2 wird hier eine Vec verwendet da dieser leichter zu verändern ist.
        // Dabei wird kein '\0' als Endzeichen verwendet, da ein Vec immer seine eigene länge kennt.
        // Rust Arrays müssen eine feste Länge beim compilieren besitzen, deshalb wird hier ein Vec verwendet
        // pub (super) bedeutet dabei das diese Feld für alle module/funktionen in dieser datei öffentlich ist, dies dient zu Testzwecken.
        pub (super) vec: Vec<char>
    }

    // Hinweis: Rust erlaubt kein overloading von Funktionen
    impl CString {
        // Neuer leerer CString
        pub fn new_empty() -> CString {
            CString {
                size: 0,
                vec: Vec::new()
            }
        }

        // Neuer CString bestehend aus einem char
        pub fn new_char(c: char) -> CString {
            CString {
                size: 1,
                vec: vec![c]
            }
        }

        // Neuer CString bestehend aus einen Vec<char>
        pub fn new_vec(vec: &Vec<char>) -> CString {
            CString {
                size: vec.len(),
                // vec wird hier einfach kopiert (deep-copy)
                vec: vec.clone()
            }
        }

        // Kopierkonstruktor
        pub fn new_string(original: &CString) -> CString {
            // Iteration nach C Art, theoretisch wäre auch das aufrufen von new_vec oder zuweisen auf original.vec.copy() möglich
            let mut values = Vec::new();
            for c in original.vec.iter() {
                values.push(*c);
            }

            CString {
                size: values.len(),
                vec: values
            }
        }

        // Neuer CString basierend auf einen str
        pub fn new_str(original: &str) -> CString {
            let mut values = Vec::new();
            for c in original.chars() {
                values.push(c);
            }

            CString {
                size: values.len(),
                vec: values
            }
        }

        // Move konstruktor, hierbei wird ein tatsächlich ein neuer CString angelegt und die felder von input dahin bewegt.
        // Dies konsumiert input.
        // Da Rust standardmäßig variablen mit = bewegt wird dieser Konstruktor eigentlich nicht benötigt
        pub fn new_move(input: CString) -> CString {
            CString {
                size: input.size,
                vec: input.vec
            }
        }

        // Zuweisungsfunktion
        pub fn assign(&mut self, rhs: &CString) {
            *self = CString::new_string(rhs);
        }
    }

    // Überschreiben des immutable [] operators (nur lesen)
    impl Index<usize> for CString {
        type Output = char;

        fn index(&self, index: usize) -> &Self::Output {
            if index >= self.size {
                panic!("CString index out of bounds: the size is {} but the index is {}! The index must be smaller the the size", self.size, index);
            }

            &self.vec[index]
        }
    }

    // Überschreibung des mutable [] operators, dies erlaubt die verwende als lvalue (lesen und schreiben)
    impl IndexMut<usize> for CString {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            if index >= self.size {
                panic!("CString index out of bounds: the size is {} but the index is {}! The index must be smaller the the size", self.size, index);
            }

            &mut self.vec[index]
        }
    }

    // Überschreibung von +=
    // Erstellt einen neuen Vec mit der Summe der beiden Größen
    // Dieser wird mit den Inhalt der beiden CStrings befüllt
    impl AddAssign<&CString> for CString {
        fn add_assign(&mut self, rhs: &Self) {
            let new_size = self.size + rhs.size;
            let mut new_vec = Vec::with_capacity(new_size);
            // chain() knüpft zwei iteratoren aneinander
            for c in self.vec.iter().chain(rhs.vec.iter()) {
                new_vec.push(*c);
            }

            self.size = new_size;
            self.vec = new_vec;
        }

    }

    // Dies ermöglicht die darstellung bei Formatierungen analog zum << operator in C++
    // Erlaubt die verwendung von z.B. println!("{}", CString-instance)
    impl Display for CString {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for i in 0..self.size {
                // Sollte write! einen Err zurückgeben wird dieser aufgrund des ? operators als Rückgabewert verwendet werden
                write!(f, "{}", self[i])?
            }
            Ok(())
        }
    }
}

// Unit-Tests für alle Methoden von CString. Unit-Tests werden mithilfe von 'cargo test' ausgeführt
#[cfg(test)]
mod tests {
    use super::custom_string::CString;

    #[test]
    fn new_empty() {
        let s = CString::new_empty();

        assert_eq!(s.size, 0);
        assert_eq!(s.vec, vec![]);
    }

    #[test]
    fn new_char() {
        let s = CString::new_char('c');

        assert_eq!(s.size, 1);
        assert_eq!(s.vec, vec!['c']);
    }

    #[test]
    fn new_str() {
        let s = CString::new_str("Foo");

        assert_eq!(s.size, 3);
        assert_eq!(s.vec, vec!['F','o', 'o']);
    }

    #[test]
    fn new_vec() {
        let s = CString::new_vec(&"Foo".chars().collect::<Vec<char>>());

        assert_eq!(s.size, 3);
        assert_eq!(s.vec, vec!['F','o', 'o']);
    }

    #[test]
    fn new_string() {
        let s1 = CString::new_str("Foo");
        let s2 = CString::new_string(&s1);

        assert_eq!(s1.size, 3);
        assert_eq!(s1.vec, vec!['F','o', 'o']);

        assert_eq!(s2.size, 3);
        assert_eq!(s2.vec, vec!['F','o', 'o']);
    }

    #[test]
    fn assign() {
        let s1 = CString::new_str("Foo");
        let mut s2 = CString::new_str("Hello");

        assert_eq!(s2.size, 5);
        assert_eq!(s2.vec, vec!['H','e','l','l','o']);

        s2.assign(&s1);

        assert_eq!(s2.size, 3);
        assert_eq!(s2.vec, vec!['F','o', 'o']);
    }

    #[test]
    fn index() {
        let s = CString::new_str("Foo");

        assert_eq!(s[0], 'F');
        assert_eq!(s[1], 'o');
        assert_eq!(s[2], 'o');

        let panic_check = std::panic::catch_unwind(|| s[3]);
        assert!(panic_check.is_err());
    }

    #[test]
    #[should_panic(expected = "CString index out of bounds: the size is 3 but the index is 3! The index must be smaller the the size")]
    fn index_mut() {
        let mut s = CString::new_str("Foo");
        s[0] = 'B';
        s[1] = 'a';
        s[2] = 'r';

        assert_eq!(s[0], 'B');
        assert_eq!(s[1], 'a');
        assert_eq!(s[2], 'r');

        // sollte ein panic verursachen
        s[3] = 'A';
    }

    #[test]
    fn add_assign() {
        let mut s1 = CString::new_str("Hello,");
        let s2 = CString::new_str(" World");

        s1+=&s2;

        assert_eq!(s1.size, 12);
        assert_eq!(&s1.vec, &"Hello, World".to_string().chars().collect::<Vec<char>>());
    }

    #[test]
    fn display() {
        let s = CString::new_str("Hello, World");

        assert_eq!(format!("{}", &s) , "Hello, World");
    }

}