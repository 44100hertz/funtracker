/// Base32 Has the following valid chars:
/// 0 1 2 3 4 5 6 7
/// 8 9 A B C D E F
/// G H J K L M N P
/// Q R T U V W X Y

pub fn char_to_base32(c: char) -> Option<char> {
    match c {
        '0' | 'O' | 'o' => Some('0'),
        '1' | 'I' | 'i' => Some('1'),
        '2' | 'Z' | 'z' => Some('2'),
        '5' | 'S' | 's' => Some('5'),
        '0'...'9' | 'A'...'Z' => Some(c),
        'a'...'z' => c.to_uppercase().next(),
        _ => None,
    }
}
