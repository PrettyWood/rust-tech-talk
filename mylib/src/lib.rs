/// Swap the first and the last words of a given sentence
/// ```
/// use mylib::swap_first_last_words;
///
/// assert_eq!(swap_first_last_words(""), "");
/// assert_eq!(swap_first_last_words("pika"), "pika");
/// assert_eq!(swap_first_last_words("pika chu"), "chu pika");
/// assert_eq!(swap_first_last_words("Pikachu really loves Sasha"), "Sasha really loves Pikachu");
/// ```
pub fn swap_first_last_words<T: AsRef<str>>(sentence: T) -> String {
    let mut words: Vec<&str> = sentence.as_ref().split(' ').collect();
    words.swap(0, words.len() - 1);
    words.join(" ")
}
