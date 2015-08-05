use std::ops;
/// A 64-bbit floating-point number with software arithmetic operations.
///
/// This is implemented as a single-value struct wrapping a `f64`
/// value, so that the arithmetic operators can be reimplemented.
///
/// A `sf64` can be dereferenced to s hardware floating-point
/// number.
pub struct sf64 { value: f64 }

impl sf64 {
    /// Creates a new `sf64` from a hardware float.
    #[inline]
    pub fn from(float: f64) -> Self {
        sf64 { value: float }
    }

    /// Access the raw binary value of a sf64.
    #[inline]
    fn raw_value(&self) -> u64 {
        self.value as u64
    }
}

impl ops::Deref for sf64 {
    type Target = f64;
    fn deref<'a>(&'a self) -> &'a f64 {
        &self.value
    }
}

impl ops::DerefMut for sf64 {
    fn deref_mut<'a>(&'a mut self) -> &'a mut f64 {
        &mut self.value
    }
}

impl ops::Add for sf64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl ops::Sub for sf64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl ops::Div for sf64 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl ops::Mul for sf64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unimplemented!()
    }
}
