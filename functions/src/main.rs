fn main() {
    let a = last_char(String::from("Hello"));
    println!("The last char is: {}", a);
}

fn last_char(string: String) -> char {
    if string.is_empty() {
        return '!';
    }
    string.chars().next_back().unwrap()
}