package acronym

import (
	"regexp"
	"strings"
)

// Abbreviate a sentence and return the acronym
func Abbreviate(s string) (output string) {
	r := regexp.MustCompile(`[a-zA-Z']+`)
	words := r.FindAllString(s, -1)
	for _, words := range words {
		output += strings.ToUpper(words[:1])
	}
	return
}
