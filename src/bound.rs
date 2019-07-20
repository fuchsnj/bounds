#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BoundType {
    Inclusive,
    Exclusive,
}

use std::ops::Neg;
use std::ops::Add;
use std::ops::Mul;
use std::cmp::Ordering;
use num::Zero;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Bound<T> {
    pub bound_type: BoundType,
    pub value: T,
}

impl<T: Ord> Bound<T> {
    pub fn upper_bound_max(a: Self, b: Self) -> Self {
        if a.is_upper_bound_max(&b) {
            a
        } else {
            b
        }
    }

    pub fn is_upper_bound_max(&self, other: &Self) -> bool {
        match self.value.cmp(&other.value) {
            Ordering::Greater => true,
            Ordering::Less => false,
            Ordering::Equal => self.bound_type == BoundType::Inclusive
        }
    }
    pub fn lower_bound_min(a: Self, b: Self) -> Self {
        if a.is_lower_bound_min(&b) {
            a
        } else {
            b
        }
    }

    pub fn is_lower_bound_min(&self, other: &Self) -> bool {
        match self.value.cmp(&other.value) {
            Ordering::Greater => false,
            Ordering::Less => true,
            Ordering::Equal => self.bound_type == BoundType::Inclusive
        }
    }
}

impl<T> Bound<T> {
    pub fn inclusive(value: T) -> Bound<T> {
        Bound {
            bound_type: BoundType::Inclusive,
            value,
        }
    }
    pub fn exclusive(value: T) -> Bound<T> {
        Bound {
            bound_type: BoundType::Exclusive,
            value,
        }
    }


    pub fn combine<F: FnOnce(T, T) -> T>(self, other: Self, func: F) -> Self {
        let bound_type = if self.bound_type == BoundType::Exclusive || other.bound_type == BoundType::Exclusive {
            BoundType::Exclusive
        } else {
            BoundType::Inclusive
        };

        Bound {
            bound_type,
            value: func(self.value, other.value),
        }
    }
}

impl<T: Neg<Output=T>> Neg for Bound<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Bound {
            bound_type: self.bound_type,
            value: -self.value,
        }
    }
}

impl<T: Add<T, Output=T>> Add for Bound<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.combine(other, Add::add)
    }
}

impl<T: Mul<T, Output=T>> Mul for Bound<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        self.combine(other, Mul::mul)
    }
}

//impl<T: Ord> Bound<T> {
//    pub fn is_max(&self, other: &Self) -> bool {
//        match self.value.cmp(&other.value) {
//            Ordering::Greater => true,
//            Ordering::Less => false,
//            Ordering::Equal => {
//                if self.bound_type == BoundType::Inclusive {
//                    true
//                } else {
//                    false
//                }
//            }
//        }
//    }
//
//    pub fn max(self, other: Self) -> Self {
//        if self.is_max(&other) {
//            self
//        } else {
//            other
//        }
//    }
//    pub fn is_min(&self, other: &Self) -> bool {
//        match self.value.cmp(&other.value) {
//            Ordering::Less => true,
//            Ordering::Greater => false,
//            Ordering::Equal => {
//                if self.bound_type == BoundType::Inclusive {
//                    true
//                } else {
//                    false
//                }
//            }
//        }
//    }
//
//    pub fn min(self, other: Self) -> Self {
//        if self.is_min(&other) {
//            self
//        } else {
//            other
//        }
//    }
//}