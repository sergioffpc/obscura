defmodule Obscura.Vector2Test do
  use ExUnit.Case
  doctest Obscura.Vector2

  import Obscura.Vector2

  test "vector2 absolute" do
    a = new(1, 1)
    assert Obscura.Vector2.abs(neg(a)) == a
  end

  test "vector2 addition" do
    a = new(1, 1)
    assert add(a, neg(a)) == new()
  end

  test "vector2 addition is commutative" do
    a = new(1, 1)
    b = new(1, -1)
    assert add(a, b) == add(b, a)
  end

  test "vector2 addition is associative" do
    a = new(1, 1)
    b = new(1, -1)
    c = new(-1, -1)
    assert add(add(a, b), c) == add(a, add(b, c))
  end

  test "vector2 addition is distributive" do
    k = 2
    a = new(1, 1)
    b = new(1, -1)
    assert mul(add(a, b), k) == add(mul(a, k), mul(b, k))
  end

  test "vector2 division by identity" do
    a = new(1, 1)
    assert Obscura.Vector2.div(a, 1) == a
  end

  test "vector2 zero length" do
    a = new(0, 0)
    assert Obscura.Vector2.length(a) == 0
  end

  test "vector2 length" do
    a = new(3, 4)
    assert Obscura.Vector2.length(a) == 5
  end

  test "vector2 multiplication by identity" do
    a = new(1, 1)
    assert mul(a, 1) == a
  end

  test "vector2 negation" do
    a = new(1, 1)
    assert neg(neg(a)) == a
  end

  test "vector2 normalization" do
    a = new(3, 4)
    assert Obscura.Vector2.length(normalize(a)) == 1
  end

  test "vector2 subtraction" do
    a = new(1, 1)
    assert sub(a, a) == new()
  end

  test "vector2 subtraction is not commutative" do
    a = new(1, 1)
    b = new(1, -1)
    assert sub(a, b) != sub(b, a)
  end

  test "vector2 subtraction is not associative" do
    a = new(1, 1)
    b = new(1, -1)
    c = new(-1, -1)
    assert sub(sub(a, b), c) != sub(a, sub(b, c))
  end

  test "vector2 subtraction is distributive" do
    k = 2
    a = new(1, 1)
    b = new(1, -1)
    assert mul(sub(a, b), k) == sub(mul(a, k), mul(b, k))
  end
end

defmodule Obscura.Vector3Test do
  use ExUnit.Case
  doctest Obscura.Vector3

  import Obscura.Vector3

  test "vector3 absolute" do
    a = new(1, 1, 1)
    assert Obscura.Vector3.abs(neg(a)) == a
  end

  test "vector3 addition" do
    a = new(1, 1, 1)
    assert add(a, neg(a)) == new()
  end

  test "vector3 addition is commutative" do
    a = new(1, 1, 1)
    b = new(1, -1, 1)
    assert add(a, b) == add(b, a)
  end

  test "vector3 addition is associative" do
    a = new(1, 1, 1)
    b = new(1, -1, 1)
    c = new(-1, -1, 1)
    assert add(add(a, b), c) == add(a, add(b, c))
  end

  test "vector3 addition is distributive" do
    k = 2
    a = new(1, 1, 1)
    b = new(1, -1, 1)
    assert mul(add(a, b), k) == add(mul(a, k), mul(b, k))
  end

  test "vector3 cross product is anticommutative" do
    a = new(1, 1, 1)
    b = new(1, -1, 1)
    assert cross(a, b) == neg(cross(b, a))
  end

  test "vector3 cross product is distributive" do
    a = new(1, 1, 1)
    b = new(1, -1, 1)
    c = new(-1, -1, 1)
    assert cross(a, add(b, c)) == add(cross(a, b), cross(a, c))
  end

  test "vector3 division by identity" do
    a = new(1, 1, 1)
    assert Obscura.Vector3.div(a, 1) == a
  end

  test "vector3 dot product is distributive" do
    a = new(1, 1, 1)
    b = new(1, -1, 1)
    c = new(-1, -1, 1)
    assert dot(a, add(b, c)) == dot(a, b) + dot(a, c)
  end

  test "vector3 zero length" do
    a = new(0, 0, 0)
    assert Obscura.Vector3.length(a) == 0
  end

  test "vector3 length" do
    a = new(3, 4, 0)
    assert Obscura.Vector3.length(a) == 5
  end

  test "vector3 multiplication by identity" do
    a = new(1, 1, 1)
    assert mul(a, 1) == a
  end

  test "vector3 multiplication by zero" do
    a = new(1, 1, 1)
    assert mul(a, 0) == new()
  end

  test "vector3 negation" do
    a = new(1, 1, 1)
    assert neg(neg(a)) == a
  end

  test "vector3 normalization" do
    a = new(3, 4, 0)
    assert Obscura.Vector3.length(normalize(a)) == 1
  end

  test "vector3 subtraction" do
    a = new(1, 1, 1)
    assert sub(a, a) == new()
  end

  test "vector3 subtraction is not commutative" do
    a = new(1, 1, 1)
    b = new(1, -1, 1)
    assert sub(a, b) != sub(b, a)
  end

  test "vector3 subtraction is not associative" do
    a = new(1, 1, 1)
    b = new(1, -1, 1)
    c = new(-1, -1, 1)
    assert sub(sub(a, b), c) != sub(a, sub(b, c))
  end

  test "vector3 subtraction is distributive" do
    k = 2
    a = new(1, 1, 1)
    b = new(1, -1, 1)
    assert mul(sub(a, b), k) == sub(mul(a, k), mul(b, k))
  end
end
