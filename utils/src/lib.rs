pub fn run<InputType, SolutionType>(
    input_paths: &[&str],
    solutions: &[fn(&InputType) -> SolutionType],
) where
    InputType: From<std::fs::File> + std::fmt::Display,
    SolutionType: std::fmt::Display,
{
    use std::time::Instant;

    for input_path in input_paths {
        let full_path = std::path::PathBuf::from("data/").join(input_path);
        let input_file =
            std::fs::File::open(full_path.clone()).expect("Failed to open the input file.");

        let input_time = Instant::now();
        let input = InputType::from(input_file);
        let input_time = input_time.elapsed();
        println!(
            "Read input from: {full_path:?} (took {} μs)",
            input_time.as_micros()
        );
        println!("{input}");

        for (i, solution) in solutions.iter().enumerate() {
            let time = Instant::now();
            let answer = solution(&input);
            let time = time.elapsed();
            println!(
                "- Part {} answer: '{answer}' (took {} μs)",
                i + 1,
                time.as_micros()
            );
        }
    }
}
