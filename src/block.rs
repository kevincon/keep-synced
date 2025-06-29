pub struct Block {
    pub line_range: std::ops::Range<usize>,
    pub lines: Vec<String>,
}

pub fn find_blocks(contents: &str) -> Vec<Block> {
    let mut blocks = Vec::new();

    for (line_number, line) in contents.lines().enumerate() {}

    blocks
}
