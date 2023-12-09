#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintMode {
    None,
    Debug,
}

pub fn run<InputType, SolutionType>(
    inputs: &[(&str, PrintMode)],
    solutions: &[fn(&InputType) -> SolutionType],
) where
    InputType: From<std::fs::File> + std::fmt::Debug,
    SolutionType: std::fmt::Debug,
{
    use std::time::Instant;

    for (input_path, print_mode) in inputs {
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

        if *print_mode == PrintMode::Debug {
            println!("{input:?}");
        }

        for (i, solution) in solutions.iter().enumerate() {
            let time = Instant::now();
            let answer = solution(&input);
            let time = time.elapsed();
            println!(
                "- Part {} answer: '{answer:?}' (took {} μs)",
                i + 1,
                time.as_micros()
            );
        }

        println!("");
    }
}
