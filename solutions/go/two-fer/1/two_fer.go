package twofer

const (
	defaultMessage = "One for you, one for me."
	msgPrefix      = "One for "
	msgSuffix      = ", one for me."
)

func ShareWith(name string) string {
	if name == "" {
		return defaultMessage
	}
	return msgPrefix + name + msgSuffix
}
