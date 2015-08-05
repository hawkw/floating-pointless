use std::ops;
use std::mem::transmute;

/// A 64-bbit floating-point number with software arithmetic operations.
///
/// This is implemented as a single-value struct wrapping a `f64`
/// value, so that the arithmetic operators can be reimplemented.
///
/// A `sf64` can be dereferenced to s hardware floating-point
/// number.
#[allow(non_camel_case_types)]
pub struct sf64 { value: u64 }

impl sf64 {
    /// Creates a new `sf64` from a hardware float.
    #[inline]
    pub fn from(float: f64) -> Self {
        sf64 { value: float as u64 }
    }

    #[inline]
    pub fn to_f64(&self) -> f64 {
        self.value as f64
    }

    #[inline]
    fn sign(&self) -> i8 {
        if self.value >> 31 == 0 { 1 } else { -1 }
    }

    #[inline]
    fn mantissa(&self) -> u64 {
        match self.value & 0x7FFFFFFF {
            0 => 0,
            i => (i | 0x80000000)
        }
    }
}

impl ops::Deref for sf64 {
    type Target = f64;

    fn deref<'a>(&'a self) -> &'a f64 {
        unsafe { transmute::<&'a u64, &'a f64>(&self.value) }
    }
}

impl ops::DerefMut for sf64 {

    fn deref_mut<'a>(&'a mut self) -> &'a mut f64 {
        unsafe { transmute::<&'a mut u64, &'a mut f64>(&mut self.value) }
    }
}

impl ops::BitXor for sf64 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        sf64 { value: self.value ^ rhs.value }
    }
}

impl ops::BitAnd for sf64 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        sf64 { value: self.value & rhs.value }
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
        // invert the sign of the right-hand side
        let inverted = rhs ^ sf64{ value: 0x80000000 };
        self + inverted // add self to the inverted RHS
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
