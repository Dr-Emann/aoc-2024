mod day1;

trait Day {
    type Parsed<'a>;

    fn generator(input: &str) -> Self::Parsed<'_>;

    fn part1(input: &Self::Parsed<'_>) -> impl std::fmt::Display;
    fn part2(input: Self::Parsed<'_>) -> impl std::fmt::Display;
}

#[derive(Debug, Clone)]
struct Timing {
    gen: std::time::Duration,
    part1: std::time::Duration,
    part2: std::time::Duration,
}

#[derive(Debug, Clone)]
struct DayResults {
    timing: Timing,
    part1: String,
    part2: String,
}

#[inline]
fn time<O>(f: impl FnOnce() -> O) -> (O, std::time::Duration) {
    let start = std::time::Instant::now();
    let res = f();
    let elapsed = start.elapsed();
    (res, elapsed)
}

fn run_day<D: Day>(input: &str) -> DayResults {
    let (parsed, gen_time) = time(|| D::generator(input));

    let (part1, part1_time) = time(|| D::part1(&parsed));
    let part1_str = part1.to_string();
    drop(part1);

    let (part2, part2_time) = time(|| D::part2(parsed));
    let part2_str = part2.to_string();
    drop(part2);

    DayResults {
        timing: Timing {
            gen: gen_time,
            part1: part1_time,
            part2: part2_time,
        },
        part1: part1_str,
        part2: part2_str,
    }
}

fn main() {
    let input = std::fs::read_to_string("input/2024/day1.txt").expect("Failed to read input.txt");
    let results = run_day::<day1::Day1>(&input);
    println!("Day 1: {:#?}", results);
}
