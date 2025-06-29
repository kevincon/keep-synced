use anyhow::Result;
use std::ops::Range;

mod block;

pub struct Problem {
    /// The path of the file containing this problem.
    pub path: std::path::PathBuf,
    /// The range of lines for which this problem applies.
    pub lines: Range<usize>,
    /// A human-readable message describing the problem.
    pub message: String,
}

pub fn find_problems(
    filename: &std::path::PathBuf,
    contents: &str,
) -> Result<Vec<Problem>, std::io::Error> {
    let blocks = block::find_blocks(contents);

    let problems = Vec::new();

    Ok(problems)
}
