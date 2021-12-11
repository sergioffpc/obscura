defmodule Obscura do
  defstruct [:broker, :scene, :step]

  alias Obscura.{Consumer, Dispatcher, Scene}

  def new(uri, options \\ []) do
    scene = Scene.parse!(uri)

    %__MODULE__{
      broker: Keyword.get(options, :broker, "amqp://guest:guest@localhost"),
      scene: scene,
      step: Keyword.get(options, :step, 256)
    }
  end

  def dispatch(renderer) do
    Dispatcher.dispatch(renderer.broker, renderer)
  end

  def consume(renderer) do
    Consumer.consume(renderer.broker, renderer)
  end
end

defprotocol Obscura.Renderer do
  def aggregate(renderer, pixels)

  def integrate(renderer, tile)

  def shred(renderer)
end

defimpl Obscura.Renderer, for: Obscura do
  def aggregate(renderer, pixels) do
    Enum.each(pixels, fn [xy, rgb] -> put_pixel(renderer, xy, rgb) end)
  end

  def integrate(_renderer, [min_x, min_y, max_x, max_y]) do
    for y <- Range.new(min_y, max_y - 1),
        x <- Range.new(min_x, max_x - 1),
        into: [],
        do: [{x, y}, {0, 0, 0}]
  end

  def shred(renderer) do
    resolution = renderer.scene.camera.film.resolution
    step = renderer.step

    for y <- Range.new(0, resolution.y - 1, step),
        x <- Range.new(0, resolution.x - 1, step),
        into: [] do
      [x, y, x + step, y + step]
    end
  end

  defp put_pixel(_renderer, xy, rgb) do
    IO.puts("xy: #{xy}, rgb: #{rgb}")
  end
end

defmodule Obscura.Dispatcher do
  def dispatch(broker, renderer) do
    {:ok, connection} = AMQP.Connection.open(broker)
    {:ok, channel} = AMQP.Channel.open(connection)

    {:ok, %{message_count: 0}} = AMQP.Queue.declare(channel, "obscura.rendering.queue")
    {:ok, %{queue: reply_to}} = AMQP.Queue.declare(channel)

    for bounds <- renderer.shred(renderer) do
      Task.async(fn ->
        AMQP.Basic.consume(channel, reply_to, self(), no_ack: true)

        pixels =
          JSON.encode!(bounds)
          |> basic_publish(channel, reply_to)
          |> basic_deliver()
          |> JSON.decode!()

        renderer.aggregate(renderer, pixels)
      end)
    end
    |> Enum.map(&Task.await(&1, :infinity))

    AMQP.Connection.close(connection)
  end

  defp basic_deliver(correlation_id) do
    receive do
      {:basic_deliver, payload, %{correlation_id: ^correlation_id}} ->
        payload
    end
  end

  defp basic_publish(payload, channel, reply_to) do
    correlation_id = :erlang.unique_integer() |> :erlang.integer_to_binary() |> Base.encode64()

    AMQP.Basic.publish(channel, "", "obscura.rendering.queue", payload,
      correlation_id: correlation_id,
      reply_to: reply_to
    )

    correlation_id
  end
end

defmodule Obscura.Consumer do
  def consume(broker, renderer) do
    {:ok, connection} = AMQP.Connection.open(broker)
    {:ok, channel} = AMQP.Channel.open(connection)

    AMQP.Queue.declare(channel, "obscura.rendering.queue")
    AMQP.Basic.qos(channel, prefetch_count: 1)
    AMQP.Basic.consume(channel, "obscura.rendering.queue")

    basic_deliver(channel, renderer)

    AMQP.Connection.close(connection)
  end

  defp basic_deliver(channel, renderer) do
    receive do
      {:basic_deliver, payload, metadata} ->
        json = renderer.integrate(renderer, JSON.decode!(payload)) |> JSON.encode!()

        AMQP.Basic.publish(channel, "", metadata.reply_to, "#{json}",
          correlation_id: metadata.correlation_id
        )

        AMQP.Basic.ack(channel, metadata.delivery_tag)

        basic_deliver(channel, renderer)
    end
  end
end
