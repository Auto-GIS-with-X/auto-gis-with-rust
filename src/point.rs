use std::{fmt, slice::Iter};

use itertools::Itertools;
use num_traits::{self, NumCast};

#[derive(Debug, PartialEq, PartialOrd)]
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
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POINT ({} {})", self.0[0], self.0[1])
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
    /// assert_eq!("MULTIPOINT (0 0, 1 0)", multi_point.to_string());
    /// ```
    pub fn new(points: Vec<Point>) -> Self {
        MultiPoint(points)
    }

    /// Returns an iterator of `Points`
    ///
    /// # Example
    /// ```
    /// use auto_gis_with_rust::point::{Point, MultiPoint};
    ///
    /// let point_0 = Point::new(0.0, 0.0);
    /// let point_1 = Point::new(1, 0);
    /// let multi_point = MultiPoint(vec![point_0, point_1]);
    ///
    /// assert_eq!("POINT (0 0)", multi_point.iter().next().unwrap().to_string())
    /// ```
    pub fn iter(&self) -> Iter<Point> {
        self.0.iter()
    }
}

impl fmt::Display for MultiPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = self.iter().format_with(", ", |point, f| {
            f(&format_args!("{} {}", point.0[0], point.0[1]))
        });
        write!(f, "MULTIPOINT ({})", points)
    }
}
