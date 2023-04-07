use ::morse::standard::morse::{MorseTranslater, MorseEncoder};

#[test]
fn test_encode_sos() {
    let translater = MorseTranslater::default();
    let plaintext = String::from("sos");
    let ciphertext = translater.encode(plaintext).unwrap();
    assert_eq!(ciphertext, "... --- ...")
}

#[test]
fn test_encode_three_words() {
    let translater = MorseTranslater::default();
    let plaintext = String::from("test some words");
    let ciphertext = translater.encode(plaintext).unwrap();
    assert_eq!(ciphertext, "- . ... - / ... --- -- . / .-- --- .-. -.. ...")
}

#[test]
fn test_encode_empty() {
    let translater = MorseTranslater::default();
    let plaintext = String::new();
    let ciphertext = translater.encode(plaintext).unwrap();
    assert!(ciphertext.is_empty())
}

#[test]
fn test_invalid_char_gives_err() {
    let translater = MorseTranslater::default();
    let plaintext = String::from("test s^mething");
    let res = translater.encode(plaintext).unwrap_err();
    assert_eq!(res, "invalid character '^' at position (word 2, char 2)")
}