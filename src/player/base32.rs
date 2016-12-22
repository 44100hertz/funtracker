/// Base32 Has the following valid chars:
/// 0 1 2 3 4 5 6 7
/// 8 9 A B C D E F
/// G H J K L M N P
/// Q R T U V W X Y

pub fn char_to_base32(c: char) -> char {
    match c {
        '0' | 'O' | 'o' => '0',
        '1' | 'I' => '1',
        '2' | 'Z' => '2',
        '5' | 'S' => '5',
        '0'...'9' | 'A'...'Z' => c,
        'a'...'z' => c.to_uppercase().next().unwrap(),
        _ => '.',
    }
}
