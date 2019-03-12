use regex::Regex;


pub fn split(s: &str, delims: Vec<&str>) -> Vec<String> {
    let mut result = vec![];
    let mut current_token = "".to_string();
    let mut number_of_quotes = 0;
    for ch in s.chars() {
        current_token.push(ch);
        if ch == '"' {
            number_of_quotes += 1;
        } else if number_of_quotes % 2 == 0 {
            for delim in delims.clone() {
                if current_token.contains(delim) {
                    for _ in 0..delim.len() {
                        current_token.pop();
                    }
                    result.push(current_token.clone());
                    result.push(delim.to_string());
                    current_token = "".to_string();
                    break;
                }
            }
        }
    }

    result.push(current_token.clone());

    result.retain(|x| x.clone() != "".to_string());
    result
}


pub fn trim(v: Vec<String>) -> Vec<String> {
    let mut result = vec![];
    for item in v {
        result.push(item.trim().to_string());
    }
    result
}

pub fn is_number(s: &str) -> bool {
    let re = Regex::new(r"^-?[0-9]+\.?([0-9]+)?$").unwrap();
    re.is_match(s)
}

pub fn is_identifier(s: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    re.is_match(s)
}

pub fn is_string(s: &str) -> bool {
    let re = Regex::new("^\"[^\"\\\\]*(\\\\.[^\"\\\\]*)*\"$").unwrap();
    re.is_match(s)
}


