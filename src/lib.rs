pub fn get_input_file() -> std::fs::File {
    let file = std::env::args().nth(1).expect("expected one argument");
    std::fs::File::open(file).expect("Cannot open file with aoc input")
}