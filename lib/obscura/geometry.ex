defmodule Obscura.Vector2 do
  defstruct [:x, :y]

  alias Obscura.Point2
  alias Obscura.Vector2

  @type t :: %__MODULE__{x: number, y: number}

  @spec abs(Obscura.Vector2.t()) :: Obscura.Vector2.t()
  def abs(v), do: Vector2.new(Kernel.abs(v.x), Kernel.abs(v.y))

  @spec add(Obscura.Vector2.t(), Obscura.Vector2.t()) :: Obscura.Vector2.t()
  def add(v, w), do: Vector2.new(v.x + w.x, v.y + w.y)

  @spec div(Obscura.Vector2.t(), number) :: Obscura.Vector2.t()
  def div(v, s), do: Vector2.mul(v, 1 / s)

  @spec length(Obscura.Vector2.t()) :: number
  def length(v), do: :math.sqrt(v.x * v.x + v.y * v.y)

  @spec mul(Obscura.Vector2.t(), number) :: Obscura.Vector2.t()
  def mul(v, s), do: Vector2.new(v.x * s, v.y * s)

  @spec new :: Obscura.Vector2.t()
  def new, do: new(0, 0)

  @spec new(number, number) :: Obscura.Vector2.t()
  def new(x, y), do: %__MODULE__{x: x, y: y}

  @spec neg(Obscura.Vector2.t()) :: Obscura.Vector2.t()
  def neg(v), do: Vector2.new(-v.x, -v.y)

  @spec normalize(Obscura.Vector2.t()) :: Obscura.Vector2.t()
  def normalize(v), do: Vector2.div(v, Vector2.length(v))

  @spec sub(Obscura.Vector2.t(), Obscura.Vector2.t()) :: Obscura.Vector2.t()
  def sub(v, w), do: Vector2.new(v.x - w.x, v.y - w.y)

  @spec to_list(Obscura.Vector2.t()) :: [...]
  def to_list(v), do: [v.x, v.y]

  @spec to_point2(Obscura.Vector2.t()) :: Obscura.Point2.t()
  def to_point2(v), do: Point2.new(v.x, v.y)
end

defmodule Obscura.Point2 do
  defstruct [:x, :y]

  alias Obscura.Point2
  alias Obscura.Vector2

  @type t :: %__MODULE__{x: number, y: number}

  @spec abs(Obscura.Point2.t()) :: Obscura.Point2.t()
  def abs(p), do: Point2.new(Kernel.abs(p.x), Kernel.abs(p.y))

  @spec add(Obscura.Point2.t(), Obscura.Point2.t()) :: Obscura.Point2.t()
  def add(p1, %Point2{} = p2), do: Point2.new(p1.x + p2.x, p1.y + p2.y)

  @spec add(Obscura.Point2.t(), Obscura.Vector2.t()) :: Obscura.Point2.t()
  def add(p, %Vector2{} = v), do: Point2.new(p.x + v.x, p.y + v.y)

  @spec distance(Obscura.Point2.t(), Obscura.Point2.t()) :: number
  def distance(p1, p2), do: sub(p1, p2) |> Vector2.length()

  @spec div(Obscura.Point2.t(), number) :: Obscura.Point2.t()
  def div(p, s), do: Point2.mul(p, 1 / s)

  @spec lerp(number, Obscura.Point2.t(), Obscura.Point2.t()) :: Obscura.Point2.t()
  def lerp(t, p1, p2), do: add(mul(p1, 1 - t), mul(p2, t))

  @spec mul(Obscura.Point2.t(), number) :: Obscura.Point2.t()
  def mul(p, s), do: Point2.new(p.x * s, p.y * s)

  @spec new :: Obscura.Point2.t()
  def new, do: new(0, 0)

  @spec new(number, number) :: Obscura.Point2.t()
  def new(x, y), do: %__MODULE__{x: x, y: y}

  @spec sub(Obscura.Point2.t(), Obscura.Point2.t()) :: Obscura.Vector2.t()
  def sub(p1, %Point2{} = p2), do: Vector2.new(p1.x - p2.x, p1.y - p2.y)

  @spec sub(Obscura.Point2.t(), Obscura.Vector2.t()) :: Obscura.Point2.t()
  def sub(p, %Vector2{} = v), do: Point2.new(p.x - v.x, p.y - v.y)

  @spec to_list(Obscura.Point2.t()) :: [...]
  def to_list(p), do: [p.x, p.y]

  @spec to_vector2(Obscura.Point2.t()) :: Obscura.Vector2.t()
  def to_vector2(p), do: Vector2.new(p.x, p.y)
end

