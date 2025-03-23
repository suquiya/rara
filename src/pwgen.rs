use rand::Rng;

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

pub mod str_list {
    pub fn get_alphabets<'a>() -> &'a str {
        return "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }

    pub fn get_symbols<'a>() -> &'a str {
        return "@=-?!#$%&'()~^|/\\_<>,.:;+[]{}\"";
    }

    pub fn get_numbers<'a>() -> &'a str {
        return "0123456789";
    }
}
