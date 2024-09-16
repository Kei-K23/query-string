// Internal helper functions

pub fn remove_char(word: &str, filter: &str) -> String {
    word.chars().filter(|&c| !filter.contains(c)).collect()
}
