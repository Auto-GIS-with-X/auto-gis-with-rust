use std::{convert::TryFrom, fmt, ops::Deref};

use itertools::Itertools;
use num_traits::NumCast;

use crate::error::GeometryError;
use crate::point::Point;
use crate::traits::{self, Curve, Geometry};
use crate::{helpers, implement_deref};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct LineSegment([[f64; 2]; 2]);

impl LineSegment {
    /// A straight line connecting two points.
    ///
    /// # Examples:
    ///
    /// Construct a new `LineSegment` from an array of two 2-float arrays.
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment_1 = LineSegment::new([[0., 0.], [1., 1.]]);
    ///
    /// assert_eq!(line_segment_1.to_string(), "LINESTRING (0 0, 1 1)")
    /// ```
    ///
    /// Construct a new `LineSegment` from an array of two 2-integer arrays.
    ///
    /// ```
    /// # use auto_gis_with_rust::line_string::LineSegment;
    /// # let line_segment_1 = LineSegment::new([[0., 0.], [1., 1.]]);
    /// let line_segment_2 = LineSegment::new([[0, 0], [1, 1]]);
    ///
    /// assert_eq!(line_segment_1, line_segment_2)
    /// ```
    pub fn new<T: NumCast>(coordinates: [[T; 2]; 2]) -> Self {
        let float_coordinates: [[f64; 2]; 2] = coordinates.map(|coordinate| {
            coordinate.map(|coordinate| -> f64 { num_traits::cast(coordinate).unwrap() })
        });
        LineSegment(float_coordinates)
    }

    /// The distance the x coordinates of the end `Point` and the start `Point`
    /// of this `LineSegment`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [4., 3.]]);
    ///
    /// assert_eq!(line_segment.x_length(), 4.)  
    /// ```
    pub fn x_length(&self) -> f64 {
        self.end_point().x() - self.start_point().x()
    }

    /// The distance the y coordinates of the end `Point` and the start `Point`
    /// of this `LineSegment`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [4., 3.]]);
    ///
    /// assert_eq!(line_segment.y_length(), 3.)  
    /// ```
    pub fn y_length(&self) -> f64 {
        self.end_point().y() - self.start_point().y()
    }
}

implement_deref!(LineSegment, [[f64; 2]; 2]);

impl fmt::Display for LineSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let points = self.iter().format_with(", ", |point, f| {
            f(&format_args!("{} {}", point[0], point[1]))
        });
        write!(f, "LINESTRING ({})", points)
    }
}

impl Geometry for LineSegment {
    /// Compute the geometric center of a geometry.
    ///
    /// For a `LineSegment`, this is a `Point` half-way between the start `Point`
    /// and the end `Point` of that `LineSegment`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::traits::Geometry;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [4., 3.]]);
    /// let centroid = line_segment.centroid();
    ///
    /// assert_eq!(centroid.to_string(), "POINT (2 1.5)")  
    /// ```
    fn centroid(&self) -> Point {
        let x = self.x_length() / 2.;
        let y = self.y_length() / 2.;
        Point::new(x, y)
    }

    /// A `LineSegment` is always simple.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::traits::Geometry;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment_1 = LineSegment::new([[0., 0.], [4., 3.]]);
    ///
    /// assert!(line_segment_1.is_simple())  
    /// ```
    ///
    /// ```
    /// use auto_gis_with_rust::traits::Geometry;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment_2 = LineSegment::new([[0., 0.], [0., 0.]]);
    ///
    /// assert!(line_segment_2.is_simple())  
    /// ```
    fn is_simple(&self) -> bool {
        for point in self.iter() {
            let mut matches: usize = 0;
            for other_point in self.iter() {
                if point == other_point {
                    matches += 1;
                    if self.is_closed() {
                        if matches > 2 {
                            return false;
                        }
                    } else if matches > 1 {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Curve for LineSegment {
    /// The length of this `LineSegment` in its associated spatial reference.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::traits::Curve;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [4., 3.]]);
    ///
    /// assert_eq!(line_segment.length(), 5.)  
    /// ```
    fn length(&self) -> f64 {
        let a = self.y_length();
        let b = self.x_length();
        let c_squared = a.powi(2) + b.powi(2);
        c_squared.sqrt()
    }

    /// The start `Point` of this `LineSegment`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::traits::Curve;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [1., 1.]]);
    /// let start_point = line_segment.start_point();
    ///
    /// assert_eq!(start_point.to_string(), "POINT (0 0)")
    /// ```
    fn start_point(&self) -> Point {
        let [x, y] = self[0];
        Point::new(x, y)
    }

    /// The end `Point` of this `LineSegment`.
    ///
    /// # Examples
    ///
    /// ```
    /// use auto_gis_with_rust::traits::Curve;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [1., 1.]]);
    /// let end_point = line_segment.end_point();
    ///
    /// assert_eq!(end_point.to_string(), "POINT (1 1)")
    /// ```
    fn end_point(&self) -> Point {
        let [x, y] = self[1];
        Point::new(x, y)
    }

    /// Returns true if this `LineSegment` is closed, i.e. if the start `Point` is
    /// equal to the end `Point`.
    ///
    /// # Examples:
    ///
    /// Generally, a `LineSegment` won't be closed.
    ///
    /// ```
    /// use auto_gis_with_rust::traits::Curve;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment_1 = LineSegment::new([[0., 0.], [1., 1.]]);
    ///
    /// assert_eq!(line_segment_1.is_closed(), false)
    /// ```
    ///
    /// But it could be...
    ///
    /// ```
    /// # use auto_gis_with_rust::traits::Curve;
    /// # use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment_2 = LineSegment::new([[0., 0.], [0., 0.]]);
    ///
    /// assert!(line_segment_2.is_closed())
    /// ```
    fn is_closed(&self) -> bool {
        self.start_point() == self.end_point()
    }

    /// Returns true if this `LineSegment` is closed, i.e. if the start `Point` is
    /// equal to the end `Point`, and this `LineSegment` is simple, i.e. it does
    /// not pass through the same `Point` more than once.
    ///
    /// # Examples:
    ///
    /// Generally, a `LineSegment` won't be a ring.
    ///
    /// ```
    /// use auto_gis_with_rust::traits::Curve;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment_1 = LineSegment::new([[0., 0.], [1., 1.]]);
    ///
    /// assert_eq!(line_segment_1.is_ring(), false)
    /// ```
    ///
    /// But it could be...
    ///
    /// ```
    /// # use auto_gis_with_rust::traits::Curve;
    /// # use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment_2 = LineSegment::new([[0., 0.], [0., 0.]]);
    ///
    /// assert!(line_segment_2.is_ring())
    /// ```
    fn is_ring(&self) -> bool {
        self.is_closed() && self.is_simple()
    }
}

impl traits::LineString for LineSegment {
    /// Returns the number of `Point`s in this `LineSegment`, i.e. 2.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::traits::LineString;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [1., 1.]]);
    /// let points = line_segment.num_points();
    ///
    /// assert_eq!(points, 2);
    /// ```
    fn num_points(&self) -> usize {
        self.len()
    }

    /// Returns the Nth `Point` in this `LineSegment`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::traits::LineString;
    /// use auto_gis_with_rust::line_string::LineSegment;
    ///
    /// let line_segment = LineSegment::new([[0., 0.], [1., 1.]]);
    /// let point_0 = line_segment.point_n(0);
    ///
    /// assert_eq!(point_0.to_string(), "POINT (0 0)");
    /// ```
    fn point_n(&self, number: usize) -> Point {
        Point::from(self[number])
    }
}

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
