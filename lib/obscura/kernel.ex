defmodule Obscura.Kernel do
  @spec clamp(number, number, number) :: number
  def clamp(n, min, _max) when n < min, do: min
  def clamp(n, _min, max) when n > max, do: max
  def clamp(n, _min, _max), do: n

  @spec lerp(number, number, number) :: number
  def lerp(t, a, b), do: a * (1 - t) + b * t
end
