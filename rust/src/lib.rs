/*
    Date:
        01/01/2026
    Function Name:
        str_center
    Function Description:
        Centers a string in the middle of a terminal.
    What prompted this function:
        I apparently rewrite this for every project. Might be useful to just 
        have ready to go.
    Inputs:
        text - A String of text.
        termin_width - The width in which we want to center the text.
    Output:
        out - a string of lenth = term_width with the text centered within it.
*/
/// Formats a String with the supplied text centered. 
///
/// # Examples
///
/// ```
/// let term_width = 20;
/// let result = function_a_day::str_center("foo", term_width);
///
/// assert_eq!(result, "         foo        ");
/// ```
pub fn str_center(text: &str, term_width: u16) -> String {
    let text_len = text.to_string().len();
    let pre_len = term_width / 2 - text_len as u16 / 2;
    let mut out = "".to_string();
    for _ in 1..=pre_len {
        out.push(' ')
    };
    out.push_str(text);
    let int_len = out.len() as u16;
    for _ in int_len..term_width {
        out.push(' ')
    };
    out
}
/*
    Date:
        02/01/2026
    Function Name:
        str_split_max
    Function Description:
        Will split a string into as many lines as needed at a reasonable spot 
        dependant on the length of the string and a given value.
    What prompted this function:
        I think this is a better sized function than the one I was trying to write
        for today and it would be super useful in the tomorrows function.
    Inputs:
        text - A String of text.
        max_split_width - The width in which we want the text to fit into.
    Output:
        out - a vec of strings not exceeding the max_split_width.
*/
/// Reduces a long string down to a number of strings that do no exceed a 
/// provided length/width.
///
/// # Examples
///
/// ```
/// let max_width = 12;
/// let result = function_a_day::str_split_max(
///     "The meaning of life is equal to 42", 
///     max_width
/// );
///
/// assert_eq!(result, vec!("The meaning", "of life is", "equal to 42"));
/// ```
pub fn str_split_max( text: &str, max_width: usize) -> Vec<String>{
    let t_len = text.len();
    let mut out = Vec::new();
    let mut p_split_i = 0;
    let mut p_ws_i = 0;
    for (i, c) in text.chars().enumerate() {
        if c.is_whitespace() {
            p_ws_i = i;
        }
        if  i - p_split_i > max_width || i - p_ws_i > max_width {

            out.push(text[p_split_i..p_ws_i].trim().to_string());
            p_split_i = p_ws_i;
        }
    }
    out.push(text[p_split_i..t_len].trim().to_string());
    out
}
/*
    Date:
        03/01/2026
    Function Name:
        longest_str_length
    Function Description:
        Get the longest string in a vec of strings and then give the length
    What prompted this function:
        Need to get something done today as clearly I am not going to complete
        the multibox today. 
    Inputs:
        vec - A vec of Strings.
    Output:
        out - Int of the longest string
*/
/// Reduces a long string down to a number of strings that do no exceed a 
/// provided length/width.
///
/// # Examples
///
/// ```
/// let max_width = 12;
/// let vec = function_a_day::str_split_max(
///     "The meaning of life is equal to 42", 
///     max_width
/// );
/// let result = function_a_day::longest_str_length(vec);
///
/// assert_eq!(result, 11);
/// ```
pub fn longest_str_length(strings: Vec<String>) -> usize {
    let mut out = 0;
    for s in strings {
        let len = s.len();
        if len > out {
            out = len;
        };
    }
    out
}

