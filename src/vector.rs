use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Default for Vec2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self { 
        Self { x, y }
    }

    pub fn floor(&self) -> Self {
        Self { x: self.x.floor(), y: self.y.floor() }
    }

    pub const fn splat(val: f32) -> Self {
        Self { x: val, y: val }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn dot(&self, b: &Self) -> f32 {
        self.x * b.x + self.y * b.y
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        
        self.x /= magnitude;
        self.y /= magnitude;
    }
}

impl From<Vec4> for Vec2 {
    fn from(vector: Vec4) -> Self {
        Self::new(vector.x, vector.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl From<Vec4> for Vec3 {
    fn from(value: Vec4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z
        }
    }
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self { 
        Self { x, y, z }
    }

    pub const fn splat(val: f32) -> Self {
        Self { x: val, y: val, z: val }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn cross(&self, b: &Self) -> Self {
        Vec3::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x 
        )
    }

    pub fn dot(&self, b: &Self) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn rotated_x(&self, angle: f32) -> Self {
        Self {
            x: self.x,
            y: self.y * angle.cos() - self.z * angle.sin(),
            z: self.y * angle.sin() + self.z * angle.cos()
        }
    }

    pub fn rotated_y(&self, angle: f32) -> Self {
        Self {
            x: self.x * angle.cos() - self.z * angle.sin(),
            y: self.y,
            z: self.x * angle.sin() + self.z * angle.cos()
        }
    }

    pub fn rotated_z(&self, angle: f32) -> Self {
        Self {
            x: self.x * angle.cos() - self.y * angle.sin(),
            y: self.x * angle.sin() + self.y * angle.cos(),
            z: self.z
        }
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();

        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self { 
        Self { x, y, z, w }
    }

    pub const fn splat(val: f32) -> Self {
        Self { x: val, y: val, z: val, w: val }
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)
    }

    pub fn dot(&self, b: Self) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();

        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
        self.w /= magnitude;
    }
}

impl From<Vec3> for Vec4 {
    fn from(value: Vec3) -> Self {
        Self::new(value.x, value.y, value.z, 1.0)
    }
}

impl Default for Vec4 {
    fn default() -> Self {
        Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Vec4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}