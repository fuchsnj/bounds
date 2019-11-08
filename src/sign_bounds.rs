use bound::{Bound, BoundType};
use num::Zero;
use std::cmp::Ordering;

pub struct SignBounds {
    pub above_zero: bool,
    pub zero: bool,
    pub below_zero: bool,
}

impl SignBounds {
    pub fn from_bounds<T: Ord + Zero>(a: &Option<Bound<T>>, b: &Option<Bound<T>>) -> SignBounds {
        let left = a.as_ref().map(Self::from_bound).unwrap_or(SignBounds::below_zero());
        let right = b.as_ref().map(Self::from_bound).unwrap_or(SignBounds::above_zero());
        left.merge(&right)
    }
    pub fn from_value<T: Zero + Ord>(value: &T) -> SignBounds {
        match value.cmp(&T::zero()) {
            Ordering::Equal => SignBounds::zero(),
            Ordering::Greater => SignBounds::above_zero(),
            Ordering::Less => SignBounds::below_zero()
        }
    }
    pub fn from_bound<T: Zero + Ord>(bound: &Bound<T>) -> SignBounds {
        match bound.value.cmp(&T::zero()) {
            Ordering::Equal => {
                if bound.bound_type == BoundType::Inclusive {
                    SignBounds::zero()
                } else {
                    SignBounds::none()
                }
            }
            Ordering::Greater => SignBounds::above_zero(),
            Ordering::Less => SignBounds::below_zero()
        }
    }
    pub fn zero() -> SignBounds {
        SignBounds {
            above_zero: false,
            zero: true,
            below_zero: false,
        }
    }
    pub fn above_zero() -> SignBounds {
        SignBounds {
            above_zero: true,
            zero: false,
            below_zero: false,
        }
    }
    pub fn below_zero() -> SignBounds {
        SignBounds {
            above_zero: false,
            zero: false,
            below_zero: true,
        }
    }
    pub fn none() -> SignBounds {
        SignBounds {
            above_zero: false,
            zero: false,
            below_zero: false,
        }
    }
    pub fn merge(&self, other: &SignBounds) -> SignBounds {
        let above_zero = self.above_zero || other.above_zero;
        let below_zero = self.below_zero || other.below_zero;
        let zero = self.zero || other.zero || (above_zero && below_zero);

        SignBounds {
            above_zero,
            below_zero,
            zero
        }
    }
}