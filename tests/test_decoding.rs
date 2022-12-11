use morse::standard::morse;

#[test]
fn test_decode_sos() {
    let ciphertext = String::from("... --- ...");
    let result = morse::decode(ciphertext);
    assert!(!result.is_error());
    assert_eq!(result.value, "sos")
}

#[test]
fn test_decode_three_words() {
    let ciphertext = String::from("- . ... - / ... --- -- . / .-- --- .-. -.. ...");
    let result = morse::decode(ciphertext);
    assert!(!result.is_error());
    assert_eq!(result.value, "test some words")
}

#[test]
fn test_decode_empty_gives_empty() {
    let ciphertext = String::new();
    let result = morse::decode(ciphertext);
    assert!(!result.is_error());
    assert!(result.value.is_empty())
}

#[test]
fn test_decode_invalid_morse_seq_gives_err() {
    let ciphertext = String::from(".......... ---");
    let res = morse::decode(ciphertext);
    assert!(res.errors.contains(&String::from("invalid morse sequence [..........] at position (word 1, char 1)")))
}

#[test]
fn test_decode_invalid_morse_symbol_gives_err() {
    let cihpertext = String::from(".a. --- ...");
    let res = morse::decode(cihpertext);
    assert!(res.errors.contains(&String::from("parsing error: invalid symbol 'a' at position (word 1, char 1)")))
}