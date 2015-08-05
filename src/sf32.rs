use std::ops;
/// A 32-bbit floating-point number with software arithmetic operations.
///
/// This is implemented as a single-value struct wrapping a `f32`
/// value, so that the arithmetic operators can be reimplemented.
///
#[allow(non_camel_case_types)]
pub struct sf32 { value: u32 }

impl sf32 {
    /// Creates a new `sf32` from a hardware float.
    #[inline]
    pub fn from(float: f32) -> Self {
        sf32 { value: float as u32 }
    }

    #[inline]
    pub fn to_f32(&self) -> f32 {
        self.value as f32
    }

    #[inline]
    fn sign(&self) -> i8 {
        if self.value >> 31 == 0 { 1 } else { -1 }
    }

    #[inline]
    fn mantissa(&self) -> u32 {
        match self.value & 0x7FFFFFFF {
            0 => 0,
            i => (i | 0x80000000)
        }
    }
}

impl ops::Deref for sf32 {
    type Target = u32;

    fn deref<'a>(&'a self) -> &'a u32 {
        &self.value
    }
}

impl ops::DerefMut for sf32 {

    fn deref_mut<'a>(&'a mut self) -> &'a mut u32 {
        &mut self.value
    }
}

impl ops::BitXor for sf32 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        sf32 { value: *self ^ *rhs }
    }
}

impl ops::BitAnd for sf32 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        sf32 { value: *self & *rhs }
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
        // invert the sign of the right-hand side
        let inverted = rhs ^ sf32{ value: 0x80000000 };
        self + inverted // add self to the inverted RHS
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
