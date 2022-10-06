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
    /// use auto_gis_with_rust::Point;
    ///
    /// let point_0 = Point::new(0.0, 1.0);
    /// let point_1 = Point::new(0, 1);
    ///
    /// assert_eq!(point_0, point_1);
    /// ```
    pub fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
        let x_float: f64 = num_traits::cast(x).unwrap();
        let y_float: f64 = num_traits::cast(y).unwrap();
        Point([x_float, y_float])
    }
}
