static INPUTS_DIR: &str = "inputs";

pub fn load_inputs(day: usize) -> Vec<String> {
    let input = std::fs::read_to_string(format!("{}/{}", INPUTS_DIR, day)).expect("Can't open/read input file");
    input.lines().map(std::string::ToString::to_string).collect()
}

// pub fn group_lines_split_by_empty_line(lines: &[String]) -> Vec<&[String]> {
//     lines.split(|line| line.is_empty()).collect()
// }
