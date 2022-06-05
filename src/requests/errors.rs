/// Returns error message for function with symbols concatenated.
pub fn get_error(action: &str, symbols: &Vec<String>) -> String {
    format!("unable to get {} for {}", action, symbols.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_message_for_single_symbol() {
        let action = "action";
        let symbols = &vec!["AAPL".to_string()];

        let actual = get_error(action, symbols);

        let expected = "unable to get action for AAPL".to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_error_message_for_multiple_symbols() {
        let action = "action";
        let symbols = &vec!["AAPL".to_string(), "TSLA".to_string()];

        let actual = get_error(action, symbols);

        let expected = "unable to get action for AAPL, TSLA".to_string();
        assert_eq!(expected, actual);
    }
}
