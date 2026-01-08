package main

import (
	"testing"
)

// jan_7_2026 Test
func str_center_test_1(t *testing.T){

    got := str_center("foobar", 20)
    want := "       foobar       "

    if got != want {
        t.Errorf("got %q, wanted %q", got, want)
    }

}
func str_center_test_2(t *testing.T){
    got := str_center("foo", 20)
    want := "         foo        "

    if got != want {
        t.Errorf("got %q, wanted %q", got, want)
    }
}

func str_center_test_3(t *testing.T){
    got := str_center("bar", 21)
    want := "         bar         "

    if got != want {
        t.Errorf("got %q, wanted %q", got, want)
    }
}
