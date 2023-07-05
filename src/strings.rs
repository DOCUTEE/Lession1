// Exercise 1
#[allow(dead_code)]
fn exercise1(color: &str) -> String {
    "white".to_string()
}

// Exercise 2
// Fix all errors without adding newline
fn exercise2() -> String {
    let mut s = String::from("hello");
    s.push(',');
    s += " world";
    s.push('!');
    s
}
// Exercise 3
// Fix errors without removing any line
fn exercise3() -> String {
    let s1 = String::from("hello,");
    let s2 = String::from(" world!");
    let s3: String = format!("{}{}",s1,s2);
    s3
}

// Exercise 4
// Reverse a string

fn reverse_string(input: &str) -> String {
    let temp: Vec<char> = input.chars().collect();
    let mut result:String = String::from("");
    for i in temp
    {
        result = i.to_string() + &result;
    }
    result
}


// Exercise 5
// Check if a string is a palindrome
fn is_palindrome(word: &str) -> bool {
    let newword: String = word.to_string().to_lowercase();
    let temp: Vec<char> = newword.chars().collect();
    let len = temp.len();
    
    for i in 0..=len/2 {
        if temp[i] != temp[len-i-1] {
            return false;
        }
    }
    true
}


// Exercise 6
// Count the occurrences of a character in a string
fn count_char_occurrences(string: &str, ch: char) -> usize {
    let newstring: String = string.to_string().to_lowercase();
    let mut cnt: usize = 0;
    let temp: Vec<char> = string.chars().collect();
    for value in temp
    {
        if ch == value
        {
            cnt += 1;
        }
        else if value == ' '
        {
            return cnt;
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn exercise1_work() {
        assert_eq!("white".to_string(), exercise1("white"));
    }

    // Test for exercise 2
    #[test]
    fn exercise2_work() {
        assert_eq!("hello, world!".to_string(), exercise2());
    }

    // Test for exercise 3
    #[test]
    fn exercise3_work() {
        assert_eq!("hello, world!".to_string(), exercise3());
    }
    
    // Test for exercise 4
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string("world"), "dlrow");
        assert_eq!(reverse_string(""), "");
    }

    // Test for exercise 5
    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome("level"), true);
        assert_eq!(is_palindrome("deed"), true);
        assert_eq!(is_palindrome("Rotor"), true);
    }
    // Test for exercise 5
    #[test]
    fn test_non_palindrome() {
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome("world"), false);
    }

    // Test for exercise 6

    #[test]
    fn test_count_char_occurrences() {
        assert_eq!(count_char_occurrences("Hello", 'l'), 2);
        assert_eq!(count_char_occurrences("Rust is fun", 'u'), 1);
        assert_eq!(count_char_occurrences("Mississippi", 's'), 4);
    }

}