use morse::standard::morse;

#[test]
fn test_encode_sos() {
    let plaintext = String::from("sos");
    let result = morse::encode(plaintext);
    assert!(!result.is_error());
    assert_eq!(result.value, "... --- ...")
}

#[test]
fn test_encode_three_words() {
    let plaintext = String::from("test some words");
    let result = morse::encode(plaintext);
    assert!(!result.is_error());
    assert_eq!(result.value, "- . ... - / ... --- -- . / .-- --- .-. -.. ...")
}

#[test]
fn test_encode_empty() {
    let plaintext = String::new();
    let result = morse::encode(plaintext);
    assert!(!result.is_error());
    assert!(result.value.is_empty())
}

#[test]
fn test_invalid_char_gives_err() {
    let plaintext = String::from("test s^mething");
    let res = morse::encode(plaintext);
    assert!(res.errors.contains(&String::from("invalid character '^' at position (word 2, char 2)")))
}