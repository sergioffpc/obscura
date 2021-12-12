defmodule Obscura.TaskEmitter do
  def emit(uri, options \\ []) do
    broker = Keyword.get(options, :broker, "amqp://guest:guest@localhost")

    scene = Obscura.Scene.parse!(uri)
    integrator = Obscura.Integrator.Whitted.new(scene, options)
    Obscura.Integrator.Dispatcher.dispatch(integrator, broker)
  end
end

defmodule Obscura.TaskReceiver do
  def receive(uri, options \\ []) do
    broker = Keyword.get(options, :broker, "amqp://guest:guest@localhost")

    scene = Obscura.Scene.parse!(uri)
    integrator = Obscura.Integrator.Whitted.new(scene)
    Obscura.Integrator.Consumer.consume(integrator, broker)
  end
end
