pub fn process(content: &str) -> String {
    let mut heading_number = new_heading_number();
    let mut buffer = String::new();
    let mut is_code_block = false;

    for line in content.lines() {
        if line.starts_with("```") {
            if is_code_block {
                is_code_block = false;
            } else {
                is_code_block = true;
            }
        }

        let mut out = line.to_string();
        // TODO: remove existing heading number
        if !is_code_block && line.starts_with("##") {
            let depth = line.chars().take_while(|&v| v == '#').count();

            // Skip if depth is gt 6.
            if depth < 6 {
                // Starts with heading depth 2 which is defined as level 1.
                let level = depth - 1;
                heading_number.count_up(level);
                let v = format!(" {}.", heading_number.as_string());
                out.insert_str(depth, &v);
            }
        }

        let s = format!("{}\n", out);
        buffer.push_str(&s);
    }

    buffer
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
