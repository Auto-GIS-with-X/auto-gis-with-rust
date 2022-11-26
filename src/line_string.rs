use std::{convert::TryFrom, fmt, ops::Deref};

use itertools::Itertools;
use num_traits::NumCast;

use crate::error::GeometryError;
use crate::{helpers, implement_deref};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LineString(Vec<[f64; 2]>);

impl LineString {
    /// Construct a new `LineString` from a vector of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::LineString;
    ///
    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 0.], [1., 1.]]).unwrap();
    ///
    /// assert_eq!("LINESTRING (0 0, 1 0, 1 1)", line_string_1.to_string());
    /// ```
    ///
    /// Construct a new `LineString` from a vector of floats or a vector of integers.
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::LineString;
    ///
    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 0.], [1., 1.]]).unwrap();
    /// let line_string_2 = LineString::new(vec![[0, 0], [1, 0], [1, 1]]).unwrap();
    ///
    /// assert_eq!(line_string_1, line_string_2);
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

implement_deref!(LineString, Vec<[f64; 2]>);

impl fmt::Display for LineString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = self.iter().format_with(", ", |point, f| {
            f(&format_args!("{} {}", point[0], point[1]))
        });
        write!(f, "LINESTRING ({})", points)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct MultiLineString(Vec<LineString>);

impl MultiLineString {
    /// Construct a new `MultiLineString` from a vector of 'LineString's.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::{LineString, MultiLineString};
    ///
    /// let line_string_1 = LineString::new(vec![[0., 0.], [1., 0.], [1., 1.]]).unwrap();
    /// let line_string_2 = LineString::new(vec![[1., 2.], [0., 2.], [0., 1.]]).unwrap();
    ///
    /// let multi_line_string = MultiLineString::new(vec![line_string_1, line_string_2]);
    ///
    /// assert_eq!("MULTILINESTRING ((0 0, 1 0, 1 1), (1 2, 0 2, 0 1))", multi_line_string.to_string());
    /// ```
    pub fn new(linestrings: Vec<LineString>) -> Self {
        MultiLineString(linestrings)
    }
}

implement_deref!(MultiLineString, Vec<LineString>);

impl fmt::Display for MultiLineString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line_strings = self
            .iter()
            .map(|line_string| {
                line_string.iter().format_with(", ", |point, f| {
                    f(&format_args!("{} {}", point[0], point[1]))
                })
            })
            .format_with(", ", |line_string, f| f(&format_args!("({})", line_string)));
        write!(f, "MULTILINESTRING ({})", line_strings)
    }
}

impl<T: NumCast> TryFrom<Vec<Vec<[T; 2]>>> for MultiLineString {
    type Error = GeometryError;

    /// Tries to convert a vector or vectors of 2-float arrays into a `MultiLineString`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use auto_gis_with_rust::line_string::{LineString, MultiLineString};
    ///
    /// let multi_line_string = MultiLineString::try_from(vec![
    ///    vec![[0., 0.], [1., 0.], [1., 1.]],
    ///    vec![[1., 2.], [0., 2.], [0., 1.]],
    /// ]).unwrap();
    ///
    /// assert_eq!("MULTILINESTRING ((0 0, 1 0, 1 1), (1 2, 0 2, 0 1))", multi_line_string.to_string());
    /// ```
    fn try_from(vectors: Vec<Vec<[T; 2]>>) -> Result<Self, GeometryError> {
        let line_strings: Result<Vec<LineString>, GeometryError> =
            vectors.into_iter().map(LineString::new).collect();
        Ok(MultiLineString::new(line_strings?))
    }
}
