defmodule Obscura.Vector2 do
  defstruct [:x, :y]

  alias Obscura.{Point2, Vector2}

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

defmodule Obscura.Vector3 do
  defstruct [:x, :y, :z]

  alias Obscura.{Normal3, Point3, Vector3}

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

defmodule Obscura.Point2 do
  defstruct [:x, :y]

  alias Obscura.{Point2, Vector2}

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

defmodule Obscura.Point3 do
  defstruct [:x, :y, :z]

  alias Obscura.{Point3, Vector3}

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

  alias Obscura.{Normal3, Vector3}

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

  alias Obscura.{Point3, Ray, RayDifferential, Vector3}

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

  alias Obscura.{Point2, Vector2}

  @type t :: %__MODULE__{min: Obscura.Point2.t(), max: Obscura.Point2.t()}

  @spec area(Obscura.Bounds2.t()) :: number
  def area(b) do
    d = diagonal(b)

    d.x * d.y
  end

  @spec diagonal(Obscura.Bounds2.t()) :: Obscura.Vector2.t()
  def diagonal(b), do: Point2.sub(b.max, b.min)

  @spec lerp(Obscura.Bounds2.t(), Obscura.Point2.t()) :: Obscura.Point2.t()
  def lerp(b, t) do
    Point2.new(
      b.min.x * (1 - t.x) + b.max.x * t.x,
      b.min.y * (1 - t.y) + b.max.y * t.y
    )
  end

  @spec maximum_extent(Obscura.Bounds2.t()) :: 0 | 1
  def maximum_extent(b) do
    d = diagonal(b)

    if d.x > d.y, do: 0, else: 1
  end

  @spec new(Obscura.Point2.t(), Obscura.Point2.t()) :: Obscura.Bounds2.t()
  def new(p1, p2) do
    min = Point2.new(Kernel.min(p1.x, p2.x), Kernel.min(p1.y, p2.y))
    max = Point2.new(Kernel.max(p1.x, p2.x), Kernel.max(p1.y, p2.y))

    %__MODULE__{min: min, max: max}
  end

  @spec offset(Obscura.Bounds2.t(), Obscura.Point2.t()) :: Obscura.Vector2.t()
  def offset(b, p) do
    o = Point2.sub(p, b.min)

    x = if b.max.x > b.min.x, do: o.x / (b.max.x - b.min.x), else: o.x
    y = if b.max.y > b.min.y, do: o.y / (b.max.y - b.min.y), else: o.y

    Vector2.new(x, y)
  end
end

