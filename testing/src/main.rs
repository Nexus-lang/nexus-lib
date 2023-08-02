fn main() {
    let test = '你';
    println!("{}", is_utf8_and_not_ascii(test))
}

fn is_utf8_and_not_ascii(c: char) -> bool {
    let s = c.to_string();
    let bytes = s.as_bytes();

    // Check if the bytes form a valid UTF-8 string
    
    if !s.is_ascii() {
        if let Ok(_) = std::str::from_utf8(bytes) {
            return true;
        } else {
            return false;
        }
    }
    false
}
