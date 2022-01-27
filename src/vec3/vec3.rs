use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub e: Vec<f32>,
}

impl Vec3 {
    // constructor
    pub fn new(first: f32, second: f32, third: f32) -> Vec3 {
        let e = vec![first, second, third];
        Vec3 { e }
    }

    // getters
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    pub fn r(&self) -> i32 {
        (255.99 * self.e[0].sqrt()) as i32
    }
    pub fn g(&self) -> i32 {
        (255.99 * self.e[1].sqrt()) as i32
    }
    pub fn b(&self) -> i32 {
        (255.99 * self.e[2].sqrt()) as i32
    }

    pub fn length(&mut self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn squared_length(&mut self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(&mut self) -> Vec3 {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;

        self.clone()
    }

    pub fn dot(&mut self, another: Vec3) -> f32 {
        self.e[0] * another.e[0] + self.e[1] * another.e[1] + self.e[2] * another.e[2]
    }

    pub fn cross(&mut self, another: Vec3) -> Vec3 {
        let e1 = self.e[0];
        let e2 = self.e[1];
        let e3 = self.e[2];
        let e4 = another.e[0];
        let e5 = another.e[1];
        let e6 = another.e[2];

        Vec3::new(e2 * e6 - e3 * e5, e3 * e4 - e1 * e6, e1 * e5 - e2 * e4)
    }


}

// operation override

// -<Vec3>
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

// <Vec3> + <Vec3>
impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

// <Vec3> - <Vec3>
impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

// <Vec3> * <Vec3>
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

// <Vec3> / <Vec3>
impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] / other.e[0],
            self.e[1] / other.e[1],
            self.e[2] / other.e[2],
        )
    }
}

// <Vec3>[]
impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        &self.e[index]
    }
}

// <Vec3>[] mut
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.e[index]
    }
}

// <Vec3> * <f32>
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.e[0] * other, self.e[1] * other, self.e[2] * other)
    }
}

// <f32> * <Vec3>
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.e[0], self * other.e[1], self * other.e[2])
    }
}

// <Vec3> / <f32>
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.e[0] / other, self.e[1] / other, self.e[2] / other)
    }
}

// <f32> / <Vec3>
impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self / other.e[0], self / other.e[1], self / other.e[2])
    }
}

// <Vec3> += <Vec3>
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

// <Vec3> -= <Vec3>
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

// <Vec3> *= <Vec3>
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

// <Vec3> *= <f32>
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

// <Vec3> /= <Vec3>
impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

// <Vec3> /= <f32>
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

// test for Vec3
#[test]
fn test_vec3_length() {
    let mut v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.length(), 3.7416573867739413);
}
#[test]
fn test_vec3_neg() {
    let vec3: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(-vec3, Vec3::new(-1.0, -2.0, -3.0));
}
#[test]
fn test_vec3_add() {
    let vec3_1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let vec3_2: Vec3 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(vec3_1 + vec3_2, Vec3::new(5.0, 7.0, 9.0));
}
#[test]
fn test_vec3_sub() {
    let vec3_1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let vec3_2: Vec3 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(vec3_1 - vec3_2, Vec3::new(-3.0, -3.0, -3.0));
}
#[test]
fn test_vec3_mul() {
    let vec3_1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let vec3_2: Vec3 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(vec3_1 * vec3_2, Vec3::new(4.0, 10.0, 18.0));
}
#[test]
fn test_vec3_div() {
    let vec3_1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let vec3_2: Vec3 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(vec3_1 / vec3_2, Vec3::new(0.25, 0.4, 0.5));
}
#[test]
fn test_vec3_index() {
    let vec3: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(vec3[0], 1.0);
    assert_eq!(vec3[1], 2.0);
    assert_eq!(vec3[2], 3.0);
}
#[test]
fn test_vec3_index_mut() {
    let mut vec3: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    vec3[0] = 4.0;
    assert_eq!(vec3[0], 4.0);
}
#[test]
fn test_vec3_mul_f32() {
    let vec3: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(vec3 * 2.0, Vec3::new(2.0, 4.0, 6.0));
}
#[test]
fn test_vec3_div_f32() {
    let vec3: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(vec3 / 2.0, Vec3::new(0.5, 1.0, 1.5));
}
#[test]
fn test_vec3_dot() {
    let mut vec3_1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let vec3_2: Vec3 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(vec3_1.dot(vec3_2), 32.0);
}
#[test]
fn test_vec3_cross() {
    let mut vec3_1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
    let vec3_2: Vec3 = Vec3::new(4.0, 5.0, 6.0);
    assert_eq!(vec3_1.cross(vec3_2), Vec3::new(-3.0, 6.0, -3.0));
}