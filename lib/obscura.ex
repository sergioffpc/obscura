defmodule Obscura do
  def render(uri, options \\ []) do
    broker = Keyword.get(options, :broker, "amqp://guest:guest@localhost")

    scene = Obscura.Scene.parse!(uri)
    Obscura.Window.start_link([1920, 1080])

    integrator = Obscura.Integrator.Whitted.new(scene, options)
    Obscura.Integrator.Consumer.start_link([integrator, broker])

    Obscura.Integrator.Dispatcher.dispatch(integrator, broker)
  end
end

defmodule Obscura.Worker do
  def start(uri, options \\ []) do
    broker = Keyword.get(options, :broker, "amqp://guest:guest@localhost")

    scene = Obscura.Scene.parse!(uri)
    integrator = Obscura.Integrator.Whitted.new(scene)
    Obscura.Integrator.Consumer.start_link([integrator, broker])
  end
end
