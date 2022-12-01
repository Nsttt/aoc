fn main() {
    let input = include_str!("./input.txt");

    let result1 = input.split("\n\n").map(|x| {
        return x.split('\n').flat_map(str::parse::<usize>).sum::<usize>();
    });

    let mut result2: Vec<usize> = input
        .split("\n\n")
        .map(|x| {
            return x.split('\n').flat_map(str::parse::<usize>).sum();
        })
        .collect();

    result2.sort_by(|a, b| b.cmp(a));

    println!("Result 1: {}", result1.max().unwrap());
    println!("Result 2: {}", result2.iter().take(3).sum::<usize>());
}
