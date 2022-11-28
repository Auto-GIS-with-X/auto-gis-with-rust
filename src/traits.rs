use crate::point::Point;

pub trait Geometry {
    fn centroid(&self) -> Point;

    fn is_simple(&self) -> bool;
}

pub trait GeometryCollection<T: Geometry> {
    fn num_geometries(&self) -> usize;

    fn geometry_n(&self, number: usize) -> T;
}

pub trait Curve {
    fn length(&self) -> f64;

    fn start_point(&self) -> Point;

    fn end_point(&self) -> Point;

    fn is_closed(&self) -> bool;

    fn is_ring(&self) -> bool;
}

pub trait LineString {
    fn num_points(&self) -> usize;

    fn point_n(&self, number: usize) -> Point;
}
