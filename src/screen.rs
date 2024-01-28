use crate::point3d::Point3D;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Screen {
    origin: Point3D,
    // note that x_vec and y_vec are not necessarily orthogonal
    x_vec: Vec3D,
    y_vec: Vec3D,
    // z_vec is cross product of y_vec and x_vec
    z_vec: Vec3D,
}

impl Screen {
    pub fn new(top_left: Point3D, bottom_left: Point3D, bottom_right: Point3D) -> Result<Self, &'static str> {
        if top_left == bottom_left || top_left == bottom_right || bottom_left == bottom_right {
            return Err("Screen points must be distinct");
        }
        let x_vec = Vec3D {
            x: bottom_right.x - bottom_left.x,
            y: bottom_right.y - bottom_left.y,
            z: bottom_right.z - bottom_left.z,
        };
        let y_vec = Vec3D {
            x: top_left.x - bottom_left.x,
            y: top_left.y - bottom_left.y,
            z: top_left.z - bottom_left.z,
        };
        let z_vec = y_vec.cross(&x_vec);
        Ok(
            Self {
                origin: bottom_left,
                x_vec,
                y_vec,
                z_vec,
            }
        )
    }

    // calculate the intercept of a vector starting at some point and the screen in screen space
    pub fn intercept(&self, x0: f64, y0: f64, z0: f64, dx: f64, dy: f64, dz: f64) -> (f64, f64) {
        let t = (self.z_vec.x * (self.origin.x - x0)
            + self.z_vec.y * (self.origin.y - y0)
            + self.z_vec.z * (self.origin.z - z0))
            / (self.z_vec.x * dx + self.z_vec.y * dy + self.z_vec.z * dz);
        let x = x0 + dx * t - self.origin.x;
        let y = y0 + dy * t - self.origin.y;
        let z = z0 + dz * t - self.origin.z;
        let intercept = Vec3D::new(x, y, z);

        // solve screen space coordinates using least squares
        let a = self.x_vec.dot(&self.x_vec);
        let b = self.x_vec.dot(&self.y_vec);
        let c = self.y_vec.dot(&self.y_vec);
        let det = 1.0 / (a * c - b * b);
        let b1 = self.x_vec.dot(&intercept);
        let b2 = self.y_vec.dot(&intercept);
        let x = (c * b1 - b * b2) * det;
        let y = (a * b2 - b * b1) * det;
        (x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_intercept() {
        let screen = Screen::new(
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
        );
        let (x, y) = screen.intercept(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);

        let (x, y) = screen.intercept(0.0, 0.0, 1.0, 0.0, 0.0, -1.0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);

        let (x, y) = screen.intercept(1.0, 0.0, 1.0, 0.0, 0.0, -1.0);
        assert_eq!(x, 1.0);
        assert_eq!(y, 0.0);

        let screen = Screen::new(
            Point3D::new(2.0, 1.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(2.0, 0.0, 0.0),
        );

        let (x, y) = screen.intercept(2.0, 1.0, 1.0, 0.0, 0.0, -1.0);
        println!("x: {}, y: {}", x, y);
        assert_eq!(x, 0.0);
        assert_eq!(y, 1.0);
    }

    fn intercept_fuzz() {
        let mut rng = rand::thread_rng();
        let top_left = Point3D::new(
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
        );
        let bottom_left = Point3D::new(
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
        );
        let bottom_right = Point3D::new(
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
        );
        let screen = Screen::new(
            top_left,
            bottom_left,
            bottom_right,
        );

        let x_vec = Vec3D::new(
            bottom_right.x - bottom_left.x,
            bottom_right.y - bottom_left.y,
            bottom_right.z - bottom_left.z,
        );
        let y_vec = Vec3D::new(
            top_left.x - bottom_left.x,
            top_left.y - bottom_left.y,
            top_left.z - bottom_left.z,
        );

        let x = rng.gen_range(-100.0..100.0);
        let y = rng.gen_range(-100.0..100.0);

        let random_start_point = Point3D::new(
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
        );

        let end_point = x_vec.scale(x).add(&y_vec.scale(y)).add(&Vec3D::new(
            bottom_left.x,
            bottom_left.y,
            bottom_left.z,
        ));

        let (x_hat, y_hat) = screen.intercept(
            random_start_point.x,
            random_start_point.y,
            random_start_point.z,
            end_point.x - random_start_point.x,
            end_point.y - random_start_point.y,
            end_point.z - random_start_point.z,
        );

        if x_hat - x > 0.00001 || y_hat - y > 0.00001 {
            println!("x: {}, y: {}", x, y);
            println!("x_hat: {}, y_hat: {}", x_hat, y_hat);
            println!("x - x_hat: {}, y - y_hat: {}", x - x_hat, y - y_hat);
            println!("x_vec: {:?}", x_vec);
            println!("y_vec: {:?}", y_vec);
            println!("random_start_point: {:?}", random_start_point);
            println!("end_point: {:?}", end_point);
            println!("top_left: {:?}", top_left);
            println!("bottom_left: {:?}", bottom_left);
            println!("bottom_right: {:?}", bottom_right);
            panic!("intercept failed");
        }
    }

    #[test]
    fn intercept_fuzz_test() {
        for _ in 0..1000 {
            intercept_fuzz();
        }
    }
}