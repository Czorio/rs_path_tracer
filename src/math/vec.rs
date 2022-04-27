use std::default;
use std::ops;

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y + rhs.y + self.z * rhs.z
    }

    fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn norm(self) -> Self {
        let length = self.length();
        self / length
    }
}

impl default::Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ops::Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}



pub struct Vec2 {
    u: f64,
    v: f64,
}

impl Vec2 {
    fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }

    fn dot(self, rhs: Self) -> f64 {
        self.u * rhs.u + self.v * rhs.v
    }

    fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    fn length_squared(self) -> f64 {
        self.u * self.u + self.v * self.v
    }

    fn norm(self) -> Self {
        let length = self.length();
        Self {
            u: self.u / length,
            v: self.v / length
        }
    }
}

impl default::Default for Vec2 {
    fn default() -> Self {
        Self {
            u: 0.0,
            v: 0.0
        }
    }
}

impl ops::Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            u: self.u + rhs.u,
            v: self.v + rhs.v
        }
    }
}

impl ops::Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            u: self.u - rhs.u,
            v: self.v - rhs.v
        }
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            u: self.u * rhs,
            v: self.v * rhs
        }
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            u: self.u / rhs,
            v: self.v / rhs
        }
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            u: -self.u,
            v: -self.v
        }
    }
}
