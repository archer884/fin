/// Finds possible descendants of the target prefix.
///
/// The main difference between this and most key/value prefix trees currently
/// available for Rust is that this is able to return multiple values for a single 
/// key.
pub fn get_descendants<'a>(target: &str, words: &'a [String]) -> Option<&'a [String]> {
    // The index returned by binary search will always be correct, 
    // whether it's returned as an error or not, so we'll just 
    // unwrap it and use it anyway in that case. Funnily enough,
    // it's not legal to call `.unwrap()`. Still.
    let idx = words
        .binary_search(&target.to_string())
        .unwrap_or_else(|e| e);

    match words[idx].starts_with(target) {
        false => None,
        true => Some(&words[idx..(idx + words[idx..]
                                  .iter()
                                  .take_while(|w| w.starts_with(target))
                                  .count())]),
    }
}

#[test]
fn can_get_descendants() {
    let dict = [
        "hellcat".to_string(),
        "hello".to_string(),
        "help".to_string(),
        "incite".to_string(),
        ];
    let matches = get_descendants("hel", &dict).unwrap();

    assert!(matches == &dict[..3]);
}
