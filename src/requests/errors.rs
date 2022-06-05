pub fn get_error(function: &str, symbols: &Vec<String>) -> String {
    format!("unable to get {} for {}", function, symbols.join(","))
}
