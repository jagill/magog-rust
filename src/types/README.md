This module contains the types that underlie geometry.

These types are "geometrical", in that the represent objects in 2D euclidean
space, and strive to follow the
[OpenGIS spec](https://www.opengeospatial.org/standards/sfa).  Choices are made
in sitations that OpenGIS underspecifies.

The geometrical types are:
  * Point: A point on a 2-d plane.  This is basically a promoted Position.
  * LineString: A series of points on a 2-d plane, tracing out a route.  This
    is basically a promoted Vec<Position>.
  * Polygon: A polygon is a solid region in the plane.  It may have holes.  It
    has a single exterior LineString, which must be a loop (the final coordinate
    should be the same as the first coordinate).  It may have 0 to many interior
    loops, which are holes.
  * MultiPoint: Multiple Points.
  * MultiLineString: Multiple LineStrings.
  * MultiPolygon: Multiple Polgons.
