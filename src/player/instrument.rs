pub struct Inst {
    init: Vec<String>,
    note: Vec<String>,
    off: Vec<String>,
}

pub fn parse_all(block: String) -> Vec<Inst> {
    block.lines()
        .map(parse_line)
        .collect::<Vec<Inst>>()
}

pub fn parse_line(line: &str) -> Inst {
    let mut parts = line.split("|");
    Inst {
        init: split_instr_part(parts.next()),
        note: split_instr_part(parts.next()),
        off: split_instr_part(parts.next()),
    }
}

pub fn split_instr_part(part: Option<&str>) -> Vec<String> {
    match part {
        Some(p) => p.split_whitespace()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>(),
        None => Vec::new(),
    }
}
