use huturs_core::*;

#[test]
pub fn test_empty() {
    assert_eq!(is_empty(""), true);
    assert_eq!(is_empty(" "), false);
}

#[test]
pub fn test_blank_string() {
    assert_eq!(is_blank("a"), false);
    assert_eq!(is_blank(""), true);
    assert_eq!(is_blank(" "), true);
    assert_eq!(is_blank("\t"), true);
}


#[test]
pub fn test_to_uppercase() {
    assert_eq!(to_uppercase("HELLO"), "HELLO");
    assert_eq!(to_uppercase("hello"), "HELLO");
    assert_eq!(to_uppercase("123"), "123");
    assert_eq!(to_uppercase("123abcAbZ123"), "123ABCABZ123");
    assert_eq!(to_uppercase(" "), " ");
    assert_eq!(to_uppercase(""), "");
}


#[test]
pub fn test_to_lowercase() {
    assert_eq!(to_lowercase("HELLO"), "hello");
    assert_eq!(to_lowercase("hello"), "hello");
    assert_eq!(to_lowercase("123"), "123");
    assert_eq!(to_lowercase("123abcAbZ123"), "123abcabz123");
    assert_eq!(to_lowercase(" "), " ");
    assert_eq!(to_lowercase(""), "");
}

#[test]
pub fn test_trim(){
    assert_eq!(trim(""), "");
    assert_eq!(trim(" "), "");
    assert_eq!(trim("     "), "");
    assert_eq!(trim("          "), "");
}

#[test]
pub fn test_trim_start(){
    assert_eq!(trim_start(""), "");
    assert_eq!(trim_start(" "), "");
    assert_eq!(trim_start("     1"), "1");
    assert_eq!(trim_start("          1 "), "1 ");
}

#[test]
pub fn test_trim_end(){
    assert_eq!(trim_end(""), "");
    assert_eq!(trim_end(" "), "");
    assert_eq!(trim_end("1    "), "1");
    assert_eq!(trim_end(" 1      "), " 1");
}

#[test]
pub  fn test_reverse(){
    assert_eq!(reverse(" "), " ");
    assert_eq!(reverse("  "), "  ");
    assert_eq!(reverse(""), "");
    assert_eq!(reverse("ab"), "ba");
    assert_eq!(reverse("abc"), "cba");
    assert_eq!(reverse("abc123"), "321cba");
    assert_eq!(reverse("abc123abc"), "cba321cba");
}
