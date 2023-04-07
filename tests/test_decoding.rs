use ::morse::standard::morse::{MorseTranslater, MorseDecoder};

#[test]
fn test_decode_sos() {
    let translater = MorseTranslater::default();
    let ciphertext = String::from("... --- ...");
    let plaintext = translater.decode(ciphertext).unwrap();
    assert_eq!(plaintext, "sos")
}

#[test]
fn test_decode_three_words() {
    let translater = MorseTranslater::default();
    let ciphertext = String::from("- . ... - / ... --- -- . / .-- --- .-. -.. ...");
    let plaintext = translater.decode(ciphertext).unwrap();
    assert_eq!(plaintext, "test some words")
}

#[test]
fn test_decode_empty_gives_empty() {
    let translater = MorseTranslater::default();
    let ciphertext = String::new();
    let plaintext = translater.decode(ciphertext).unwrap();
    assert!(plaintext.is_empty())
}

#[test]
fn test_decode_invalid_morse_seq_gives_err() {
    let translater = MorseTranslater::default();
    let ciphertext = String::from(".......... ---");
    let res = translater.decode(ciphertext);
    let error = res.expect_err("expected error");
    assert_eq!(error, "invalid morse sequence [..........] at position (word 1, char 1)")
}

#[test]
fn test_decode_invalid_morse_symbol_gives_err() {
    let translater = MorseTranslater::default();
    let cihpertext = String::from(".a. --- ...");
    let res = translater.decode(cihpertext);
    let error = res.expect_err("expected error");
    assert_eq!(error, "parsing error: invalid symbol 'a' at position (word 1, char 1)")
}