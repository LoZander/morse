use super::super::standard::morse;

#[test]
fn test_encode_sos() {
    let plaintext = String::from("sos");
    let ciphertext = morse::encode(plaintext).unwrap();
    assert_eq!(ciphertext, "... --- ...")
}

#[test]
fn test_encode_three_words() {
    let plaintext = String::from("test some words");
    let ciphertext = morse::encode(plaintext).unwrap();
    assert_eq!(ciphertext, "- . ... - / ... --- -- . / .-- --- .-. -.. ...")
}

#[test]
fn test_encode_empty() {
    let plaintext = String::new();
    let ciphertext = morse::encode(plaintext).unwrap();
    assert!(ciphertext.is_empty())
}

#[test]
fn test_invalid_char_gives_err() {
    let plaintext = String::from("test s^mething");
    let res = morse::encode(plaintext).unwrap_err();
    assert_eq!(res, "invalid character '^' at position (word 2, char 2)")
}