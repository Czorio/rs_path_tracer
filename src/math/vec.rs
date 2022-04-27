use std::default;
use std::ops;

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    fn dot(self, rhs: Self) -> f64 {
        todo!()
    }

    fn cross(self, rhs: Self) -> Self {
        todo!()
    }

    fn length(self) -> f64 {
        todo!()
    }

    fn length_squared(self) -> f64 {
        todo!()
    }

    fn norm(self) -> Self {
        todo!()
    }
}

impl default::Default for Vec3 {
    fn default() -> Self {
        todo!()
    }
}

impl ops::Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        todo!()
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        todo!()
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        todo!()
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
        todo!()
    }

    fn cross(self, rhs: Self) -> Self {
        todo!()
    }

    fn length(self) -> f64 {
        todo!()
    }

    fn length_squared(self) -> f64 {
        todo!()
    }

    fn norm(self) -> Self {
        todo!()
    }
}

impl default::Default for Vec2 {
    fn default() -> Self {
        todo!()
    }
}

impl ops::Add<Self> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Sub<Self> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        todo!()
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        todo!()
    }
}

impl ops::Neg for Vec2 {
    type Output = ();

    fn neg(self) -> Self::Output {
        todo!()
    }
}
