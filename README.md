# auto-gis-with-rust

Solving the AutoGIS 2021 exercises with Rust.

## Problem 1: Creating basic geometries

### Part 1: Create a function called `create_point_geom()` that has two parameters (`x_coord`, `y_coord`). Function should create a `shapely` `Point` geometry object and return that.

- Rust Point struct
- GeoArrow in memory format
- Rust new type pattern

```rust
// main.rs

#[derive(Debug)]
pub struct Point([f64; 2]);

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point([x, y])
    }
}

fn main() {
    let point = Point::new(0.0, 1.0);
    dbg!(point);
}
```

- Handle any number with `num-traits`

```sh
cargo add num-traits
```

```diff
# main.rs

+ use num_traits::{self, NumCast};

/#[derive(Debug)]
pub struct Point([f64; 2]);

impl Point {
-    fn new(x: f64, y: f64) -> Self {
+    fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
+        let x_float: f64 = num_traits::cast(x).unwrap();
+        let y_float: f64 = num_traits::cast(y).unwrap();
-        Point([x, y])
+        Point([x_float, y_float])
    }
}

fn main() {
    let point = Point::new(0, 1);
    dbg!(point);
}
```

- Add unit testing and documentation with doctest

```diff
# lib.rs

use num_traits::{self, NumCast};

- #[derive(Debug)]
+ #[derive(Debug, PartialEq, PartialOrd)]
pub struct Point([f64; 2]);

impl Point {
+    /// Construct a new `Point`.
+    ///
+    /// # Examples:
+    ///
+    /// Construct a new point from x and y floats or x and y integers.
+    ///
+    /// ```
+    /// use auto_gis_with_rust::Point;
+    ///
+    /// let point_0 = Point::new(0.0, 1.0);
+    /// let point_1 = Point::new(0, 1);
+    ///
+    /// assert_eq!(point_0, point_1);
+    /// ```
    pub fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
        let x_float: f64 = num_traits::cast(x).unwrap();
        let y_float: f64 = num_traits::cast(y).unwrap();
        Point([x_float, y_float])
    }
}
```

```diff
# main.rs

- use num_traits::{self, NumCast};
+ use auto_gis_with_rust::Point;

- /#[derive(Debug)]
- pub struct Point([f64; 2]);

- impl Point {
-    fn new<T: NumCast, U: NumCast>(x: T, y: U) -> Self {
-       let x_float: f64 = num_traits::cast(x).unwrap();
-       let y_float: f64 = num_traits::cast(y).unwrap();
-        Point([x_float, y_float])
-    }
- }

fn main() {
    let point = Point::new(0, 1);
    dbg!(point);
}
```