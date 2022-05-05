use core::{
    cmp::Ordering,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub},
};

use super::{xdr::ScObjectType, Object, Val};

#[repr(transparent)]
#[derive(Clone)]
pub struct BigNum(Object);

impl TryFrom<Object> for BigNum {
    type Error = ();

    fn try_from(obj: Object) -> Result<Self, Self::Error> {
        if obj.is_type(ScObjectType::ScoBigint) {
            Ok(BigNum(obj))
        } else {
            Err(())
        }
    }
}

// impl TryFrom<Val> for BigNum {
//     type Error = ();

//     fn try_from(val: Val) -> Result<Self, Self::Error> {
//         let obj: Object = val.try_into()?;
//         if obj.is_type(ScObjectType::ScoBigint) {
//             Ok(BigNum(obj))
//         } else {
//             Err(())
//         }
//     }
// }

impl From<BigNum> for Object {
    fn from(b: BigNum) -> Self {
        b.0
    }
}

impl From<BigNum> for Val {
    fn from(b: BigNum) -> Self {
        b.0.into()
    }
}

// impl ObjType for BigNum {
//     fn is_obj_type(obj: Object) -> bool {
//         obj.is_type(ScObjectType::ScoBigint)
//     }

//     unsafe fn unchecked_from_obj(obj: Object) -> Self {
//         Self(obj)
//     }
// }

impl From<u64> for BigNum {
    fn from(x: u64) -> Self {
        // unsafe { Self::unchecked_new(host::bignum::from_u64(x)) }
        todo!()
    }
}

impl From<BigNum> for u64 {
    fn from(b: BigNum) -> Self {
        // unsafe { host::bignum::to_u64(b.into()) }
        todo!()
    }
}

impl From<i64> for BigNum {
    fn from(x: i64) -> Self {
        // unsafe { Self::unchecked_new(host::bignum::from_i64(x)) }
        todo!()
    }
}

impl From<BigNum> for i64 {
    fn from(b: BigNum) -> Self {
        // unsafe { host::bignum::to_i64(b.into()) }
        todo!()
    }
}

impl From<u32> for BigNum {
    fn from(x: u32) -> Self {
        // unsafe { Self::unchecked_new(host::bignum::from_u64(x.into())) }
        todo!()
    }
}

// TODO: impl From<BigNum> for u32

impl From<i32> for BigNum {
    fn from(x: i32) -> Self {
        // unsafe { Self::unchecked_new(host::bignum::from_i64(x.into())) }
        todo!()
    }
}

// TODO: impl From<BigNum> for i32

impl Add for BigNum {
    type Output = BigNum;
    fn add(self, rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::add(self.into(), rhs.into())) }
        todo!()
    }
}

impl Sub for BigNum {
    type Output = BigNum;
    fn sub(self, rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::sub(self.into(), rhs.into())) }
        todo!()
    }
}

impl Mul for BigNum {
    type Output = BigNum;
    fn mul(self, rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::mul(self.into(), rhs.into())) }
        todo!()
    }
}

impl Div for BigNum {
    type Output = BigNum;
    fn div(self, rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::div(self.into(), rhs.into())) }
        todo!()
    }
}

impl Rem for BigNum {
    type Output = BigNum;
    fn rem(self, rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::rem(self.into(), rhs.into())) }
        todo!()
    }
}

impl BitAnd for BigNum {
    type Output = BigNum;
    fn bitand(self, rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::and(self.into(), rhs.into())) }
        todo!()
    }
}

impl BitOr for BigNum {
    type Output = BigNum;
    fn bitor(self, rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::or(self.into(), rhs.into())) }
        todo!()
    }
}

impl BitXor for BigNum {
    type Output = BigNum;
    fn bitxor(self, rhs: Self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::xor(self.into(), rhs.into())) }
        todo!()
    }
}

impl Neg for BigNum {
    type Output = BigNum;
    fn neg(self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::neg(self.into())) }
        todo!()
    }
}

impl Not for BigNum {
    type Output = BigNum;
    fn not(self) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::not(self.into())) }
        todo!()
    }
}

impl Shl<u64> for BigNum {
    type Output = BigNum;
    fn shl(self, rhs: u64) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::shl(self.into(), rhs)) }
        todo!()
    }
}

impl Shr<u64> for BigNum {
    type Output = BigNum;
    fn shr(self, rhs: u64) -> Self::Output {
        // unsafe { Self::unchecked_new(host::bignum::shr(self.into(), rhs)) }
        todo!()
    }
}

impl PartialEq for BigNum {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Eq for BigNum {}
impl Ord for BigNum {
    fn cmp(&self, other: &Self) -> Ordering {
        // let i = unsafe {
        //     <i32 as ValType>::unchecked_from_val(host::bignum::cmp((*self).into(), (*other).into()))
        todo!()
        // };
        // if i < 0 {
        //     Ordering::Less
        // } else if i > 0 {
        //     Ordering::Greater
        // } else {
        //     Ordering::Equal
        // }
    }
}

impl BigNum {
    unsafe fn unchecked_new(obj: Object) -> Self {
        Self(obj)
    }

    pub fn gcd(&self, other: BigNum) -> BigNum {
        // unsafe { Self::unchecked_new(host::bignum::gcd((*self).into(), other.into())) }
        todo!()
    }

    pub fn lcm(&self, other: BigNum) -> BigNum {
        // unsafe { Self::unchecked_new(host::bignum::lcm((*self).into(), other.into())) }
        todo!()
    }

    pub fn pow(&self, k: u64) -> BigNum {
        // unsafe { Self::unchecked_new(host::bignum::pow((*self).into(), k)) }
        todo!()
    }

    pub fn pow_mod(&self, q: BigNum, m: BigNum) -> BigNum {
        // unsafe { Self::unchecked_new(host::bignum::pow_mod((*self).into(), q.into(), m.into())) }
        todo!()
    }

    pub fn sqrt(&self) -> BigNum {
        // unsafe { Self::unchecked_new(host::bignum::sqrt((*self).into())) }
        todo!()
    }

    pub fn is_zero(&self) -> bool {
        // unsafe { <bool as ValType>::unchecked_from_val(host::bignum::is_zero((*self).into())) }
        todo!()
    }

    pub fn bits(&self) -> u64 {
        // unsafe { host::bignum::bits((*self).into()) }
        todo!()
    }
}
