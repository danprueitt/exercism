#include "difference_of_squares.h"

namespace difference_of_squares {
    int square_of_sum(int num) {
        return pow((num * (num + 1)) / 2, 2);
    }

    int sum_of_squares(int num) {
        return (num * (num + 1) * ((num * 2) + 1)) / 6;
    }

    int difference(int num) {
        int sum, square_sum;

        sum = (num * (num + 1)) / 2;
        square_sum = (num * (num + 1) * ((num * 2) + 1)) / 6;

        return (sum * sum) - square_sum;
    }
}  // namespace difference_of_squares