defmodule Obscura.Vector3 do
  defstruct [:x, :y, :z]

  alias Obscura.Normal3
  alias Obscura.Point3
  alias Obscura.Vector3

  @type t :: %__MODULE__{x: number, y: number, z: number}

  @spec abs(Obscura.Vector3.t()) :: Obscura.Vector3.t()
  def abs(v), do: Vector3.new(Kernel.abs(v.x), Kernel.abs(v.y), Kernel.abs(v.z))

  @spec add(Obscura.Vector3.t(), Obscura.Vector3.t()) :: Obscura.Vector3.t()
  def add(v, w), do: Vector3.new(v.x + w.x, v.y + w.y, v.z + w.z)

  @spec cross(Obscura.Vector3.t(), Obscura.Vector3.t()) :: Obscura.Vector3.t()
  def cross(v, w),
    do: Vector3.new(v.y * w.z - v.z * w.y, v.z * w.x - v.x * w.z, v.x * w.y - v.y * w.x)

  @spec div(Obscura.Vector3.t(), number) :: Obscura.Vector3.t()
  def div(v, s), do: Vector3.mul(v, 1 / s)

  @spec dot(Obscura.Vector3.t(), Obscura.Vector3.t()) :: number
  def dot(v, w), do: v.x * w.x + v.y * w.y + v.z * w.z

  @spec length(Obscura.Vector3.t()) :: number
  def length(v), do: :math.sqrt(v.x * v.x + v.y * v.y + v.z * v.z)

  @spec mul(Obscura.Vector3.t(), number) :: Obscura.Vector3.t()
  def mul(v, s), do: Vector3.new(v.x * s, v.y * s, v.z * s)

  @spec new :: Obscura.Vector3.t()
  def new, do: new(0, 0, 0)

  @spec new(number, number, number) :: Obscura.Vector3.t()
  def new(x, y, z), do: %__MODULE__{x: x, y: y, z: z}

  @spec normalize(Obscura.Vector3.t()) :: Obscura.Vector3.t()
  def normalize(v), do: Vector3.div(v, Vector3.length(v))

  @spec neg(Obscura.Vector3.t()) :: Obscura.Vector3.t()
  def neg(v), do: Vector3.new(-v.x, -v.y, -v.z)

  @spec sub(Obscura.Vector3.t(), Obscura.Vector3.t()) :: Obscura.Vector3.t()
  def sub(v, w), do: Vector3.new(v.x - w.x, v.y - w.y, v.z - w.z)

  @spec to_normal3(Obscura.Vector3.t()) :: Obscura.Normal3.t()
  def to_normal3(v), do: Normal3.new(v.x, v.y, v.z)

  @spec to_point3(Obscura.Vector3.t()) :: Obscura.Point3.t()
  def to_point3(v), do: Point3.new(v.x, v.y, v.z)

  @spec to_list(Obscura.Vector3.t()) :: [...]
  def to_list(v), do: [v.x, v.y, v.z]
end

defmodule Obscura.Point3 do
  defstruct [:x, :y, :z]

  alias Obscura.Point3
  alias Obscura.Vector3

  @type t :: %__MODULE__{x: number, y: number, z: number}

  @spec abs(Obscura.Point3.t()) :: Obscura.Point3.t()
  def abs(p), do: Point3.new(Kernel.abs(p.x), Kernel.abs(p.y), Kernel.abs(p.z))

  @spec add(Obscura.Point3.t(), Obscura.Point3.t()) :: Obscura.Point3.t()
  def add(p1, %Point3{} = p2), do: Point3.new(p1.x + p2.x, p1.y + p2.y, p1.z + p2.z)

  @spec add(Obscura.Point3.t(), Obscura.Vector3.t()) :: Obscura.Point3.t()
  def add(p, %Vector3{} = v), do: Point3.new(p.x + v.x, p.y + v.y, p.z + v.z)

  @spec distance(Obscura.Point3.t(), Obscura.Point3.t()) :: number
  def distance(p1, p2), do: sub(p1, p2) |> Vector3.length()

  @spec div(Obscura.Point3.t(), number) :: Obscura.Point3.t()
  def div(p, s), do: Point3.mul(p, 1 / s)

  @spec lerp(number, Obscura.Point3.t(), Obscura.Point3.t()) :: Obscura.Point3.t()
  def lerp(t, p1, p2), do: add(mul(p1, 1 - t), mul(p2, t))

  @spec mul(Obscura.Point3.t(), number) :: Obscura.Point3.t()
  def mul(p, s), do: Point3.new(p.x * s, p.y * s, p.z * s)

  @spec new :: Obscura.Point3.t()
  def new, do: new(0, 0, 0)

  @spec new(number, number, number) :: Obscura.Point3.t()
  def new(x, y, z), do: %__MODULE__{x: x, y: y, z: z}

  @spec sub(Obscura.Point3.t(), Obscura.Point3.t()) :: Obscura.Vector3.t()
  def sub(p1, %Point3{} = p2), do: Vector3.new(p1.x - p2.x, p1.y - p2.y, p1.z - p2.z)

  @spec sub(Obscura.Point3.t(), Obscura.Vector3.t()) :: Obscura.Point3.t()
  def sub(p, %Vector3{} = v), do: Point3.new(p.x - v.x, p.y - v.y, p.z - v.z)

  @spec to_list(Obscura.Point3.t()) :: [...]
  def to_list(p), do: [p.x, p.y, p.z]

  @spec to_vector3(Obscura.Point3.t()) :: Obscura.Vector3.t()
  def to_vector3(p), do: Vector3.new(p.x, p.y, p.z)
