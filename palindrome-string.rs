fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>();
    let s = s.to_lowercase();

    let reversed = s.chars().rev().collect::<String>();

    s == reversed
}

fn main() {
    let input = "A man, a plan, a canal, Panama";
    
    if is_palindrome(input) {
        println!("The string is a palindrome.");
    } else {
        println!("The string is not a palindrome.");
    }
}
