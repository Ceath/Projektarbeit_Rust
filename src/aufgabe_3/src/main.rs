use aufgabe_3::custom_string::CString;

// Testausgabe für CString.
// Hinweis CString befindet sich in lib.rs, da dies erlaubt unit-tests für das Struct zu schreiben
fn main() {
    let mut s1 = CString::new_empty();
    let mut s2 = CString::new_str("Hello");
    let s3 = CString::new_string(&s2);
    let s4 = CString::new_char('a');

    println!("s1:= '{}'", &s1);
    println!("s2:= '{}'", &s2);
    println!("s3:= '{}'", &s3);
    println!("s4:= '{}'\n", &s4);

    s1 += &s2;
    println!("s1+=s2:= '{}'(s1) and '{}'(s2)\n", &s1, &s2);

    s2.assign(&s3);
    println!("s2=s3:= '{}'(s2) and '{}'(s3)\n", &s2, &s3);

    println!("s2[2]:= '{}'\n", &s2[2]);

    s2[1] = 'a';
    println!("s2[1]='a':= '{}'\n", &s2);
}

