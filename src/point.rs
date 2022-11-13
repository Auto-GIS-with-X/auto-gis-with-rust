use std::fmt;

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
    /// assert_eq!("POINT (0 1)", format!("{}", point_0));
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
    /// let point_1 = Point::new(1.0, 0.0);
    /// let multi_point_0 = MultiPoint(vec![point_0, point_1]);

    /// let point_2 = Point::new(0, 0);
    /// let point_3 = Point::new(1, 0);
    /// let multi_point_1 = MultiPoint(vec![point_2, point_3]);
    ///
    /// assert_eq!(multi_point_0, multi_point_1);
    /// ```
    pub fn new(points: Vec<Point>) -> Self {
        MultiPoint(points)
    }
}
