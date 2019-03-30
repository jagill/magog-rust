This module contains the types that underlie geometry.

The first class of types are "foundational" types; geometrical operations will
reduce to operations on these.  They are all fixed size, cheaply copyable,
and this safe and quick on the stack.

The foundational types are:
  * Position: An x-y pair.  Note that we let the actual value to be any float.
  * Segment: A finite line between two points.
  * Triangle: Three points.  They may be colinear.
  * Rect: A bounding box with min/max-x/y values.  This will allow many
    algorithms to short-circuit quickly.
  * Envelope: Either empty (None), or a Rect. A None Envelope comes from an
    empty geometry.

The second class of types are the "geometrical" types, which may contain many
coordinates in various configurations.

The geometrical types are:
  * Point: A point on a 2-d plane.  This is basically a promoted Position.
  * LineString: A series of points on a 2-d plane, tracing out a root.  This
    is basically a promoted Vec<Position>.
  * Polygon: A polygon is a solid region in the plane.  It may have holes.  It
    has a single exterior LineString, which must be a loop (the final coordinate
    should be the same as the first coordinate).  It may have 0 to many interior
    loops, which are holes.
  * MultiPoint: Multiple Points.
  * MultiLineString: Multiple LineStrings.
  * MultiPolygon: Multiple Polgons.
