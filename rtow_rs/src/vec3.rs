use std::ops::{Add, AddAssign, DivAssign, Mul, MulAssign, Neg, Sub};

// some code reference glam
pub struct Vec3 {
    e: [f64; 3],
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    fn index(&self, i: usize) -> f64 {
        self.e[i]
    }

    fn x(self) -> f64 {
        return self.e[0];
    }

    fn y(self) -> f64 {
        return self.e[1];
    }

    fn z(self) -> f64 {
        return self.e[2];
    }

    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
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
        Vec3::new(self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2])
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.e[0], self * v.e[1], self * v.e[2])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 {
        t * self
    }
}
