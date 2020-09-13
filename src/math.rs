//! Contains math related structures and functions.

use amethyst::core::math::*;

/// Represents a point on the screen using screen coordinates.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScreenPoint {
    /// The X coordinate of the point.
    pub x: f32,
    /// The Y coordinate of the point.
    pub y: f32,
}

impl ScreenPoint {
    /// Constructor for making a new screen point.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<(f32, f32)> for ScreenPoint {
    /// Gets a screen point from a tuple.
    #[inline]
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

/// Holds a 2D line.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Line {
    start: Point3<f32>,
    end: Point3<f32>,
}

impl Line {
    pub fn new(start: Point3<f32>, end: Point3<f32>) -> Self {
        Self { start, end }
    }

    /// Gets the start point of the line.
    pub fn start(&self) -> Point3<f32> {
        self.start
    }

    /// Gets the end point of the line.
    pub fn end(&self) -> Point3<f32> {
        self.end
    }
}

/// Represents an axis aligned rectangle with equal length sides.
///
/// This is useful for representing tic-tac-toe squares.
///
/// **Note**: The square is defined using world coordinates.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Square {
    // The square's center point.
    center: Point3<f32>,
    // Half of the square's with or height.
    // Storing half the size makes the math for the accessor functions easier.
    half_size: f32,
}

impl Square {
    /// Builds a new square from the provided center point and size.
    ///
    /// # Panics
    /// The size of the square must be zero or greater.
    pub fn new(center: Point3<f32>, size: f32) -> Self {
        assert!(size >= 0.0, "The size of the square cannot be negative.");
        Self {
            center,
            half_size: size / 2.0,
        }
    }

    /// Gets the square's center point.
    pub fn center(&self) -> Point3<f32> {
        self.center
    }

    /// Gets the square's size -- that is the width or height, of the square.
    pub fn size(&self) -> f32 {
        self.half_size * 2.0
    }

    /// Gets the top of the square directly above the center point.
    pub fn top_center(&self) -> Point3<f32> {
        Point3::new(self.center.x, self.center.y + self.half_size, self.center.z)
    }

    /// Gets the bottom of the square directly below the center point.
    pub fn bottom_center(&self) -> Point3<f32> {
        Point3::new(self.center.x, self.center.y - self.half_size, self.center.z)
    }

    /// Gets the left size of the square directly to the left of the center point.
    pub fn center_left(&self) -> Point3<f32> {
        Point3::new(self.center.x - self.half_size, self.center.y, self.center.z)
    }

    /// Gets the right side of the square directly to the right of the center point.
    pub fn center_right(&self) -> Point3<f32> {
        Point3::new(self.center.x + self.half_size, self.center.y, self.center.z)
    }

    /// Gets the top left corner of the square.
    pub fn top_left(&self) -> Point3<f32> {
        Point3::new(
            self.center.x - self.half_size,
            self.center.y + self.half_size,
            self.center.z,
        )
    }

    /// Gets the top right corner of the square.
    pub fn top_right(&self) -> Point3<f32> {
        Point3::new(
            self.center.x + self.half_size,
            self.center.y + self.half_size,
            self.center.z,
        )
    }

    /// Gets the bottom left corner of the square.
    pub fn bottom_left(&self) -> Point3<f32> {
        Point3::new(
            self.center.x - self.half_size,
            self.center.y - self.half_size,
            self.center.z,
        )
    }

