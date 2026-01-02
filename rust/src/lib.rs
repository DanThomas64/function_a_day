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
    println!("{t_len}");
    let mut out = Vec::new();
    let mut p_split_i = 0;
    let mut p_ws_i = 0;
    for (i, c) in text.chars().enumerate() {
        println!("For index {i} we have character {c} p_split_i is {p_split_i} and p_ws_i is {p_ws_i}");
        if c.is_whitespace() {
            p_ws_i = i;
        }
        if  i - p_split_i > max_width || i - p_ws_i > max_width {
            println!("We got into the if statement");

            out.push(text[p_split_i..p_ws_i].trim().to_string());
            println!("{text}");
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
        str_multiline_box
    Function Description:
        Creates a box around a given line of text.
        The box will be made up of line chars by default but will be customizable
        with an arg. 
        The box will will default to a ratio of 1/3/1 with one line top and 
        bottom of full character lines.
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
// pub fn str_multiline_box(
//         text: &str,
//         term_width: u16,
//         decorators: Option<Vec<char>>,
//         centered: Option<bool>,
//         fill: Option<bool>
//     ) {
//     let term_blk_size = term_width / 5;
//     let text_blk_size = term_blk_size * 3 + term_width % 5;
//     let trimmed = text.trim();
//     let mut out = "".to_string();
//     if let Some(d) = decorators {
//         if let Some(c) = d.get(0){
//         out.push(*c)
//         }
//     };
//     for _ in 2..term_width {
//         out.push(' ');
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
}
