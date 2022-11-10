use num_traits::NumCast;

use crate::error::GeometryError;
use crate::helpers;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct PolygonRing(Vec<[f64; 2]>);

impl PolygonRing {
    /// Construct a new `PolygonRing` from a vector of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// Construct a new `PolygonRing` from a vector of floats or a vector of integers.
    ///
    /// ```
    /// use auto_gis_with_rust::polygon::PolygonRing;
    ///
    /// let polygon_ring_1 = PolygonRing::new(vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]).unwrap();
    /// let polygon_ring_2 = PolygonRing::new(vec![[0, 0], [0, 1], [1, 1]]).unwrap();
    ///
    /// assert_eq!(polygon_ring_1, polygon_ring_2)
    /// ```
    pub fn new<T: NumCast>(coordinates: Vec<[T; 2]>) -> Result<Self, GeometryError> {
        let number_of_coordinates = coordinates.len();
        if number_of_coordinates < 3 {
            Err(GeometryError::TooFewCoords(number_of_coordinates))
        } else {
            let mut float_coordinates = helpers::get_float_coordinates(coordinates);
            if float_coordinates[0] != float_coordinates[number_of_coordinates - 1] {
                float_coordinates.push(float_coordinates[0]);
                Ok(PolygonRing(float_coordinates))
            } else {
                Ok(PolygonRing(float_coordinates))
            }
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Polygon(Vec<PolygonRing>);

impl Polygon {
    /// Construct a new `Polygon` from a vector of vectors of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// Construct a new `Polygon` from a vector of vectors of floats or a vector of vectors of integers.
    ///
    /// ```
    /// use auto_gis_with_rust::polygon::Polygon;
    ///
    /// let polygon_1 = Polygon::new(vec![vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]]).unwrap();
    /// let polygon_2 = Polygon::new(vec![vec![[0, 0], [0, 1], [1, 1]]]).unwrap();
    ///
    /// assert_eq!(polygon_1, polygon_2)
    /// ```
    pub fn new<T: NumCast>(rings: Vec<Vec<[T; 2]>>) -> Result<Self, GeometryError> {
        let polygon_rings: Vec<PolygonRing> = rings
            .into_iter()
            .map(|ring| PolygonRing::new(ring).unwrap())
            .collect();
        Ok(Polygon(polygon_rings))
    }
}
