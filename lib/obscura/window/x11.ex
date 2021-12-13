defmodule Obscura.Window.X11 do
  @on_load {:init, 0}

  app = Mix.Project.config()[:app]

  def init do
    path = :filename.join(:code.priv_dir(unquote(app)), 'x11')
    :ok = :erlang.load_nif(path, 0)
  end

  def create(_width, _height) do
    exit(:nif_library_not_loaded)
  end

  def destroy() do
    exit(:nif_library_not_loaded)
  end

  def put_pixel(_x, _y, _rgb) do
    exit(:nif_library_not_loaded)
  end

  def put_image() do
    exit(:nif_library_not_loaded)
  end
end
