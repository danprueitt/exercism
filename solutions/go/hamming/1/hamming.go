// package hamming provides functions to calculate the hamming distance
package hamming

import "errors"

// Distance calculates the hamming distance between two strings
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return -1, errors.New("lengths must be the same")
	}

	var count int
	for i := range a {
		if a[i] != b[i] {
			count++
		}
	}
	return count, nil
}
