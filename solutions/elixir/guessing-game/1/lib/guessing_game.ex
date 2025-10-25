defmodule GuessingGame do
  def compare(secret_number, guess \\ :no_guess), do: compare_numbers(secret_number, guess)

  defp compare_numbers(_, :no_guess), do: "Make a guess"
  defp compare_numbers(secret_number, guess) when secret_number == guess, do: "Correct"
  defp compare_numbers(secret_number, guess) when secret_number == guess, do: "Correct"
  defp compare_numbers(secret_number, guess) when abs(secret_number - guess) == 1, do: "So close"
  defp compare_numbers(secret_number, guess) when secret_number > guess, do: "Too low"
  defp compare_numbers(secret_number, guess) when secret_number < guess, do: "Too high"
end