/*
    Date:
        04/01/2026
    Function Name:
        str_multiline_box
    Function Description:
        Creates a box around a given line of text.
        The box will be made up of line chars by default but will be customizable
        with an arg. 
        An optionally centered box will default to a ratio of 1/3/1.
        An optionally filled box will have one line top and bottom of fill 
        character lines.
    What prompted this function:
        I apparently rewrite this for every project. Might be useful to just 
        have ready to go.
    Inputs:
        text - A String of text.
        termin_width - The width in which we want to center the text.
    Output:
        out - a string of lenth = term_width with the text centered within it.
*/
/// Formats a String with the supplied text centered. 
///
/// # Examples
///
/// ```
/// let term_width = 20;
/// let result = function_a_day::str_center("foo", term_width);
///
/// assert_eq!(result, "         foo        ");
/// ```
// pub enum StrMultilineStyles{
//     Classic,
//     Modern,
//     Filled,
//     Custom(StrMultilineStyle)
// }
// pub struct StrMultilineStyle {
//     decorators: Vec<char>,
//     text_centered: bool,
//     filled: bool,
//     filled_char: char,
// }
// #[derive(Default)]
// impl StrMultilineStyles {
//     fn get_style(self) -> StrMultilineStyle {
//         match self {
//             #[default]
//             StrMultilineStyles::Classic => StrMultilineStyle {
//                 decorators: vec!{'/', '\\', '\\', '/','=', '|'},
//                 text_centered: true,
//                 filled: false,
//                 filled_char: ' '
//             },
//             StrMultilineStyles::Modern => StrMultilineStyle {
//                 decorators: vec!{'*', '*', '*', '*','-', '|'},
//                 text_centered: true,
//                 filled: false,
//                 filled_char: ' '
//             },
//             StrMultilineStyles::Filled => StrMultilineStyle {
//                 decorators: vec!{'&', '&', '&', '&','&', '&'},
//                 text_centered: false,
//                 filled: true,
//                 filled_char: '#'
//             },
//             StrMultilineStyles::Custom(data) => data
//     }
// }
// pub fn str_multiline_box(
//         text: &str,
//         term_width: u16,
//         centered: bool,
//         style: Option<StrMultilineStyle>
//     ) {
//     let term_blk_size = term_width / 5;
//     let text_blk_size = term_blk_size * 3 + term_width % 5;
//     let mut formatted_text = text.str_split_max(text, text_blk_size - 2 );
//     let mut out = "".to_string();
//     let opts = if Some(style) {
//         style.get_style();
//     } else {
//         style.default()
//     };
//     if opts.centered {
//         formatted_text = formatted_text
//             .iter()
//             .map( |l| str_center(l, term_width))
//             .collect();
//     }
//     // If filled then fill first line with fill characters
//     if opts.filled {
//         for _ in 0..term_width {
//             out.push(opts.filled_char);
//             out.push_str("\\\n");
//         };
//     };
//     // Top line of the box
//     if centered {
//         for _ in 1..term_blk_size {
//             out.push(opts.filled_char);
//         };
//     };
//     // Top left corner decorator
//     out.push(opts.decorators.get(0));
//     // Top line of the box
//     if centered {
//         for _ in start..term_width {
//             out.push(opts.decorators.get(4));
//         };
//     };
//     out.push(opts.decorators.get(4));
//     if centered {
//         for _ in 1..term_blk_size {
//             out.push(opts.filled_char);
//         };
//     };
//     // If centered then first 1/5 gets filled with fill char plus one space
//     // else is first 1 char is filled and then rest is text and post fill.
//     for line in formatted_text {
//         if centered {
//             for _ in 1..term_blk_size {
//                 out.push(opts.filled_char);
//             };
//         };
//         out.push(opts.decorators.get(4));
//         out.push(' ');
//         out.push_str(line);
//         out.push(' ');
//         out.push(opts.decorators.get(4));
//     };
//
//     out.push(opts.decorators.get(0));
//     for _ in 2..text_blk_size {
//         out.push();
//     };
//     out.push_str("\\\n");
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jan_1_2026() {
        let result = str_center("foobar", 20);
        assert_eq!(result, "       foobar       ");
        let result = str_center("foo", 20);
        assert_eq!(result, "         foo        ");
        let result = str_center("bar", 21);
        assert_eq!(result, "         bar         ");
    }
    #[test]
    fn jan_2_2026() {
        let result = str_split_max("The quick brown fox jumped stupidly", 12);
        assert_eq!(result, vec!("The quick", "brown fox", "jumped", "stupidly"));
        let result = str_split_max("Take me home, country road, to the place I belong!!!", 16);
        assert_eq!(result, vec!("Take me home,", "country road, to", "the place I", "belong!!!"));
    }
    #[test]
    fn jan_3_2026() {
        let vec = str_split_max("The quick brown fox jumped stupidly", 12);
        let result = longest_str_length(vec);
        assert_eq!(result, 9);
    }
}
