use std::ops;
use std::fmt;
use std::mem::transmute;

/// A 32-bbit floating-point number with software arithmetic operations.
///
/// This is implemented as a single-value struct wrapping a `f32`
/// value, so that the arithmetic operators can be reimplemented.
///
/// A `sf32` can be dereferenced to s hardware floating-point
/// number.
#[allow(non_camel_case_types)]
#[derive(Debug)]
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
        let sign_segmt: u32 = (sign as u32) << 31;
        let exp_segmt: u32 = (exponent as u32 + 127u32) << 23;
        sf32 { value: sign_segmt | exp_segmt | (mantissa & 0x7fffff) }
    }
}

impl fmt::Display for sf32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_f32())
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


fn ieee_rounding_shift(mantissa: u32, n: i16) -> u32 {
    assert!(n <= 23, "cannot shift a mantissa more than 23 bits");
    // shift the value to the right the specified number of bits
    let result = mantissa >> n;

    // extract the bits shifted out using the bitmask corresponding
    // to the number of bits to shift, and use it to determine how
    // to round the result value
    match mantissa & SHIFT_MASKS[n as usize] {
        // if bits shifted out is greater than half the LO bit,
        // round the value up by 1
        i if i > SHIFT_HO_MASKS[n as usize]  => result + 1,
        // if the bits shifted out are equal to exactly half
        // the LO bit, then round to the nearest number
        // whose LO bit is 0
        i if i == SHIFT_HO_MASKS[n as usize] => result + (result & 1),
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

    /// Add two floating-point numbers.
    ///
    /// Add two software floating-point numbers, returning a third
    /// floating-point number. Infinity and NaNs are propagated to
    /// the result.
    ///
    /// ## Example
    /// ```
    /// # use floating_pointless::sf32::sf32;
    /// let a: f32 = 1.0;
    /// let b: f32 = 2.0;
    /// let result = sf32::from_f32(a) + sf32::from_f32(b);
    /// assert_eq!(*result, a + b)
    /// ```
    fn add(self, rhs: Self) -> Self {
        let (lmantissa, lexp, lsign) = self.parts();
        let (rmantissa, rexp, rsign) = rhs.parts();
        match (lexp, lmantissa, rexp, rmantissa) {
            // both operands are infinity with the same signs, return infinity
            (127, 0, 127, 0) if rsign == lsign => self,
            // both operands are infinity with different signs, return NaN
            (127, 0, 127, 0) => sf32{ value: 0x7fc00000 },
            // left operand is NaN, propagate the same NaN
            (127, _, _, _) => self,
            // Right operand is NaN, propagate it
            (_, _, 127, _) => rhs,
            // Neither side is NaN, actually perform the add
            _ => {
                // denormalise the smaller mantissa
                let (dexp, rmant, lmant) = if rexp > lexp {
                    (rexp,
                     rmantissa,
                     ieee_rounding_shift(lmantissa, (rexp - lexp)))
                } else if rexp < lexp {
                    (lexp,
                     ieee_rounding_shift(rmantissa, (lexp - rexp)),
                     lmantissa)
                } else {
                    (rexp, rmantissa, lmantissa)
                };
                // add the mantissas
                let (dmant, dsign) = if rsign == lsign {
                    // if the signs are opposite, we actually subtract
                    if lmant > rmant {
                        (lmant - rmant, lsign)
                    } else {
                        (rmant - lmant, rsign)
                    }
                } else {
                    // if the signs are the same we add
                    (lmant + rmant, lsign)
                };
                // finally, we have to normalise the result
                if dmant >= 0x1000000 {
                    // round so the mantissa doesn't overflow into bit 24
                    sf32::from_parts(
                        ieee_rounding_shift(dmant, 1),
                        dexp + 1,
                        dsign)
                } else if dmant == 0 {
                    // if the mantissa is zero, just zero everything.
                    // this is faster and also prevents weird zeros that don't
                    // equal each other
                    sf32::from_parts(0,0,0)
                } else {
                    // if the HO bit is clear, normalise by shifting up
                    // and decrementing the exponent
                    let mut mant = dmant;
                    let mut exp = dexp;
                    while (mant < 0x800000) && (exp > -127) {
                        // TODO: find a better/more functional way to
                        // express this
                        mant = mant << 1;
                        exp = exp -1;
                    }
                    sf32::from_parts(mant, exp, dsign)

                }
            }
        }
    }
}

impl ops::Sub for sf32 {
    type Output = Self;
    /// Subtract two floating-point numbers.
    ///
    /// ## Example
    /// ```
    ///  # use floating_pointless::sf32::sf32;
    /// let a: f32 = 1.0;
    /// let b: f32 = 2.0;
    /// let result = sf32::from_f32(a) - sf32::from_f32(b);
    /// assert_eq!(*result, a - b)
    /// ```
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
