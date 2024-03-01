defmodule MiscTest do
  use ExUnit.Case
  doctest Misc

  test "tail recursive factorial" do
    assert Misc.factorial(5) == 120
    assert Misc.factorial(4) == 24
    assert Misc.factorial(3) == 6
  end
end
