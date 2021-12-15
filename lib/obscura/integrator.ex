defprotocol Obscura.Integrator do
  def join(integrator, pixels)

  def render(integrator, bounds)

  def split(integrator)
end

defmodule Obscura.Integrator.Dispatcher do
  alias Obscura.Integrator

  @queue "obscura.rendering.queue"

  def dispatch(integrator, broker) do
    {:ok, connection} = AMQP.Connection.open(broker)
    {:ok, channel} = AMQP.Channel.open(connection)
    {:ok, %{}} = AMQP.Queue.declare(channel, @queue, auto_delete: true)
    {:ok, %{queue: reply_to}} = AMQP.Queue.declare(channel, "", auto_delete: true)
    AMQP.Basic.consume(channel, reply_to, self())

    for bounds <- Integrator.split(integrator) do
      Task.async(fn ->
        AMQP.Basic.publish(channel, "", @queue, JSON.encode!(bounds), reply_to: reply_to)
      end)
    end
    |> Enum.map(fn task ->
      Task.await(task, :infinity)

      receive do
        {:basic_deliver, payload, metadata} ->
          Integrator.join(integrator, JSON.decode!(payload))
          AMQP.Basic.ack(channel, metadata.delivery_tag)
      end
    end)

    AMQP.Connection.close(connection)
  end
end

defmodule Obscura.Integrator.Consumer do
  use GenServer

  alias Obscura.Integrator

  @queue "obscura.rendering.queue"

  def start_link(options) do
    GenServer.start_link(__MODULE__, options, name: __MODULE__)
  end

  @impl true
  def init([integrator, broker]) do
    {:ok, connection} = AMQP.Connection.open(broker)
    {:ok, channel} = AMQP.Channel.open(connection)
    {:ok, %{}} = AMQP.Queue.declare(channel, @queue, auto_delete: true)
    :ok = AMQP.Basic.qos(channel, prefetch_count: 1)
    {:ok, _consumer_tag} = AMQP.Basic.consume(channel, @queue)
    {:ok, %{channel: channel, integrator: integrator}}
  end

  @impl true
  def handle_info({:basic_consume_ok, %{}}, state) do
    {:noreply, state}
  end

  @impl true
  def handle_info({:basic_cancel, %{}}, state) do
    {:stop, :normal, state}
  end

  @impl true
  def handle_info({:basic_cancel_ok, %{}}, state) do
    {:noreply, state}
  end

  @impl true
  def handle_info({:basic_deliver, payload, metadata}, state) do
    try do
      response = Integrator.render(state.integrator, JSON.decode!(payload)) |> JSON.encode!()

      AMQP.Basic.publish(state.channel, "", metadata.reply_to, "#{response}",
        correlation_id: metadata.correlation_id
      )

      AMQP.Basic.ack(state.channel, metadata.delivery_tag)
    rescue
      _exception ->
        :ok =
          AMQP.Basic.reject(state.channel, metadata.delivery_tag,
            requeue: not metadata.redelivered
          )
    end

    {:noreply, state}
  end
end
