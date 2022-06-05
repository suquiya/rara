pub fn pwgen(length: usize, number: usize, use_string: String) {}

pub mod strList {
    pub fn get_alphabets() -> &str {
        return "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }

    pub fn get_symbols() -> &str {
        return "@=-?!#$%&'()~^|/\\_<>,.:;+[]{}\"";
    }

    pub fn get_numbers() -> &str {
        return "0123456789";
    }
}
