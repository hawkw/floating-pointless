use std::ops;

/// A floating-point number with software arithmetic operations.
///
/// This is implemented as a single-value struct wrapping a `f64`
/// value, so that the arithmetic operators can be reimplemented.
///
/// A SoftFloat can be dereferenced to s hardware floating-point
/// number.
pub struct SoftFloat { value: f64 }

impl ops::Deref for SoftFloat {
    type Target = f64;
    fn deref<'a>(&'a self) -> &'a f64 {
        &self.value
    }
}

impl ops::DerefMut for SoftFloat {
    fn deref_mut<'a>(&'a mut self) -> &'a mut f64 {
        &mut self.value
    }
}

impl SoftFloat {
    /// Creates a new `SoftFloat` from a hardware float.
    pub fn from(float: f64) -> Self {
        SoftFloat { value: float }
    }

    /// Access the raw binary value of a SoftFloat.
    fn raw_value(&self) -> u64 {
        self.value as u64
    }
}

#[test]
fn it_works() {
}
