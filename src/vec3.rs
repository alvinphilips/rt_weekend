#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.magnitude_squared())
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn normalized(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let (x, y, z): (f64, f64, f64) = rand::random();
        Vec3(
            map_01(x, min, max),
            map_01(y, min, max),
            map_01(z, min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let vector = Vec3::random_range(-1.0, 1.0);
            if vector.magnitude_squared() < 1.0 {
                return vector;
            };
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let (x, y): (f64, f64) = rand::random();
            let x = map_01(x, -1.0, 1.0);
            let y = map_01(y, -1.0, 1.0);
            let vector = Vec3(x, y, 0.0);
            if vector.magnitude_squared() < 1.0 {
                return vector;
            };
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().normalized()
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refract(uv: &Vec3, normal: &Vec3, etai_over_etat: f64) -> Self {
        let uv = *uv;
        let normal = *normal;
        let cos_theta = f64::min(Vec3::dot(&(-uv), &normal), 1.0);
        let ray_out_perpendicular = etai_over_etat * (uv + cos_theta * normal);
        let ray_out_parallel =
            -f64::sqrt(f64::abs(1.0 - ray_out_perpendicular.magnitude_squared())) * normal;

        ray_out_perpendicular + ray_out_parallel
    }

    pub const RIGHT: Vec3 = Vec3(1.0, 0.0, 0.0);
    pub const UP: Vec3 = Vec3(0.0, 1.0, 0.0);
    pub const FORWARD: Vec3 = Vec3(0.0, 0.0, 1.0);
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl std::ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl rand::distributions::Distribution<Vec3> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        let (x, y, z): (f64, f64, f64) = rng.gen();
        Vec3(x, y, z)
    }
}

pub use Vec3 as Point3;

use crate::utils::map_01;
