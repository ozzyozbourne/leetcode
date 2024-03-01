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
    
    
end
