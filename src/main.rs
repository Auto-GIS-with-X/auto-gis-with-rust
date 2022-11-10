use auto_gis_with_rust::{error::GeometryError, polygon::Polygon};

fn main() -> Result<(), GeometryError> {
    let polygon = Polygon::new(vec![[0., 0.], [0., 1.], [1., 1.]])?;
    dbg!(polygon);
    Ok(())
}
