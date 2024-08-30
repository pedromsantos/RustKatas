pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod reverse_string_tests {
    use super::*;

    #[test]
    fn reverse_hello() {
        let input = "hello";
        let output = reverse(input);
        let expected = "olleh";
        assert_eq!(output, expected);
    }

    #[test]
    fn reverse_string_with_punctuation() {
        let input = "hello, world!";
        let output = reverse(input);
        let expected = "!dlrow ,olleh";
        assert_eq!(output, expected);
    }

    #[test]
    fn reverse_empty_string() {
        let input = "";
        let output = reverse(input);
        let expected = "";
        assert_eq!(output, expected);
    }

    #[test]
    fn reverse_unicode() {
        let input = "🦀";
        let output = reverse(input);
        let expected = "🦀";
        assert_eq!(output, expected);
    }

    #[test]
    fn reverse_unicode_hello() {
        let input = "hello, 🦀!";
        let output = reverse(input);
        let expected = "!🦀 ,olleh";
        assert_eq!(output, expected);
    }
}
