use std::ops;
/// A 32-bbit floating-point number with software arithmetic operations.
///
/// This is implemented as a single-value struct wrapping a `f32`
/// value, so that the arithmetic operators can be reimplemented.
///
/// A `sf32` can be dereferenced to s hardware floating-point
/// number.
pub struct sf32 { value: f32 }

impl sf32 {
    /// Creates a new `sf32` from a hardware float.
    #[inline]
    pub fn from(float: f32) -> Self {
        sf32 { value: float }
    }

    /// Access the raw binary value of a sf32.
    #[inline]
    fn raw_value(&self) -> u32 {
        self.value as u32
    }
}

impl ops::Deref for sf32 {
    type Target = f32;
    fn deref<'a>(&'a self) -> &'a f32 {
        &self.value
    }
}

impl ops::DerefMut for sf32 {
    fn deref_mut<'a>(&'a mut self) -> &'a mut f32 {
        &mut self.value
    }
}

impl ops::Add for sf32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl ops::Sub for sf32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl ops::Div for sf32 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl ops::Mul for sf32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unimplemented!()
    }
}
