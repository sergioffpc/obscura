defmodule Obscura.MixProject do
  use Mix.Project

  def project do
    [
      app: :obscura,
      version: "0.1.0",
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
      compilers: [:elixir_make | Mix.compilers()],
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:elixir_make, "~> 0.6", runtime: false},
      {:amqp, "~> 3.0"},
      {:json, "~> 1.4"}
    ]
  end
end
