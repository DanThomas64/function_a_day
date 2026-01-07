use function_a_day::*;

fn main() {
    static STR: &str = "The quick brown fox jumped stupidly";

    let result = str_center(STR, 80);
    println!("{result}");

    let result = str_split_max(STR, 12);
    println!("{:?}", result);

    let result = longest_str_length(result);
    println!("{:?}", result);

    let result = str_multiline_box(STR, 80, true, None);
    println!("{}", result);
    let result = str_multiline_box(STR, 20, false, None);
    println!("{}", result);
    let result = str_multiline_box(STR, 20, false, Some(StrMultilineStyles::Modern.get_style()));
    println!("{}", result);
    let result = str_multiline_box(STR, 45, true, Some(StrMultilineStyles::Filled.get_style()));
    println!("{}", result);
    let result = str_multiline_box(
        STR,
        25,
        false,
        Some(StrMultilineStyles::Classic.get_style()),
    );
    println!("{}", result);
    let result = str_multiline_box(
        STR,
        35,
        true,
        Some(
            StrMultilineStyles::Custom(StrMultilineStyle::new(
                vec!['8', 'D', '8', 'D', '=', ' '],
                true,
                false,
                ' ',
            ))
            .get_style(),
        ),
    );
    println!("{}", result);
}
