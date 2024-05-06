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


  defmodule LC_2788_SplitStringsbySeparator do
    @spec split_words_by_separator(words :: [String.t], separator :: char) :: [String.t]
    def split_words_by_separator(words, separator) do
      words 
      |> Enum.map(&(String.split(&1, <<separator>>)))
      |> Enum.concat()
      |> Enum.filter(&(&1 != ""))
    end
  end


  defmodule LC_2828_CheckifAStringIsAnAcronymofWords do
    @spec is_acronym(words :: [String.t], s :: String.t) :: boolean
    def is_acronym(words, s),  do: acronym?(words, s)

    defp acronym?([<<ch, _::bytes>> | tl], <<ch1, rest::bytes>>), do: ch == ch1 && acronym?(tl, rest)
    defp acronym?([], <<>>), do: true
    defp acronym?(_, _), do: false
  end


  defmodule LC_2864_MaximumOddBinaryNumber do
    @spec maximum_odd_binary_number(s :: String.t) :: String.t
    def maximum_odd_binary_number(s), do: solve(s, "", "")

    defp solve(<<?0, rest::binary>>, ones, zeros), do: solve(rest, ones, <<?0, zeros::binary>>)
    defp solve(<<?1, rest::binary>>, ones, zeros), do: solve(rest, <<?1, ones::binary>>, zeros)
    defp solve("", <<_, ones::binary>>, zeros),    do: ones <> zeros <> <<?1>>
  end


  defmodule LC_2798_NumberofEmployeesWhoMettheTarget do
    @spec number_of_employees_who_met_target(hours :: [integer], target :: integer) :: integer
    def number_of_employees_who_met_target(hours, target), do: Enum.count(hours, &(&1 >= target))
  end


  defmodule LC_3005_CountElementsWithMaximumFrequency do
    @spec max_frequency_elements(nums :: [integer]) :: integer
    def max_frequency_elements(nums) do
      Enum.frequencies(nums)
      |> Map.values()
      |> Enum.frequencies()
      |> Enum.max()
      |> then(fn {k, v} -> k * v end)
    end
  end


  defmodule LC_58_LengthofLastWord do
    @spec length_of_last_word(s :: String.t) :: integer
    def length_of_last_word(s), do: solve(s, true, 0)

    defp solve(<<?\s, tail::binary>>, _, ans), do: solve(tail, true, ans)
    defp solve(<<_, tail::binary>>, true, _), do: solve(tail, false, 1)
    defp solve(<<_, tail::binary>>, false, ans), do: solve(tail, false, ans + 1)
    defp solve("", _, ans), do: ans
  
  end


  defmodule LC_392_IsSubsequence do
    @spec is_subsequence(s :: String.t, t :: String.t) :: boolean
    def is_subsequence("", _), do: true
    def is_subsequence(_, ""), do: false
    def is_subsequence(<<ch, s::binary>>, <<ch, t::binary>>), do: is_subsequence(s, t)
    def is_subsequence(s, <<_, t::binary>>), do: is_subsequence(s, t)
  end


  defmodule LC_2839_CheckIfStringsCanBeMadeEqualWithOperationsI do
    @spec can_be_equal(s1 :: String.t, s2 :: String.t) :: boolean
    def can_be_equal(<<a1, a2, a3, a4>>, <<b1, b2, b3, b4>>) do
      Enum.sort([a1, a3]) == Enum.sort([b1, b3]) &&
      Enum.sort([a2, a4]) == Enum.sort([b2, b4])
    end
  end


end
