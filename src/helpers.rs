use num_traits::{self, NumCast};

/// Convert a vector of two-item arrays of generics that implement `NumCast` into a vector of two-item arrays of floats.
///
/// Examples:
///
/// ```
/// use auto_gis_with_rust::helpers::get_float_coordinates;
///
/// let output = get_float_coordinates(vec![[0, 0], [0, 1], [1, 1]]);
/// let expected = vec![[0., 0.], [0., 1.], [1., 1.]];
///
/// assert_eq!(output, expected)
/// ```
pub fn get_float_coordinates<T: NumCast>(coordinates: Vec<[T; 2]>) -> Vec<[f64; 2]> {
    let float_coordinates: Vec<[f64; 2]> = coordinates
        .into_iter()
        .map(|coordinate| {
            coordinate.map(|coordinate| -> f64 { num_traits::cast(coordinate).unwrap() })
        })
        .collect();
    float_coordinates
}
