defmodule Obscura do
  alias Obscura.{Bounds2, Point2}

  def render(scene, options \\ []) do
    broker = Keyword.get(options, :broker, "amqp://guest:guest@localhost")
    step = Keyword.get(options, :step, 4)

    {:ok, connection} = AMQP.Connection.open(broker)
    {:ok, channel} = AMQP.Channel.open(connection)

    {:ok, %{message_count: 0}} = AMQP.Queue.declare(channel, "obscura.rendering.queue")

    {:ok, %{queue: reply_to}} = AMQP.Queue.declare(channel, "", exclusive: true)
    AMQP.Basic.consume(channel, reply_to, self(), no_ack: true)

    bounds = Bounds2.new(Point2.new(), Point2.new(16, 16))

    for tile <- tiles(bounds, step) do
      json = JSON.encode!(scene: scene, bounds: {tile.min.x, tile.min.y, tile.max.x, tile.max.y})
      Task.async(fn -> basic_publish(json, channel, reply_to) end)
    end
    |> Enum.map(&Task.await(&1, :infinity))
    |> Enum.map(&basic_deliver/1)
  end

  defp basic_deliver(correlation_id) do
    receive do
      {:basic_deliver, payload, %{correlation_id: ^correlation_id}} ->
        JSON.decode!(payload)
    end
  end

  defp basic_publish(json, channel, reply_to) do
    correlation_id = :erlang.unique_integer() |> :erlang.integer_to_binary() |> Base.encode64()

    AMQP.Basic.publish(channel, "", "obscura.rendering.queue", json,
      reply_to: reply_to,
      correlation_id: correlation_id
    )

    correlation_id
  end

  defp tiles(bounds, step) do
    for y <- Range.new(bounds.min.y, bounds.max.y - 1, step),
        x <- Range.new(bounds.min.x, bounds.max.x - 1, step),
        into: [] do
      min = Point2.new(x, y)
      max = Point2.new(x + step, y + step)
      Bounds2.new(min, max)
    end
  end
end

defmodule Obscura.Worker do
  alias Obscura.{Point2, Scene}

  def start(options \\ []) do
    broker = Keyword.get(options, :broker, "amqp://guest:guest@localhost")

    {:ok, connection} = AMQP.Connection.open(broker)
    {:ok, channel} = AMQP.Channel.open(connection)

    AMQP.Queue.declare(channel, "obscura.rendering.queue")
    AMQP.Basic.qos(channel, prefetch_count: 1)
    AMQP.Basic.consume(channel, "obscura.rendering.queue")

    basic_deliver(channel)
  end

  defp basic_deliver(channel) do
    receive do
      {:basic_deliver, payload, metadata} ->
        json = JSON.decode!(payload) |> handle |> JSON.encode!()

        AMQP.Basic.publish(channel, "", metadata.reply_to, "#{json}",
          correlation_id: metadata.correlation_id
        )

        AMQP.Basic.ack(channel, metadata.delivery_tag)

        basic_deliver(channel)
    end
  end

  defp handle(request) do
    _scene = Scene.parse!(request["scene"])

    for pixel <- pixels(request["bounds"]) do
      Task.async(fn -> [xy: {pixel.x, pixel.y}, rgb: {0, 0, 0}] end)
    end
    |> Enum.map(&Task.await/1)
  end

  defp pixels([min_x, min_y, max_x, max_y] = _tile) do
    for y <- Range.new(min_y, max_y - 1),
        x <- Range.new(min_x, max_x - 1),
        into: [],
        do: Point2.new(x, y)
  end
end
