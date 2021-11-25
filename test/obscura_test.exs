defmodule ObscuraTest do
  use ExUnit.Case
  doctest Obscura

  test "greets the world" do
    assert Obscura.hello() == :world
  end
end
