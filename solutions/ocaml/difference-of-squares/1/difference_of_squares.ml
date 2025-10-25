let square_of_sum n =
  let rec sum acc n = if n = 0 then acc else sum (acc + n) (n - 1) in
  let sum = sum 0 n in
  sum * sum

let sum_of_squares n =
  let rec sum acc n = if n = 0 then acc else sum (acc + (n * n)) (n - 1) in
  sum 0 n

let difference_of_squares n = square_of_sum n - sum_of_squares n
