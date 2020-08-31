use test::*;
use std::collections::HashMap;

fn main() {
    // Testfälle
    let test_cases = vec![
        TestCase::new("Ha::ll::o".to_string(), "::".to_string(), "o".to_string()),
        TestCase::new("Ha::ll::o".to_string(), ":".to_string(), "o".to_string()),
        TestCase::new("Ha::ll::o".to_string(), "o".to_string(), "".to_string()),
        TestCase::new("Hallo".to_string(), "ll".to_string(), "o".to_string()),
        TestCase::new("Hallo, Welt".to_string(), ",".to_string(), " Welt".to_string()),
        TestCase::new("Hallo, Welt".to_string(), "::".to_string(), "Hallo, Welt".to_string())];

    // Dictionary welches die unterschiedlichen extract funktionen beinhaltet
    // Die Keys stellen den funktionsnamen dar
    // Die Values sind die zu testende funktion, dargestellt mit closures (Lambdas)
    let mut functions: HashMap<&str, for<'a> fn(&'a str, &str) -> &'a str> = std::collections::HashMap::new();
    functions.insert("extract_c", extract_c);
    functions.insert("extract_rust", extract_rust);

    // Iteration über functions mit Key/Value tupel
    for (name, function) in &functions {
        println!("Testing function {}", name);
        let mut errors = 0;
        // Iteration über die Testfälle
        for case in &test_cases {
            let res = run_test(case, function);
            if let TestResult::Failed = res {
                errors += 1;
            }
        }
        println!("Found {} errors while testing function {}\n",errors, name);
    }
}

// Diese extract function ist an C code angelegt, da sie char iteratoren verwendet welche den char* aus C ähneln
// Vereinfacht ausgedrückt iteriert diese funktion über jeden char von text und überprüft dabei ob die nachfolgenden chars dem pattern entsprechen.
// Da das kopieren der Parameter nicht benötigt wird und diese funktion keine ownership übernehmen sollte/muss,
// werden die Parameter als &str dargestellt
// Zurückgegeben wird ein slice des "text" parameters, hierfür muss die Lebenszeit 'a angegeben werden
fn extract_c<'a>(text: &'a str, pattern: &str) -> &'a str
{
    // Stellt die position in der äußeren iteration über text dar
    let mut text_position = text.chars();
    // Die Position ab der der suffix beginnt, zu begin ist dies text
    let mut suffix_position = text_position.clone();

    // Iteration über text bis text_position.next() ein None zurückgibt (dann wurde durch ganz text iteriert)
    while let Some(current_text_char) = text_position.next() {
        // Zeiger auf pattern, wird bei jeder äußeren iteration auf den anfang gesetzt
        let mut pattern_pos = pattern.chars();
        // Kopie von text_position damit im inneren weiter über text iteriert werden kann ohne text_position zu verändern
        let mut inner_text_position = text_position.clone();
        // Speichert ob ab dieser text_position als nächstes pattern vorkommt
        let mut text_equal = true;

        // Überprüft ob der char von text_position und der char in pattern identisch sind
        if pattern_pos.next() == Some(current_text_char) {
            // Iteriert über pattern
            while let Some(pattern_char) = pattern_pos.next() {
                // Vergleicht den aktuellen char von pattern mit dem char der inneren text iteration,
                // wenn diese ungleich sind, ist pattern nicht in der direkten folge text_position enthalten
                if inner_text_position.next() != Some(pattern_char) {
                    text_equal = false;
                    break;
                }
            }

            if text_equal {
                // Setzt die suffix position auf den ersten char(oder None) nach dem pattern
                suffix_position = inner_text_position.clone();
            }
        }
    }

    // Rückgabe von suffix_position als &str. RUST erkennt return values anhand des fehlenden ;
    suffix_position.as_str()
}

// Diese Funktion verwendet standard Rust Funktionen von str
fn extract_rust<'a>(text: &'a str, pattern: &str) -> &'a str {
    // split() teilt text bei jeden vorkommen von pattern auf und gibt einen Iterator zurück
    // collect() wandelt den Iterator in eine Vec um
    // Von diesem vector wird dann das letzte element zurückgegeben
    text.split(pattern).collect::<Vec<&str>>().last().unwrap()
}

mod test {

    // Struct welches eine Testfall beschreibt mit Eingabe und erwarteter Ausgabe
    // Das Struct übernimmt ownership der Felder
    pub struct TestCase {
        pub input: String,
        pub pattern: String,
        pub expected: String
    }
 
    impl TestCase {
        // Konstruktor für TestCase
        pub fn new(input: String, pattern: String, expected: String) -> TestCase {
            TestCase {
                input, pattern, expected
            }
        }
    }

    // Einfacher Enum
    pub enum TestResult {
        Passed,
        Failed
    }

    // Ein Template T wird benötigt um das Closure zu beschreiben
    pub fn run_test<T>(test_case: &TestCase, extract_function: T) -> TestResult
    where T: for<'a> Fn(&'a str, &str) -> &'a str {
        let output = extract_function(&test_case.input, &test_case.pattern);
        let test_passed = output == test_case.expected;
        println!("Test with input '{}' and pattern '{}' returned '{}' (expected output '{}'). The test {}!",
                 test_case.input, test_case.pattern, output, test_case.expected,
                 if test_passed {"passed"} else {"failed"});
        
        if test_passed {TestResult::Passed } else {TestResult::Failed }
    }
}
