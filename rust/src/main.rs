use function_a_day::*;

fn main() {
    static STR: &str = "The quick brown fox jumped stupidly";

    let result = str_center(STR, 80);
    println!("{result}");

    let result = str_split_max(STR, 12);
    println!("{:?}", result);

    let result = longest_str_length(result);
    println!("{:?}", result);
}
