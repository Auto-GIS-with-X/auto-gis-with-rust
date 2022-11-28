use std::{fmt, ops::Deref};

use itertools::Itertools;
use num_traits::{self, NumCast};

use crate::geometry::{Geometry, GeometryCollection};
use crate::implement_deref;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point([f64; 2]);

impl Point {
    /// Construct a new `Point`.
    ///
    /// # Examples:
    ///
    /// Construct a new point from x and y floats or x and y integers.
    ///
    /// ```
    /// use auto_gis_with_rust::point::Point;
    ///
    /// let point_0 = Point::new(0.0, 1.0);
    /// let point_1 = Point::new(0, 1);
    ///
    /// assert_eq!("POINT (0 1)", point_0.to_string());
    ///
    /// assert_eq!(point_0, point_1);
    /// ```
    pub fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
        let x_float: f64 = num_traits::cast(x).unwrap();
        let y_float: f64 = num_traits::cast(y).unwrap();
        Point([x_float, y_float])
    }

    /// Return the x-coordinate value for this `Point`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::point::Point;
    ///
    /// let point = Point::new(0.0, 1.0);
    /// let x = point.x();
    ///
    /// assert_eq!(x, 0f64);
    /// ```
    pub fn x(&self) -> f64 {
        self[0]
    }

    /// Return the y-coordinate value for this `Point`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::point::Point;
    ///
    /// let point = Point::new(0.0, 1.0);
    /// let y = point.y();
    ///
    /// assert_eq!(y, 1f64);
    /// ```
    pub fn y(&self) -> f64 {
        self[1]
    }
}

implement_deref!(Point, [f64; 2]);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POINT ({} {})", self.x(), self.y())
    }
}

impl<T: NumCast + Copy> From<[T; 2]> for Point {
    /// Construct a `Point` from a 2-element array.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::point::Point;
    ///
    /// let point = Point::from([0.0, 1.0]);
    ///
    /// assert_eq!("POINT (0 1)", point.to_string());
    /// ```
    fn from(coordinates: [T; 2]) -> Self {
        Point::new(coordinates[0], coordinates[1])
    }
}

impl Geometry for Point {
    /// Compute the geometric center of a geometry.
    ///
    /// For a `Point`, this is a new `Point` with the same coordinates.
    ///
    /// ```
    /// use auto_gis_with_rust::geometry::Geometry;
    /// use auto_gis_with_rust::point::Point;
    ///
    /// let point = Point::new(0.0, 1.0);
    /// let expected_centroid = Point::new(0.0, 1.0);
    ///
    /// assert_eq!(point.centroid(), expected_centroid);
    /// ```
    fn centroid(&self) -> Point {
        Point::new(self.x(), self.y())
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct MultiPoint(pub Vec<Point>);

impl MultiPoint {
    /// Construct a new `MultiPoint`.
    ///
    /// # Examples:
    ///
    /// Construct a new multi-point vector of `Point`s.
    ///
    /// ```
    /// use auto_gis_with_rust::point::{Point, MultiPoint};
    ///
    /// let point_0 = Point::new(0.0, 0.0);
    /// let point_1 = Point::new(1, 0);
    /// let multi_point = MultiPoint(vec![point_0, point_1]);
    ///
    /// assert_eq!("MULTIPOINT ((0 0), (1 0))", multi_point.to_string());
    /// ```
    pub fn new(points: Vec<Point>) -> Self {
        MultiPoint(points)
    }
}

implement_deref!(MultiPoint, Vec<Point>);

impl fmt::Display for MultiPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = self.iter().format_with(", ", |point, f| {
            f(&format_args!("({} {})", point[0], point[1]))
        });
        write!(f, "MULTIPOINT ({})", points)
    }
}

impl<T: NumCast + Copy> From<Vec<[T; 2]>> for MultiPoint {
    /// Construct a `MultiPoint` from a vector of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::point::MultiPoint;
    ///
    /// let multi_point = MultiPoint::from(vec![[0.0, 0.0], [1.0, 0.0]]);
    ///
    /// assert_eq!("MULTIPOINT ((0 0), (1 0))", multi_point.to_string());
    /// ```
    fn from(items: Vec<[T; 2]>) -> Self {
        let points: Vec<Point> = items.into_iter().map(Point::from).collect();
        MultiPoint::new(points)
    }
}

impl GeometryCollection<Point> for MultiPoint {
    /// Returns the number of `Point`s in this `MultiPoint` collection.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::geometry::GeometryCollection;
    /// use auto_gis_with_rust::point::MultiPoint;
    ///
    /// let multi_point = MultiPoint::from(vec![[0.0, 0.0], [1.0, 0.0]]);
    /// let points = multi_point.num_geometries();
    ///
    /// assert_eq!(points, 2);
    /// ```
    fn num_geometries(&self) -> usize {
        self.len()
    }

    /// Returns the Nth `Point` in this `MultiPoint` collection.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::geometry::GeometryCollection;
    /// use auto_gis_with_rust::point::MultiPoint;
    ///
    /// let multi_point = MultiPoint::from(vec![[0.0, 0.0], [1.0, 0.0]]);
    /// let point_0 = multi_point.geometry_n(0);
    ///
    /// assert_eq!("POINT (0 0)", point_0.to_string());
    /// ```
    fn geometry_n(&self, number: usize) -> Point {
        self[number]
    }
}

impl Geometry for MultiPoint {
    /// Compute the geometric center of a geometry.
    ///
    /// For a `MultiPoint`, this is a new `Point` with the mean x and y coordinates of all the points in the collection.
    ///
    /// ```
    /// use auto_gis_with_rust::geometry::Geometry;
    /// use auto_gis_with_rust::point::MultiPoint;
    ///
    /// let multi_point = MultiPoint::from(vec![[0., 0.], [1., 0.]]);
    ///
    /// assert_eq!(multi_point.centroid().to_string(), "POINT (0.5 0)");
    /// ```
    fn centroid(&self) -> Point {
        let points = self.num_geometries() as f64;
        let sum_x: f64 = self.iter().map(|point| point.x()).sum();
        let sum_y: f64 = self.iter().map(|point| point.y()).sum();
        Point::new(sum_x / points, sum_y / points)
    }
}
