use core::ops;

// TODO: color struct that implements debug
// TODO: implement new for Vec3
// TODO: figure out f32 and u64


pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    fn dotproduct(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn crossproduct(self, rhs: Self) -> Self {
        let x = self.y * rhs.z - self.z * rhs.y;
        let y = self.x * rhs.z - self.z * rhs.x;
        let z = self.x * rhs.y - self.y * rhs.x;
        Vec3 { x, y, z }
    }
    
    fn length_squared(&self) -> f32{
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> Option<f32>{
        Some(self.length_squared().sqrt())
    }

    fn unit(self) -> Option<Vec3> {
        if let Some(l) = self.length(){
            Some(Vec3 { x: self.x / l, y: self.y / l, z: self.z / l })
        } else {
            None
        }
    }

}

impl core::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl core::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl core::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl core::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl core::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
