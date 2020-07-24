use std::str::Chars;
use std::string::String;
use test_functions::TestCase;

fn main() {
    let test_data = vec![
        TestCase::new("Hallo", "ollaH"),
        TestCase::new("Welt", "tleW"),
        TestCase::new("Foo", "ooF"),
        TestCase::new("","")];

    // Die test funktionen für reverse, reverse_rec and reverse_rust. reverse verwendet einen closure um von &str zu Chars zu konvertieren
    let test_functions: Vec<(&str, fn(&str)-> String)> =
        vec![("reverse", |word: &str| reverse(word.chars())),
             ("reverse_rec", part_b::reverse_rec_str),
             ("reverse_rust", reverse_rust)];

    for (name, func) in &test_functions {
        let errors = test_functions::test_function(&test_data, name, func );
        println!("Function '{}' detected {} errors!\n", name, errors);
    }
}

// reverse nach c art.
// Zuerst wird die größe des inputs ermittelt, anschließend wird ein Vec<char> mit der entsprechend größe initialisiert
// und von hinten nach vorne aufgefüllt.
fn reverse(mut text_pointer: Chars) -> String {
    let mut reverse_string;
    let mut size = 0;
    // da .next() einen iterator konsumiert, muss eine kopie erstellt werden damit zweimal über den input iteriert werden kann
    let mut test_pointer_copy = text_pointer.clone();

    // iteration zum erkennen der länge
    while let Some(_) = test_pointer_copy.next() {
        size +=1;
    }

    // initialisierung des output vectors mit der erkannten größe
    reverse_string = vec!['\0'; size];

    // erneutes iterieren über den input, jeder char wird an der letzten nicht verwendeten position von reverse_string gesetzt
    while let Some(x) = text_pointer.next() {
        reverse_string[size-1] = x;
        size -=1;
    }

    // konvertierung von reverse_string zu einen String, reverse_string wird dabei konsumiert
    reverse_string.into_iter().collect::<String>()
}

fn reverse_rust(input: &str) -> String {
    // einige iteratoren bieten eine rev() methode an welche die richtung auf rechts -> links umstellt,
    // beginnend am ende des iterators und endend an der aktuellen position.
    // Diese neue iterator wird dann in einen String konvertiert
    input.chars().rev().collect::<String>()
}

mod part_b {
    use std::string::String;
    use std::slice::IterMut;
    use std::slice::Iter;

    // Gleiche längenberechnung wie in reverse()
    fn length(mut pointer: Iter<char>) -> usize {
        let mut len = 0;
        while let Some(_) = pointer.next() {
            len +=1;
        }
        len
    }

    // iteriert über input_pointer n mal und fügt weißt dabei jeden char der aktuellen position von output_pointer zu.
    // IterMut sind iteratoren welche das zuweisen ermöglichen.
    // unwrap() entpackt ein Option<T> auf T (hier is T =  &mut char), sollte die Option jedoch kein wert enthalten führt das zur einem panic (exception),
    // das bedeutet das input_pointer und output_pointer mindestens n elemente lang sein müssen
    pub fn copy(mut input_pointer: Iter<char>, n: usize, mut output_pointer: IterMut<char>) {
        for _ in 0..n {
            *output_pointer.next().unwrap() = *input_pointer.next().unwrap();
        }
    }

    // Erstellt einen neuen Vec<char> mit der länge von input +1,
    // kopiert den inhalt von input an den anfang und setzt das letzte zeichen auf c bevor der Vec zurückgeben wird
    pub fn put_back(input: Iter<char>, c: char ) -> std::vec::Vec<char> {
        let length = length(input.clone());
        // hier wird ein Vec<char> verwendet da bei diesen die char werte leichter veränderbar sind.
        // Rust erlaubt nicht das einfache bearbeiten der char werte eines Strings oder str
        let mut output = vec!['\0'; length +1];
        copy(input, length, output.iter_mut());
        output[length] = c;
        output
    }

    // Wrapper für reverse_rec welcher die eingabe als &str erlaubt und einen String zurückgibt
    pub fn reverse_rec_str(input: &str) -> String {
        reverse_rec(input.chars().collect::<std::vec::Vec<char>>().iter()).into_iter().collect()
    }


    // Prüft und merkt den aktuelle wert von input, anschließend wird reverse_rec() recursive aufgerufen
    // mit der nächsten position von input.
    // An dieser rückgabe wird der aktuelle char am ende angefügt.
    // Wenn input leer ist wird eine leerer Vec zurückgegeben. Dies ist die endbedienung der Rekursiven funktion
    pub fn reverse_rec(mut input: Iter<char>) -> std::vec::Vec<char> {
        if let Some(c) = input.next() {
            let reverse = reverse_rec(input);
            put_back(reverse.iter(), *c)
        }
        else {
            std::vec::Vec::new()
        }
    }
}

// Testkonstrukt analog zu Aufgabe1
mod test_functions {
    use super::*;

    fn print_result(input: &str, expected: &str, output: &str, passed: bool) {
        println!("Test with input '{}' returned '{}' (expected '{}'). The test {}",
        input, output, expected, if passed {"passed"} else {"failed"});
    }


    // TestCase speichert hier die werte als &str, dafür muss eine lifetime angegeben werden die garantiert
    // das die felder mindestens solange leben wie die Instanz von TestCase
    pub struct TestCase<'a> {
        pub word: &'a str,
        pub expected: &'a str
    }

    impl<'a> TestCase<'a> {
        pub fn new(word: &'a str, expected: &'a str) -> TestCase<'a> {
            TestCase{word,expected}
        }
    }

    pub fn test_function<T>(test_data: &std::vec::Vec<TestCase>, function_name: &str, func: T) -> usize
    where T: Fn(&str) -> String{
        let mut failures = 0;

        println!("Testing function '{}'", function_name);
        for test_case in test_data.iter() {
            let output = func(test_case.word);
            let test_passed = output == test_case.expected;
            print_result(test_case.word, test_case.expected, &output, test_passed);
            if !test_passed {
                failures+=1;
            }
        }

        failures
    }
}
