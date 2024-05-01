defmodule Misc do
  @moduledoc """
  This module contains miscellaneous function to test
  """

  @doc """
  120

  ## Examples

      iex> Misc.factorial(5)
      120

  """
  def  factorial(n),      do: factorial(n, 1)
  defp factorial(1, acc), do: acc
  defp factorial(n, acc), do: factorial(n-1, n*acc)
    
  # LC 2788. Split Strings by Separator
  @spec split_words_by_separator(words :: [String.t], separator :: char) :: [String.t]
  def split_words_by_separator(words, separator) do
    words 
        |> Enum.map(&(String.split(&1, <<separator>>)))
        |> Enum.concat()
        |> Enum.filter(&(&1 != ""))
  end
  
  # LC 2828. Check if a String Is an Acronym of Words
  @spec is_acronym(words :: [String.t], s :: String.t) :: boolean
  def is_acronym(words, s),  do: acronym?(words, s)

  defp acronym?([<<ch, _::bytes>> | tl], <<ch1, rest::bytes>>), do: ch == ch1 && acronym?(tl, rest)
  defp acronym?([], <<>>), do: true
  defp acronym?(_, _), do: false
    
end