    /// Gets the bottom right corner of the square.
    pub fn bottom_right(&self) -> Point3<f32> {
        Point3::new(
            self.center.x + self.half_size,
            self.center.y - self.half_size,
            self.center.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_ulps_eq;

    #[test]
    fn screen_point_from_tuple_should_get_x_from_first_element_and_y_from_second() {
        let x = 1.0;
        let y = 2.0;
        let tuple_point = (x, y);

        let screen_point = ScreenPoint::from(tuple_point);

        assert_ulps_eq!(screen_point.x, tuple_point.0);
        assert_ulps_eq!(screen_point.y, tuple_point.1);
    }

    #[test]
    #[should_panic]
    fn square_new_when_negative_size_should_panic() {
        let center_point = Point3::new(1.0, 2.0, 3.0);
        let size = -1.0;

        let _square = Square::new(center_point, size);
    }

    #[test]
    fn square_new_sets_center_point_and_size() {
        let center_point = Point3::new(1.0, 2.0, 3.0);
        let size = 4.0;

        let square = Square::new(center_point, size);

        assert_ulps_eq!(square.center(), center_point);
        assert_ulps_eq!(square.size(), size);
    }

    #[test]
    fn square_top_center_should_get_point_plus_y_above_center_point() {
        let size = 4.0;
        let half_size = size / 2.0;
        let center_point = Point3::new(1.0, 1.0, 3.0);
        let expected_top = Point3::new(center_point.x, center_point.y + half_size, center_point.z);

        let square = Square::new(center_point, size);

        assert_ulps_eq!(square.top_center(), expected_top);
    }

    #[test]
    fn square_bottom_center_should_get_point_minus_y_below_center_point() {
        let size = 4.0;
        let half_size = size / 2.0;
        let center_point = Point3::new(1.0, 1.0, 3.0);
        let expected_bottom =
            Point3::new(center_point.x, center_point.y - half_size, center_point.z);

        let square = Square::new(center_point, size);

        assert_ulps_eq!(square.bottom_center(), expected_bottom);
    }

    #[test]
    fn square_center_left_should_get_point_minus_x_from_center_point() {
        let size = 4.0;
        let half_size = size / 2.0;
        let center_point = Point3::new(1.0, 1.0, 3.0);
        let expected_left = Point3::new(center_point.x - half_size, center_point.y, center_point.z);

        let square = Square::new(center_point, size);

        assert_ulps_eq!(square.center_left(), expected_left);
    }

    #[test]
    fn square_center_right_should_get_point_plus_x_from_center_point() {
        let size = 4.0;
        let half_size = size / 2.0;
        let center_point = Point3::new(1.0, 1.0, 3.0);
        let expected_right =
            Point3::new(center_point.x + half_size, center_point.y, center_point.z);

        let square = Square::new(center_point, size);

        assert_ulps_eq!(square.center_right(), expected_right);
    }

    #[test]
    fn square_top_left_should_get_point_minus_x_and_plus_y_from_center_point() {
        let size = 4.0;
        let half_size = size / 2.0;
        let center_point = Point3::new(1.0, 2.0, 3.0);
        let expected_top_left = Point3::new(
            center_point.x - half_size,
            center_point.y + half_size,
            center_point.z,
        );

        let square = Square::new(center_point, size);

        assert_ulps_eq!(square.top_left(), expected_top_left);
    }

    #[test]
    fn square_top_right_should_get_point_plus_x_and_plus_y_from_center_point() {
        let size = 4.0;
        let half_size = size / 2.0;
        let center_point = Point3::new(1.0, 2.0, 3.0);
        let expected_top_right = Point3::new(
            center_point.x + half_size,
            center_point.y + half_size,
            center_point.z,
        );

        let square = Square::new(center_point, size);

        assert_ulps_eq!(square.top_right(), expected_top_right);
    }

    #[test]
    fn square_bottom_left_should_get_point_minus_x_and_minux_y_from_center_point() {
        let size = 4.0;
        let half_size = size / 2.0;
        let center_point = Point3::new(1.0, 2.0, 3.0);
        let expected_bottom_left = Point3::new(
            center_point.x - half_size,
            center_point.y - half_size,
            center_point.z,
        );

        let square = Square::new(center_point, size);

        assert_ulps_eq!(square.bottom_left(), expected_bottom_left);
    }

    #[test]
    fn square_bottom_right_should_get_point_plus_x_and_minus_y_from_center_point() {
        let size = 4.0;
        let half_size = size / 2.0;
        let center_point = Point3::new(1.0, 2.0, 3.0);
        let expected_bottom_right = Point3::new(
            center_point.x + half_size,
            center_point.y - half_size,
            center_point.z,
        );

        let square = Square::new(center_point, size);

        assert_ulps_eq!(square.bottom_right(), expected_bottom_right);
    }

    #[test]
    fn line_new_should_set_fields() {
        let start = Point3::new(1.0, 2.0, 3.0);
        let end = Point3::new(4.0, 5.0, 6.0);

        let line = Line::new(start, end);

        assert_eq!(line.start(), start);
        assert_eq!(line.end(), end);
    }
}
