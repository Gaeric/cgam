use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rand::Rng;

use crate::rtweekend::{random_double, random_double_range};

// some code reference glam
#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn x(self) -> f64 {
        self.x
    }

    pub fn y(self) -> f64 {
        self.y
    }

    pub fn z(self) -> f64 {
        self.z
    }

    pub fn index_mut(&mut self, index: usize) -> &mut f64 {
        if index == 0 {
            return &mut self.x;
        }

        if index == 1 {
            return &mut self.y;
        }

        return &mut self.z;
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, v: &Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut vec3 = Vec3::default();

        for index in 0..3 {
            let num = min + (max - min) * rng.gen::<f64>();
            *vec3.index_mut(index) = num;
        }

        return vec3;
    }

    pub fn random_random() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut vec3 = Vec3::default();
        for index in 0..3 {
            let num = rng.gen::<f64>();
            *vec3.index_mut(index) = num;
        }
        vec3
    }

    pub fn unit(&self) -> Vec3 {
        Vec3::new(
            self.x / self.length(),
            self.y / self.length(),
            self.z / self.length(),
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_random();
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p.unit();
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_range(-1.0, 1.0),
                random_double_range(-1.0, 1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn neal_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    // to be verify
    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        (*self - (2.0 * (self.dot(n))) * n).clone()
    }

    // to be verify
    pub fn refrace(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = -(*self).dot(n).min(1.0);
        let r_out_perp: Vec3 = etai_over_etat * (*self + cos_theta * n);
        let r_out_parallel: Vec3 = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        return r_out_perp + r_out_parallel;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x + &v.x, self.y + &v.y, self.z + &v.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Vec3 {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Vec3 {
        return self * (1.0 / rhs);
    }
}

pub type Point3 = Vec3;
