defprotocol Obscura.Integrator do
  def join(integrator, pixels)

  def render(integrator, bounds)

  def split(integrator)
end

defmodule Obscura.Integrator.Dispatcher do
  alias Obscura.Integrator

  def dispatch(integrator, broker) do
    {:ok, connection} = AMQP.Connection.open(broker)
    {:ok, channel} = AMQP.Channel.open(connection)

    {:ok, %{message_count: 0}} = AMQP.Queue.declare(channel, "obscura.rendering.queue")
    {:ok, %{queue: reply_to}} = AMQP.Queue.declare(channel)

    for bounds <- Integrator.split(integrator) do
      Task.async(fn ->
        AMQP.Basic.consume(channel, reply_to, self(), no_ack: true)

        pixels =
          JSON.encode!(bounds)
          |> basic_publish(channel, reply_to)
          |> basic_deliver()
          |> JSON.decode!()

        Integrator.join(integrator, pixels)
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

defmodule Obscura.Integrator.Consumer do
  alias Obscura.Integrator

  def consume(integrator, broker) do
    {:ok, connection} = AMQP.Connection.open(broker)
    {:ok, channel} = AMQP.Channel.open(connection)

    AMQP.Queue.declare(channel, "obscura.rendering.queue")
    AMQP.Basic.qos(channel, prefetch_count: 1)
    AMQP.Basic.consume(channel, "obscura.rendering.queue")

    basic_deliver(channel, integrator)
  end

  defp basic_deliver(channel, integrator) do
    receive do
      {:basic_deliver, payload, metadata} ->
        json = Integrator.render(integrator, JSON.decode!(payload)) |> JSON.encode!()

        AMQP.Basic.publish(channel, "", metadata.reply_to, "#{json}",
          correlation_id: metadata.correlation_id
        )

        AMQP.Basic.ack(channel, metadata.delivery_tag)

        basic_deliver(channel, integrator)
    end
  end
end
