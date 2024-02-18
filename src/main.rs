mod runtime;

fn main() {
    let args = std::env::args();
    let file = args.into_iter().last().unwrap();
    let file_string = std::fs::read_to_string(file).unwrap();

    runtime::run_code(&file_string);
}
