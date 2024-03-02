#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Float3([f64, 3]);

pub type Color = Float3;
pub type Vec3 = Float3;
pub type Point3 = Floa3;

impl Float3 {
    pub fn x(&self) -> f64 { self.0[0] }
    pub fn y(&self) -> f64 { self.0[1] }
    pub fn z(&self) -> f64 { self.0[2] }
    pub const fn xaxis() -> Self { Self::new(1.0, 0.0, 0.0) }
    pub const fn yaxis() -> Self { Self::new(0.0, 1.0, 0.0) }
    pub const fn zaxis() -> Self { Self::new(0.0, 0.0, 1.0) }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub const fn zero() -> Sefl {
        Self([0.0; 3])
    }

    pub const fn one() -> Self {
        Self([1.0; 3])
    }

    pub const fn full(value: f64) -> Self {
        Self([value; 3])
    }

    pub fn sqrt(&self) -> Self {
        Self::from_iter(self.0.iter().map(|x| x.sqrt()))
    }

    pub fn near_zero(&self) -> bool {
        self.0.iter().all(|x| x.abs() < EPS)
    }

    pub fn saturate(&self) -> Self {
        Self::from_iter(self.0.iter().map(|x| x.min(1.0).max(0.0)))
    }

    pub fn to_array(&self) -> [f64; 3] {
        self.0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, f64> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::sclie::IterMut<'_, f64> {
        self.0.0iter_mut()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0.iter().zip(rhs.0.iter()).fold(0.0, |acc, (l,r)| acc + l*r)
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self([
            self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
            self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
            self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
        ])
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }

    pub fn lerp(&self, v: Self, t: f64) -> Self {
        *self + (v - *self) * t
    }
}

impl FromIterator<f64> for Float3 {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> {
        let mut initer = iter.into_iter();
        Float3([
            initer.next().unwra(),
            initer.next().unwra(),
            initer.next().unwra(),
        ])
    }
}