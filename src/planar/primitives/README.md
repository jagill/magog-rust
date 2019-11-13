This module contains the primitive types that underlie geometry.

These primitive types are are "foundational" types; geometrical operations will
reduce to operations on these.  They are all fixed size and cheaply copyable,
so this safe and quick on the stack.

The primitive types are:
  * Coordinate: A single coordinate, eg distance along the x-axis.  Note that we
    let the actual value to be any float.
  * Position: An x-y pair of Coordinates.  
  * Segment: A finite line between two points, start and end.  Start and end may
    be identical.
  * Triangle: Three points.  They may be colinear.
  * Rect: A bounding box with min/max-x/y values.  This will allow many
    algorithms to short-circuit quickly.
  * Envelope: Either empty (None), or a Rect. A None Envelope comes from an
    empty geometry, and should be viewed as the empty geometry wrt intersections,
    containment, etc.
