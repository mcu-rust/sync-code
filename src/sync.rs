use regex::Regex;
use std::fs;
use std::path::PathBuf;

pub struct Sync {
    dest: PathBuf,
    src: PathBuf,
}

impl Sync {
    pub fn new(dest: PathBuf, src: PathBuf) -> Self {
        Sync { dest, src }
    }

    pub fn sync(&self) {
        let mut changed = false;
        let src = fs::read_to_string(&self.src).unwrap();
        let mut output = fs::read_to_string(&self.dest).unwrap();

        let re = Regex::new(r"// \$sync .+\n").unwrap();
        for mark in re.find_iter(&src).map(|c| c.as_str()) {
            let (_, code1, _) = get_marked_code(&src, mark);
            let (before, code2, after) = get_marked_code(&output, mark);
            if !code1.is_empty() && !code2.is_empty() && code1 != code2 {
                changed = true;
                output = before.to_string() + code1 + after;
            }
        }

        if changed {
            println!("Sync: {:?}", self.dest);
            fs::write(&self.dest, output).unwrap();
        }
    }
}

fn get_marked_code<'a>(code: &'a str, mark: &str) -> (&'a str, &'a str, &'a str) {
    let i = code.find(mark);
    if let Some(i) = i {
        let before = &code[..i];
        let rest = &code[i..];
        if let Some(i) = rest[1..].find("// $sync") {
            let after = &rest[i..];
            let code = &rest[..i];
            return (before, code, after);
        }
    }

    ("", "", "")
}
