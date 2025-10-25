// Package raindrops contains functions to complete the raindrop problem
package raindrops

import "strconv"

// Convert converts a number into a string that contains
// raindrop sounds corresponding to certain potential factors
func Convert(i int) (out string) {
	if i%3 == 0 {
		out += "Pling"
	}
	if i%5 == 0 {
		out += "Plang"
	}
	if i%7 == 0 {
		out += "Plong"
	}
	if out == "" {
		out += strconv.Itoa(i)
	}
	return
}
