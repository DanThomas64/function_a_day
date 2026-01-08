package main

import "fmt"

func main() {
	str_center("Test string", 80)
}

/*
    Date:
        07/01/2026
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
func str_center(str string, term_width int) string {

	fmt.Println(str)

	pre_len := term_width / 2 - len(str) / 2

	runeSlice := []rune(str)

	out := make([]rune, term_width, term_width)

	for i := range out {
		out[i] = ' '
	}

	for i := pre_len; i < pre_len + len(str); i++ {
		out[i] = runeSlice[i - pre_len]
	}

	out_str := string(out)
	
	return out_str
}
