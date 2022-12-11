use super::super::standard::morse;

#[test]
fn test_decode_sos() {
    let ciphertext = String::from("... --- ...");
    let plaintext = morse::decode(ciphertext).unwrap();
    assert_eq!(plaintext, "sos")
}

#[test]
fn test_decode_three_words() {
    let ciphertext = String::from("- . ... - / ... --- -- . / .-- --- .-. -.. ...");
    let plaintext = morse::decode(ciphertext).unwrap();
    assert_eq!(plaintext, "test some words")
}

#[test]
fn test_decode_empty_gives_empty() {
    let ciphertext = String::new();
    let plaintext = morse::decode(ciphertext).unwrap();
    assert!(plaintext.is_empty())
}