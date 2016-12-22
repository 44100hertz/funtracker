/// Base32 Has the following valid chars:
/// 0 1 2 3 4 5 6 7
/// 8 9 A B C D E F
/// G H J K L M N P
/// Q R T U V W X Y

pub fn char_to_base32(c: char) -> char {
    match c {
        '0' | 'O' | 'o' => '0',
        '1' | 'I' | 'i' => '1',
        '2' | 'Z' | 'z' => '2',
        '5' | 'S' | 's' => '5',
        '0'...'9' | 'A'...'Z' => c,
        'a'...'z' => c.to_uppercase().next().unwrap(),
        _ => '.',
    }
}

pub fn string_to_base32(s: &str) -> String {
    let mut outstr = "".to_string();
    for c in s.chars() {
        let b32: char = char_to_base32(c);
        if b32 != '.' {outstr.push(b32);}
    };
    outstr
}
