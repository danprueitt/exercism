package collatzconjecture

import "errors"

func CollatzConjecture(num int) (count int, err error) {
	if num < 1 {
		return count, errors.New("num must be positive int")
	}

	for num != 1 {
		if num%2 == 0 {
			num = num / 2
		} else {
			num = num * 3 + 1
		}
		count++
	}
	return
}
