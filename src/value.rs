use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Rem, RemAssign};

#[derive(Clone, Copy)]
pub(crate) struct Value {
    pub(crate) data: i32,
}

fn power(a: i32, b: i32) -> i32{
    match b {
        0 => 1,
        _ => a * power(a, b - 1)
    }
}

impl Value {
    pub(crate) fn pow(&mut self, p: i32) {
        *self = Self {
            data: power(self.data, p)
        }
    }
}

impl From<i32> for Value {
    fn from(data: i32) -> Self {
        Value { data }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, x: Value) -> Value {
        Value {
            data: self.data + x.data,
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, x: Value) -> Value {
        Value {
            data: self.data - x.data,
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, x: Value) -> Value {
        Value {
            data: self.data * x.data,
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, x: Value) -> Value {
        Value {
            data: self.data / x.data,
        }
    }
}

impl Rem for Value {
    type Output = Value;

    fn rem(self, x: Value) -> Value {
        Value {
            data: self.data % x.data
        }
    }
}

impl AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            data: self.data + rhs.data
        }
    }
}

impl SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            data: self.data - rhs.data
        }
    }
}

impl MulAssign for Value {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            data: self.data * rhs.data
        }
    }
}

impl DivAssign for Value {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self {
            data: self.data / rhs.data
        }
    }
}

impl RemAssign for Value {
    fn rem_assign(&mut self, rhs: Self) {
        *self = Self {
            data: self.data % rhs.data
        }
    }
}