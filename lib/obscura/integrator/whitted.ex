defmodule Obscura.Integrator.Whitted do
  defstruct [:scene, :tile]

  def new(scene, options \\ []) do
    tile = Keyword.get(options, :tile, 64)

    %__MODULE__{scene: scene, tile: tile}
  end

  def radiance(_integrator, _xy) do
    0x000000FF
  end
end

defimpl Obscura.Integrator, for: Obscura.Integrator.Whitted do
  import Obscura.Kernel

  alias Obscura.Integrator.Whitted

  def join(_integrator, pixels) do
    Enum.each(pixels, fn [[x, y], rgb] -> Obscura.Window.put_pixel(x, y, rgb) end)
    Obscura.Window.put_image()
  end

  def render(integrator, [min_x, min_y, max_x, max_y]) do
    for y <- Range.new(min_y, max_y - 1),
        x <- Range.new(min_x, max_x - 1),
        into: [],
        do: [[x, y], Whitted.radiance(integrator, {x, y})]
  end

  def split(integrator) do
    resolution = Obscura.Point2.new(1920, 1080)
    step = integrator.tile

    for y <- Range.new(0, resolution.y - 1, step),
        x <- Range.new(0, resolution.x - 1, step),
        into: [] do
      [x, y, clamp(x + step, 0, resolution.x - 1), clamp(y + step, 0, resolution.y - 1)]
    end
  end
end
