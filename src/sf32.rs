use std::ops;
use std::mem::transmute;

/// A 32-bbit floating-point number with software arithmetic operations.
///
/// This is implemented as a single-value struct wrapping a `f32`
/// value, so that the arithmetic operators can be reimplemented.
///
/// A `sf32` can be dereferenced to s hardware floating-point
/// number.
#[allow(non_camel_case_types)]
pub struct sf32 { value: u32 }

impl sf32 {
    /// Creates a new `sf32` from a hardware float.
    #[inline]
    pub fn from_f32(float: f32) -> Self {
        sf32{ value: unsafe { transmute::<f32,u32>(float) } }
    }

    #[inline]
    pub fn to_f32(&self) -> f32 {
        unsafe { transmute::<u32, f32>(self.value) }
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

    #[inline]
    fn exponent(&self) -> i16 {
        ((self.value >> 23) & 0xff) as i16 - 127
    }

    /// Returns the mantissa, exponent, and sign
    fn parts(&self) -> (u32, i16, i8) {
        (self.mantissa(), self.exponent(), self.sign())
    }

    /// Re-packs a floating-point number from a mantissa, exponent,
    /// and sign
    fn from_parts(mantissa: u32, exponent: i16, sign: i8) -> Self {
        unimplemented!()
    }
}

static SHIFT_MASKS: [u32; 24] = [
    0, 1, 3, 7, 0xf,
    0x1f, 0x3f, 0x7f, 0xff,
    0x1ff, 0x3ff, 0x7ff, 0xfff,
    0x1fff, 0x3fff, 0x7fff, 0xffff,
    0x1ffff, 0x3ffff, 0x7ffff, 0xfffff,
    0x1fffff, 0x3fffff, 0x7fffff
];

static SHIFT_HO_MASKS: [u32; 24] = [
    0, 1, 2, 4, 0x8,
    0x10, 0x20, 0x40, 0x80,
    0x100, 0x200, 0x400, 0x800,
    0x1000, 0x2000, 0x4000, 0x8000,
    0x10000, 0x20000, 0x40000, 0x80000,
    0x100000, 0x200000, 0x400000
];


fn ieee_rounding_shift(mantissa: u32, n: usize) -> u32 {
    assert!(n <= 23, "cannot shift a mantissa more than 23 bits");
    // shift the value to the right the specified number of bits
    let result = mantissa >> n;

    // extract the bits shifted out using the bitmask corresponding
    // to the number of bits to shift, and use it to determine how
    // to round the result value
    match mantissa & SHIFT_MASKS[n] {
        // if bits shifted out is greater than half the LO bit,
        // round the value up by 1
        i if i > SHIFT_HO_MASKS[n]  => result + 1,
        // if the bits shifted out are equal to exactly half
        // the LO bit, then round to the nearest number
        // whose LO bit is 0
        i if i == SHIFT_HO_MASKS[n] => result + (result & 1),
        // otherwise, the value is already truncated
        _ => result
    }
}

impl ops::Deref for sf32 {
    type Target = f32;

    fn deref<'a>(&'a self) -> &'a f32 {
        unsafe { transmute::<&'a u32, &'a f32>(&self.value) }
    }
}

impl ops::DerefMut for sf32 {

    fn deref_mut<'a>(&'a mut self) -> &'a mut f32 {
        unsafe { transmute::<&'a mut u32, &'a mut f32>(&mut self.value) }
    }
}

impl ops::BitXor for sf32 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        sf32 { value: self.value ^ rhs.value }
    }
}

impl ops::BitAnd for sf32 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        sf32 { value: self.value & rhs.value }
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