defmodule Obscura.Bounds3 do
  defstruct [:min, :max]

  use Bitwise

  alias Obscura.{Bounds3, Point3, Vector3}

  @type t :: %__MODULE__{min: Obscura.Point3.t(), max: Obscura.Point3.t()}

  @spec new(Obscura.Point3.t(), Obscura.Point3.t()) :: Obscura.Bounds3.t()
  def new(p1, p2) do
    min = Point3.new(Kernel.min(p1.x, p2.x), Kernel.min(p1.y, p2.y), Kernel.min(p1.z, p2.z))
    max = Point3.new(Kernel.max(p1.x, p2.x), Kernel.max(p1.y, p2.y), Kernel.min(p1.z, p2.z))

    %__MODULE__{min: min, max: max}
  end

  @spec corner(Obscura.Bounds3.t(), integer) :: Obscura.Point3.t()
  def corner(b, i) do
    x = if((i &&& 1) == 0, do: b.min.x, else: b.max.x)
    y = if((i &&& 2) == 0, do: b.min.y, else: b.max.y)
    z = if((i &&& 4) == 0, do: b.min.z, else: b.max.z)

    Point3.new(x, y, z)
  end

  @spec union(Obscura.Bounds3.t(), Obscura.Point3.t()) :: Obscura.Bounds3.t()
  def union(b, %Obscura.Point3{} = p) do
    min = Point3.new(Kernel.min(b.min.x, p.x), Kernel.min(b.min.y, p.y), Kernel.min(b.min.z, p.z))
    max = Point3.new(Kernel.max(b.max.x, p.x), Kernel.max(b.max.y, p.y), Kernel.max(b.max.z, p.z))

    Bounds3.new(min, max)
  end

  @spec union(Obscura.Bounds3.t(), Obscura.Bounds3.t()) :: Obscura.Bounds3.t()
  def union(b1, %Obscura.Bounds3{} = b2) do
    min =
      Point3.new(
        Kernel.min(b1.min.x, b2.min.x),
        Kernel.min(b1.min.y, b2.min.y),
        Kernel.min(b1.min.z, b2.min.z)
      )

    max =
      Point3.new(
        Kernel.max(b1.max.x, b2.max.x),
        Kernel.max(b1.max.y, b2.max.y),
        Kernel.max(b1.max.z, b2.max.z)
      )

    Bounds3.new(min, max)
  end

  @spec intersect(Obscura.Bounds3.t(), Obscura.Bounds3.t()) :: Obscura.Bounds3.t()
  def intersect(b1, b2) do
    min =
      Point3.new(
        Kernel.max(b1.min.x, b2.min.x),
        Kernel.max(b1.min.y, b2.min.y),
        Kernel.max(b1.min.z, b2.min.z)
      )

    max =
      Point3.new(
        Kernel.min(b1.max.x, b2.max.x),
        Kernel.min(b1.max.y, b2.max.y),
        Kernel.min(b1.max.z, b2.max.z)
      )

    Bounds3.new(min, max)
  end

  @spec overlaps(Obscura.Bounds3.t(), Obscura.Bounds3.t()) :: boolean
  def overlaps(b1, b2) do
    x = b1.max.x >= b2.min.x && b1.min.x <= b2.max.x
    y = b1.max.y >= b2.min.y && b1.min.y <= b2.max.y
    z = b1.max.z >= b2.min.z && b1.min.z <= b2.max.z

    x && y && z
  end

  @spec inside(Obscura.Bounds3.t(), Obscura.Point3.t()) :: boolean
  def inside(b, p) do
    p.x >= b.min.x && p.x <= b.max.x &&
      p.y >= b.min.y && p.y <= b.max.y &&
      p.z >= b.min.z && p.z <= b.max.z
  end

  @spec inside_exclusive(Obscura.Bounds3.t(), Obscura.Point3.t()) :: boolean
  def inside_exclusive(b, p) do
    p.x >= b.min.x && p.x < b.max.x &&
      p.y >= b.min.y && p.y < b.max.y &&
      p.z >= b.min.z && p.z < b.max.z
  end

  @spec expand(Obscura.Bounds3.t(), number) :: Obscura.Bounds3.t()
  def expand(b, delta) do
    min = Point3.sub(b.min, Vector3.new(delta, delta, delta))
    max = Point3.add(b.max, Vector3.new(delta, delta, delta))

    Bounds3.new(min, max)
  end

  @spec diagonal(Obscura.Bounds3.t()) :: Obscura.Vector3.t()
  def diagonal(b), do: Point3.sub(b.max, b.min)

  @spec surface_area(Obscura.Bounds3.t()) :: number
  def surface_area(b) do
    d = diagonal(b)

    2 * (d.x * d.y + d.x * d.z + d.y * d.z)
  end

  @spec volume(Obscura.Bounds3.t()) :: number
  def volume(b) do
    d = diagonal(b)

    d.x * d.y * d.z
  end

  @spec maximum_extent(Obscura.Bounds3.t()) :: 0 | 1 | 2
  def maximum_extent(b) do
    d = diagonal(b)

    cond do
      d.x > d.y && d.x > d.z -> 0
      d.y > d.z -> 1
      true -> 2
    end
  end

  @spec lerp(Obscura.Bounds3.t(), Obscura.Point3.t()) :: Obscura.Point3.t()
  def lerp(b, t) do
    Point3.new(
      b.min.x * (1 - t.x) + b.max.x * t.x,
      b.min.y * (1 - t.y) + b.max.y * t.y,
      b.min.z * (1 - t.z) + b.max.z * t.z
    )
  end

  @spec offset(Obscura.Bounds3.t(), Obscura.Point3.t()) :: Obscura.Vector3.t()
  def offset(b, p) do
    o = Point3.sub(p, b.min)

    x = if b.max.x > b.min.x, do: o.x / (b.max.x - b.min.x), else: o.x
    y = if b.max.y > b.min.y, do: o.y / (b.max.y - b.min.y), else: o.y
    z = if b.max.z > b.min.z, do: o.z / (b.max.z - b.min.z), else: o.z

    Vector3.new(x, y, z)
  end

  @spec bounding_sphere(Obscura.Bounds3.t()) :: {Obscura.Point3.t(), number}
  def bounding_sphere(b) do
    center = Point3.add(b.min, b.max) |> Point3.div(2)
    radius = if inside(b, center), do: Point3.distance(center, b.max), else: 0

    {center, radius}
  end
end