end

defmodule Obscura.Normal3 do
  defstruct [:x, :y, :z]

  alias Obscura.Normal3
  alias Obscura.Vector3

  @type t :: %__MODULE__{x: number, y: number, z: number}

  @spec add(Obscura.Normal3.t(), Obscura.Normal3.t()) :: Obscura.Normal3.t()
  def add(n1, n2), do: Normal3.new(n1.x + n2.x, n1.y + n2.y, n1.z + n2.z)

  @spec div(Obscura.Normal3.t(), number) :: Obscura.Normal3.t()
  def div(n, s), do: Normal3.mul(n, 1 / s)

  @spec faceforward(Obscura.Normal3.t(), Obscura.Vector3.t()) :: Obscura.Normal3.t()
  def faceforward(n, v), do: if(Vector3.dot(to_vector3(n), v) < 0.0, do: neg(n), else: n)

  @spec length(Obscura.Normal3.t()) :: number
  def length(n), do: :math.sqrt(n.x * n.x + n.y * n.y + n.z * n.z)

  @spec mul(Obscura.Normal3.t(), number) :: Obscura.Normal3.t()
  def mul(n, s), do: Normal3.new(n.x * s, n.y * s, n.z * s)

  @spec new :: Obscura.Normal3.t()
  def new, do: new(0, 0, 0)

  @spec new(number, number, number) :: Obscura.Normal3.t()
  def new(x, y, z), do: %__MODULE__{x: x, y: y, z: z}

  @spec neg(Obscura.Normal3.t()) :: Obscura.Normal3.t()
  def neg(n), do: Normal3.new(-n.x, -n.y, -n.z)

  @spec sub(Obscura.Normal3.t(), Obscura.Normal3.t()) :: Obscura.Normal3.t()
  def sub(n1, n2), do: Normal3.new(n1.x - n2.x, n1.y - n2.y, n1.z - n2.z)

  @spec to_vector3(Obscura.Normal3.t()) :: Obscura.Vector3.t()
  def to_vector3(n), do: Vector3.new(n.x, n.y, n.z)

  @spec to_list(Obscura.Normal3.t()) :: [...]
  def to_list(n), do: [n.x, n.y, n.z]
end

defmodule Obscura.Ray do
  defstruct [:o, :d]

  alias Obscura.Point3

  @type t :: %__MODULE__{o: Obscura.Point3.t(), d: Obscura.Vector3.t()}

  @spec new(Obscura.Point3.t(), Obscura.Vector3.t()) :: Obscura.Ray.t()
  def new(o, d), do: %__MODULE__{o: o, d: d}

  @spec ray(Obscura.Ray, number) :: Obscura.Point3.t()
  def ray(r, t), do: Point3.mul(r.d, t) |> Point3.add(r.o)
end

defmodule Obscura.RayDifferential do
  defstruct [:r, :rx, :ry]

  alias Obscura.Point3
  alias Obscura.Ray
  alias Obscura.RayDifferential
  alias Obscura.Vector3

  @type t :: %__MODULE__{r: Obscura.Ray.t(), rx: Obscura.Ray.t(), ry: Obscura.Ray.t()}

  @spec new(Obscura.Ray.t(), Obscura.Ray.t(), Obscura.Ray.t()) :: Obscura.RayDifferential.t()
  def new(r, rx, ry), do: %__MODULE__{r: r, rx: rx, ry: ry}

  @spec scale_differentials(Obscura.RayDifferential.t(), number) :: Obscura.RayDifferential.t()
  def scale_differentials(rd, s) do
    rx_o = Point3.sub(rd.rx.o, rd.r.o) |> Point3.mul(s) |> Point3.add(rd.r.o)
    rx_d = Vector3.sub(rd.rx.d, rd.r.d) |> Vector3.mul(s) |> Vector3.add(rd.r.d)
    rx = Ray.new(rx_o, rx_d)

    ry_o = Point3.sub(rd.ry.o, rd.r.o) |> Point3.mul(s) |> Point3.add(rd.r.o)
    ry_d = Vector3.sub(rd.ry.d, rd.r.d) |> Vector3.mul(s) |> Vector3.add(rd.r.d)
    ry = Ray.new(ry_o, ry_d)

    RayDifferential.new(rd.r, rx, ry)
  end
end

defmodule Obscura.Bounds2 do
  defstruct [:min, :max]

  alias Obscura.Point2

  @type t :: %__MODULE__{min: Obscura.Point2.t(), max: Obscura.Point2.t()}

  @spec new(Obscura.Point2.t(), Obscura.Point2.t()) :: Obscura.Bounds2.t()
  def new(p1, p2) do
    min = Point2.new(Kernel.min(p1.x, p2.x), Kernel.min(p1.y, p2.y))
    max = Point2.new(Kernel.max(p1.x, p2.x), Kernel.max(p1.y, p2.y))

    %__MODULE__{min: min, max: max}
  end
end
