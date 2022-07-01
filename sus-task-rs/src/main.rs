const SCRIPTURE: &str = include_str!("../../Scripture.md");

fn main() {
    println!("START OF SCRIPTURE\n");

    for mut line in SCRIPTURE.lines() {
        line = line.trim_start_matches(|c: char| c.is_ascii_whitespace() || c == '#');
        println!("{line}")
    }
    println!("\nEND OF SCRIPTURE")
}
