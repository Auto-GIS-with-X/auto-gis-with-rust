use std::{convert::TryFrom, fmt, ops::Deref};

use itertools::Itertools;
use num_traits::NumCast;

use crate::{error::GeometryError, helpers, implement_deref};

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

implement_deref!(PolygonRing, Vec<[f64; 2]>);

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Polygon(Vec<PolygonRing>);

impl Polygon {
    /// Construct a new `Polygon` from a vector of vectors of 2-element arrays.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::polygon::Polygon;
    ///
    /// let polygon_1 = Polygon::new(vec![vec![[0., 0.], [0., 1.], [1., 1.], [0., 0.]]]).unwrap();
    ///
    /// assert_eq!("POLYGON ((0 0, 0 1, 1 1, 0 0))", polygon_1.to_string());
    /// ```
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

implement_deref!(Polygon, Vec<PolygonRing>);

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rings = self
            .iter()
            .map(|ring| {
                ring.iter().format_with(", ", |point, f| {
                    f(&format_args!("{} {}", point[0], point[1]))
                })
            })
            .format_with(", ", |ring, f| f(&format_args!("({})", ring)));
        write!(f, "POLYGON ({})", rings)
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
pub struct MultiPolygon(Vec<Polygon>);

impl MultiPolygon {
    /// Construct a new `MultiPolygon` from a vector of `Polygon`s.
    ///
    /// # Examples:
    ///
    /// ```
    /// use auto_gis_with_rust::polygon::{MultiPolygon, Polygon};
    ///
    /// let polygon_1 = Polygon::new(vec![vec![[0., 0.], [0., 1.], [1., 1.], [1., 0.], [0., 0.]]]).unwrap();
    /// let polygon_2 = Polygon::new(vec![vec![[1, 1], [1, 2], [2, 2], [2, 1]]]).unwrap();
    ///
    /// let multi_polygon = MultiPolygon::new(vec![polygon_1, polygon_2]);
    ///
    /// assert_eq!("MULTIPOLYGON (((0 0, 0 1, 1 1, 1 0, 0 0)), ((1 1, 1 2, 2 2, 2 1, 1 1)))", multi_polygon.to_string());
    /// ```
    pub fn new(polygons: Vec<Polygon>) -> Self {
        MultiPolygon(polygons)
    }
}

implement_deref!(MultiPolygon, Vec<Polygon>);

impl fmt::Display for MultiPolygon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let polygons = self
            .iter()
            .map(|polygon| {
                polygon
                    .iter()
                    .map(|ring| {
                        ring.iter().format_with(", ", |point, f| {
                            f(&format_args!("{} {}", point[0], point[1]))
                        })
                    })
                    .format_with(", ", |ring, f| f(&format_args!("({})", ring)))
            })
            .format_with(", ", |polygon, f| f(&format_args!("({})", polygon)));
        write!(f, "MULTIPOLYGON ({})", polygons)
    }
}

impl<T: NumCast> TryFrom<Vec<Vec<Vec<[T; 2]>>>> for MultiPolygon {
    type Error = GeometryError;

    /// Tries to convert a vector of vectors of vectors of 2-float arrays into a `MultiPolygon`.
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use auto_gis_with_rust::polygon::MultiPolygon;
    ///
    /// let multi_polygon_1 = MultiPolygon::try_from(vec![
    ///     vec![
    ///         vec![[0., 0.], [0., 1.], [1., 1.], [1., 0.], [0., 0.]],
    ///     ],
    ///     vec![
    ///         vec![[1., 1.], [1., 2.], [2., 2.], [2., 1.]],
    ///     ],
    /// ]).unwrap();
    ///
    /// assert_eq!("MULTIPOLYGON (((0 0, 0 1, 1 1, 1 0, 0 0)), ((1 1, 1 2, 2 2, 2 1, 1 1)))", multi_polygon_1.to_string());
    /// ```
    ///
    /// Or tries to convert a vector of vectors of vectors of 2-integer arrays into a `MultiPolygon`.
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use auto_gis_with_rust::polygon::MultiPolygon;
    /// #    
    /// # let multi_polygon_1 = MultiPolygon::try_from(vec![
    /// #     vec![
    /// #         vec![[0., 0.], [0., 1.], [1., 1.], [1., 0.], [0., 0.]],
    /// #     ],
    /// #     vec![
    /// #         vec![[1., 1.], [1., 2.], [2., 2.], [2., 1.]],
    /// #     ],
    /// # ]).unwrap();
    ///
    /// let multi_polygon_2 = MultiPolygon::try_from(vec![
    ///     vec![
    ///         vec![[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]],
    ///     ],
    ///     vec![
    ///         vec![[1, 1], [1, 2], [2, 2], [2, 1]],
    ///     ],
    /// ]).unwrap();
    ///
    /// assert_eq!(multi_polygon_1, multi_polygon_2);
    /// ```
    fn try_from(vectors: Vec<Vec<Vec<[T; 2]>>>) -> Result<Self, GeometryError> {
        let polygons: Result<Vec<Polygon>, GeometryError> =
            vectors.into_iter().map(Polygon::new).collect();
        Ok(MultiPolygon::new(polygons?))
    }
}
