// Package hamming provides functions to calculate the hamming distance
package hamming

import "errors"

// Distance calculates the hamming distance between two strings
func Distance(a, b string) (int, error) {
	ar, br := []rune(a), []rune(b)

	if len(ar) != len(br) {
		return 0, errors.New("lengths must be the same")
	}

	var count int
	for i := range ar {
		if ar[i] != br[i] {
			count++
		}
	}
	return count, nil
}
