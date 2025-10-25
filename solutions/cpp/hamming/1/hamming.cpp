#include "hamming.h"

namespace hamming {

    int compute(const std::string &first_string, const std::string &second_string) {
        unsigned int str_len = first_string.length();

        if (str_len != second_string.length()) {
            throw std::domain_error("String lengths must be equal");
        }

        int count = 0;
        for (int i = 0; i < str_len; i++) {
            if (first_string[i] != second_string[i]) {
                count++;
            }
        }
        return count;
    }
}  // namespace hamming
