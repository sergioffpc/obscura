defmodule Obscura.Integrator.Whitted do
  defstruct [:scene, :step]

  def new(scene, options \\ []) do
    step = Keyword.get(options, :step, 16)

    %__MODULE__{scene: scene, step: step}
  end

  def radiance(_integrator, _xy) do
    {0, 0, 0}
  end

  def write(_integrator, _xy, _rgb) do
  end
end

defimpl Obscura.Integrator, for: Obscura.Integrator.Whitted do
  alias Obscura.Integrator.Whitted

  def join(integrator, pixels) do
    Enum.each(pixels, fn [xy, rgb] -> Whitted.write(integrator, xy, rgb) end)
  end

  def render(integrator, [min_x, min_y, max_x, max_y]) do
    for y <- Range.new(min_y, max_y - 1),
        x <- Range.new(min_x, max_x - 1),
        into: [],
        do: [{x, y}, Whitted.radiance(integrator, {x, y})]
  end

  def split(integrator) do
    resolution = integrator.scene.camera.film.resolution
    step = integrator.step

    for y <- Range.new(0, resolution.y - 1, step),
        x <- Range.new(0, resolution.x - 1, step),
        into: [] do
      [x, y, x + step, y + step]
    end
  end
end
