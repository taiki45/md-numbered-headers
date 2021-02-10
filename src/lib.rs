#[macro_use]
extern crate lazy_static;
use regex::Regex;

pub fn process(content: &str) -> String {
    let mut heading_number = new_heading_number();
    let mut buffer = String::new();
    let mut is_code_block = false;

    for line in content.lines() {
        if line.starts_with("```") {
            is_code_block = !is_code_block;
        }

        let mut out = if !is_code_block && line.starts_with("##") {
            let s = cleanup(line);
            add_number_string(s, &mut heading_number)
        } else {
            line.to_string()
        };

        out.push('\n');
        buffer.push_str(&out);
    }

    buffer
}

pub fn process_cleanup(content: &str) -> String {
    let mut buffer = String::new();
    let mut is_code_block = false;

    for line in content.lines() {
        if line.starts_with("```") {
            is_code_block = !is_code_block;
        }

        let mut out = if !is_code_block && line.starts_with("##") {
            cleanup(line)
        } else {
            line.to_string()
        };

        out.push('\n');
        buffer.push_str(&out);
    }

    buffer
}

fn add_number_string(line: String, heading_number: &mut HeadingNumber) -> String {
    let depth = line.chars().take_while(|&v| v == '#').count();
    let mut out = line;
    // Skip if depth is gt 6.
    if depth < 6 {
        // Starts with heading depth 2 which is defined as level 1.
        let level = depth - 1;
        heading_number.count_up(level);
        let v = format!(" {}.", heading_number.as_string());
        out.insert_str(depth, &v);
    }
    out
}

fn cleanup(line: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((\d+\.)+\s?)").unwrap();
    }
    RE.replacen(&line, 1, "").to_string()
}

struct HeadingNumber {
    level_count: Vec<usize>,
}

fn new_heading_number() -> HeadingNumber {
    HeadingNumber {
        level_count: vec![0, 0, 0, 0],
    }
}

impl HeadingNumber {
    fn as_string(&self) -> String {
        self.level_count
            .iter()
            .filter(|&&v| v >= 1)
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(".")
    }

    fn count_up(&mut self, level: usize) {
        assert!(level != 0 && level <= 4);

        self.fill_blank(level);
        self.reset(level);
        self.level_count[level - 1] += 1;
    }

    /*
     * Given [0, 0, 0], level=2, fill the 1th element.
     */
    fn fill_blank(&mut self, level: usize) {
        let index = level - 1;
        let res = self.level_count.iter().position(|&v| v == 0);
        match res {
            Some(blank_pos) if blank_pos < index => {
                for blank_level in (blank_pos + 1)..level {
                    self.count_up(blank_level);
                }
            }
            _ => (),
        }
    }

    /*
     * Given [0, 0, 0], level=2, reset the 3th-to-last elements.
     */
    fn reset(&mut self, level: usize) {
        for i in level..self.level_count.len() {
            self.level_count[i] = 0;
        }
    }
}
