defmodule Obscura.Window do
  use GenServer

  def start_link(options) do
    GenServer.start_link(__MODULE__, options, name: __MODULE__)
  end

  def put_pixel(x, y, rgb) do
    GenServer.cast(__MODULE__, {:put_pixel, x, y, rgb})
  end

  def put_image() do
    GenServer.cast(__MODULE__, :put_image)
  end

  @impl true
  def init([width, height]) do
    Obscura.Window.X11.create(width, height)
    {:ok, %{}}
  end

  @impl true
  def terminate(_reason, _state) do
    Obscura.Window.X11.destroy()
  end

  @impl true
  def handle_cast({:put_pixel, x, y, rgb}, _state) do
    Obscura.Window.X11.put_pixel(x, y, rgb)
    {:noreply, %{}}
  end

  @impl true
  def handle_cast(:put_image, _state) do
    Obscura.Window.X11.put_image()
    {:noreply, %{}}
  end
end
