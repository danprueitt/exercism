package acronym

import (
	"regexp"
	"strings"
)

// Abbreviate
func Abbreviate(s string) string {
	letterRegex := regexp.MustCompile("[^a-zA-Z0-9]+")
	s = strings.ToTitle(s)
	s = letterRegex.ReplaceAllString(s, " ")
	words := strings.Split(s, " ")

	var ab []string
	for _, word := range words {
		ab = append(ab, word[:1])
	}

	return strings.Join(ab, "")
}
