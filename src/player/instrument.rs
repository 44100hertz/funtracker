use player::song::Chan;
use player::command;

pub struct Inst {
    init: String,
    note: String,
    off: String,
}

impl Inst {
    pub fn apply_init(&mut self, chan: &mut Chan) {
        apply(&self.init, chan)
    }

    pub fn apply_note(&mut self, chan: &mut Chan) {
        apply(&self.note, chan)
    }

    pub fn apply_off(&mut self, chan: &mut Chan) {
        apply(&self.off, chan)
    }
}

fn apply(part: &str, chan: &mut Chan) {
    for field in part.split_whitespace() {
        command::set(field, chan)
    }
}

pub fn parse_all(block: String) -> Vec<Inst> {
    block.lines()
        .map(parse_line)
        .collect::<Vec<Inst>>()
}

pub fn parse_line(line: &str) -> Inst {
    fn ss(slice: Option<&str>) -> String {
        match slice {
            Some(s) => s.to_owned(),
            None => "".to_owned(),
        }
    }

    let mut parts = line.split("|");
    Inst {
        init: ss(parts.next()),
        note: ss(parts.next()),
        off: ss(parts.next()),
    }
}

