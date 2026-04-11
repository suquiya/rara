use rand::RngExt;

/// Generates a list of random passwords with the given length and number of passwords and the given characters.
pub fn pwgen(length: usize, number: usize, use_chars: &[char]) -> Vec<String> {
    let mut rng = rand::rng();
    let use_chars_len = use_chars.len();
    let mut queue = Vec::with_capacity(number);
    for _ in 0..number {
        let password: String = (0..length)
            .map(|_| use_chars[rng.random_range(0..use_chars_len)])
            .collect();
        queue.push(password);
    }
    queue
}

/// Provides a list of characters for use in password generation.
pub mod str_list {
    /// Returns a string of lowercase and uppercase alphabetic characters.
    pub fn get_alphabets<'a>() -> &'a str {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    }

    /// Returns a string of symbols for use in password generation.
    pub fn get_symbols<'a>() -> &'a str {
        "@=-?!#$%&'()~^|/\\_<>,.:;+[]{}\""
    }

    /// Returns a string of numbers for use in password generation.
    pub fn get_numbers<'a>() -> &'a str {
        "0123456789"
    }
}
