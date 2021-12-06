fn parse_input<I: Iterator<Item = String>>(mut input_lines: I) -> [u64; 9] {
    let initial_population_line = input_lines
        .next()
        .expect("expected one line of comma-separated ages for the initial lanternfish population");

    initial_population_line
        .split(',')
        .map(|age_str| {
            age_str
                .parse::<usize>()
                .expect("expected comma separated unsigned integers")
        })
        .fold([0; 9], |mut population, lanternfish_age| {
            population[lanternfish_age] += 1;
            population
        })
}

fn simulate_lanternfish_population(initial_population: [u64; 9], num_days: usize) -> [u64; 9] {
    let mut current_population = initial_population;

    for _ in 0..num_days {
        let mut new_population = current_population;

        for age in (1..=8).rev() {
            new_population[age - 1] = current_population[age]
        }

        new_population[8] = current_population[0];
        new_population[6] += current_population[0];

        current_population = new_population;
    }

    current_population
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let initial_population = parse_input(input_lines);

    let final_population = simulate_lanternfish_population(initial_population, 80);

    final_population
        .iter()
        .copied()
        .map(|num| num as u64)
        .sum::<u64>()
        .to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let initial_population = parse_input(input_lines);

    let final_population = simulate_lanternfish_population(initial_population, 256);

    final_population
        .iter()
        .copied()
        .map(|num| num as u64)
        .sum::<u64>()
        .to_string()
}
