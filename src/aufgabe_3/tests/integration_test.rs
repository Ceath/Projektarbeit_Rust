#[cfg(test)]
// Integration-Tests der beide main() funktionen aus der aufgabenstellung
mod integration_tests {
    use aufgabe_3::custom_string::CString;

    #[test]
    fn test_example_1() {
        let mut s1 = CString::new_empty();
        let mut s2 = CString::new_str("Hello");
        let s3 = CString::new_string(&s2);

        s1 += &s2;
        s2.assign(&s3);

        assert_eq!(format!("{}", &s1), "Hello");
        assert_eq!(format!("{}", &s2), "Hello");
        assert_eq!(format!("{}", &s3), "Hello");
        assert_eq!(s2[2], 'l');

        assert_eq!(s1.size, 5);
        assert_eq!(s2.size, 5);
        assert_eq!(s3.size, 5);
    }

    #[test]
    fn test_example_2() {
        let s1 = CString::new_empty();
        let s2 = CString::new_char('H');

        assert_eq!(format!("{}", &s1), "");
        assert_eq!(format!("{}", &s2), "H");

        assert_eq!(s1.size, 0);
        assert_eq!(s2.size, 1);
    }
}