#[macro_use]
extern crate lazy_static;
use regex::Regex;

pub struct Opt {
    pub cleanup_only: bool,
    pub start_depth: usize,
    pub end_depth: usize,
    pub reset_with_higher_depth: bool,
}

pub fn process(content: &str, opt: Opt) -> String {
    let mut heading_number = new_heading_number(opt.end_depth - opt.start_depth);
    let mut buffer = String::new();
    let mut is_code_block = false;

    for line in content.lines() {
        if line.starts_with("```") {
            is_code_block = !is_code_block;
        }

        let mut out = line.to_string();
        if !is_code_block && out.starts_with("#") {
            out = cleanup(out);

            if !opt.cleanup_only {
                add_number_string(&mut out, &mut heading_number, &opt);
            }
        }
        out.push('\n');
        buffer.push_str(&out);
    }

    buffer
}

fn cleanup(line: String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((\d+\.)+\s?)").unwrap();
    }
    RE.replacen(&line, 1, "").to_string()
}

fn add_number_string(line: &mut String, heading_number: &mut HeadingNumber, opt: &Opt) {
    let depth = line.chars().take_while(|&v| v == '#').count();
    // reset
    if opt.reset_with_higher_depth == true && depth < opt.start_depth {
        heading_number.reset_all();
    }

    // count up and add the number string
    let level = depth - (opt.start_depth - 1);
    if 1 <= level && level <= (opt.end_depth - opt.start_depth) {
        heading_number.count_up(level);
        let v = format!(" {}.", heading_number.as_string());
        line.insert_str(depth, &v);
    }
}

struct HeadingNumber {
    level_count: Vec<usize>,
}

fn new_heading_number(len: usize) -> HeadingNumber {
    HeadingNumber {
        level_count: vec![0; len],
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
        assert!(1 <= level && level <= self.level_count.len());

        self.fill_blank(level);
        self.reset(level);
        self.level_count[level - 1] += 1;
    }

    fn reset_all(&mut self) {
        self.reset(0);
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
