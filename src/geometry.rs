use crate::point::Point;

pub trait Geometry {
    fn centroid(&self) -> Point;
}

pub trait GeometryCollection<T: Geometry> {
    fn num_geometries(&self) -> usize;

    fn geometry_n(&self, number: usize) -> T;
}
