use num_traits::NumCast;

use crate::error::GeometryError;
use crate::helpers;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LineString(Vec<[f64; 2]>);

impl LineString {
    /// Construct a new `LineString` from a vector of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// Construct a new `LineString` from a vector of floats or a vector of integers.
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::LineString;
    ///
    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 1.]]).unwrap();
    /// let line_string_2 = LineString::new(vec![[0, 0], [1, 1]]).unwrap();
    ///
    /// assert_eq!(line_string_1, line_string_2)
    /// ```
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        let number_of_coordinates = coordinates.len();
        if number_of_coordinates < 2 {
            Err(GeometryError::TooFewCoords(number_of_coordinates))
        } else {
            let float_coordinates = helpers::get_float_coordinates(coordinates);
            Ok(LineString(float_coordinates))
        }
    }
}
