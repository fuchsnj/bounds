use std::ops::Range;
use bound::Bound;
use std::ops::RangeTo;
use std::ops::RangeFrom;
use std::ops::RangeFull;
use std::ops::Neg;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
use std::cmp::Ordering;
use bound::BoundType::*;
use comparison::Comparison;
use num::Zero;
use std::fmt::Debug;
use sign_bounds::SignBounds;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Bounds<T> {
    Exact(T),
    Range(Option<Bound<T>>, Option<Bound<T>>),
}

//impl<T> From<Option<Bound<T>>> for Bounds<T> {
//    fn from(value: Option<Bound<T>>) -> Self {
//        if let Some(value) = value {
//            if value.bound_type == Inclusive {
//                return Bounds::Exact(value.value);
//            }
//        }
//        Bounds::from(..)
//    }
//}

impl<T> From<Range<T>> for Bounds<T> {
    fn from(range: Range<T>) -> Self {
        let start = Bound::inclusive(range.start);
        let end = Bound::exclusive(range.end);
        Bounds::Range(Some(start), Some(end))
    }
}

impl<T> From<RangeTo<T>> for Bounds<T> {
    fn from(range: RangeTo<T>) -> Self {
        let end = Bound::exclusive(range.end);
        Bounds::Range(None, Some(end))
    }
}

impl<T> From<RangeFrom<T>> for Bounds<T> {
    fn from(range: RangeFrom<T>) -> Self {
        let start = Bound::inclusive(range.start);
        Bounds::Range(Some(start), None)
    }
}

impl<T> From<RangeFull> for Bounds<T> {
    fn from(_: RangeFull) -> Self {
        Bounds::Range(None, None)
    }
}

impl<T: Neg<Output=T>> Neg for Bounds<T> {
    type Output = Bounds<T>;

    fn neg(self) -> Self::Output {
        match self {
            Bounds::Exact(x) => Bounds::Exact(-x),
            Bounds::Range(a, b) => Bounds::Range(b.map(Neg::neg), a.map(Neg::neg))
        }
    }
}

//impl<T: Clone + Eq + Ord + Add<T, Output=T>> Bounds<T> {
//    pub fn combine_add(self, other: Self) -> Self {
//
//    }
//}

//impl<T: Clone + Eq + Ord> Bounds<T> {
//    pub fn combine<F, R>(self, other: Self, func: F, check_reverse: R) -> Self
//        where F: Fn(T, T) -> T + Copy,
//              R: Fn(&T) -> bool + Copy {
//        let bound_func = |a: Bound<T>, b: Bound<T>| -> Bound<T> {
//            a.combine(b, |a, b| func(a, b))
//        };
//        let opt_func = |a: Option<Bound<T>>, b: Option<Bound<T>>| -> Option<Bound<T>>{
//            combine_opts(a, b, |a, b| bound_func(a, b))
//        };
//
//        match (self, other) {
//            (Bounds::Exact(a), Bounds::Exact(x)) => {
//                Bounds::Exact(func(a, x))
//            }
//            (Bounds::Range(a, b), Bounds::Range(x, y)) => {
////                Bounds::from(a.clone()).combine(Bounds::Range(x.clone(), y.clone()), func, check_reverse)
////                    .merge(Bounds::from(b.clone()).combine(Bounds::Range(x.clone(), y.clone()), func, check_reverse))
//                Bounds::Range(opt_func(a.clone(), x.clone()), opt_func(a, y.clone()))
//                    .merge(Bounds::Range(opt_func(b.clone(), x), opt_func(b, y)))
//            }
//            (Bounds::Exact(a), Bounds::Range(x, y)) | (Bounds::Range(x, y), Bounds::Exact(a)) => {
//                let reverse = check_reverse(&a);
//                let first = opt_func(Some(Bound::inclusive(a.clone())), x);
//                let second = opt_func(Some(Bound::inclusive(a)), y);
//                if reverse {
//                    Bounds::Range(second, first)
//                } else {
//                    Bounds::Range(first, second)
//                }
//            }
//        }
//    }
//}

fn combine_opts<T, F: FnOnce(T, T) -> T>(a: Option<T>, b: Option<T>, func: F) -> Option<T> {
    match (a, b) {
        (Some(a), Some(b)) => Some(func(a, b)),
        _ => None
    }
}

//fn combine_bounds<T, F: FnOnce(T, T) -> T>(a: Option<Bound<T>>, b: Option<Bound<T>>, func: F) -> Option<Bound<T>> {
//    combine_opts(a, b, |a, b| a.combine(b, |x, y| func))
//}

impl<T: Add<T, Output=T> + Clone + Eq + Ord> Add for Bounds<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let opt_func = |a: Option<Bound<T>>, b: Option<Bound<T>>| -> Option<Bound<T>>{
            combine_opts(a, b, |a, b| a.combine(b, |x, y| x + y))
        };
        match (self, other) {
            (Bounds::Exact(a), Bounds::Exact(x)) => {
                Bounds::Exact(a + x)
            }
            (Bounds::Range(a, b), Bounds::Range(x, y)) => {
                Bounds::Range(opt_func(a, x), opt_func(b, y))
            }
            (Bounds::Exact(a), Bounds::Range(x, y)) | (Bounds::Range(x, y), Bounds::Exact(a)) => {
                Bounds::Range(
                    opt_func(Some(Bound::inclusive(a.clone())), x),
                    opt_func(Some(Bound::inclusive(a)), y),
                )
            }
        }
    }
}

impl<T: Mul<T, Output=T> + Clone + Eq + Ord + Zero + Debug> Mul for Bounds<T> {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        let opt_func = |a: Option<Bound<T>>, b: Option<Bound<T>>| -> Option<Bound<T>>{
            combine_opts(a, b, |a, b| a.combine(b, |x, y| x * y))
        };
        match (self, other) {
            (Bounds::Exact(a), Bounds::Exact(x)) => {
                Some(Bounds::Exact(a * x))
            }
            (Bounds::Range(a, b), Bounds::Range(x, y)) => {
                let left_signs = SignBounds::from_bounds(&a, &b);
                let right_signs = SignBounds::from_bounds(&x, &y);
                let mut negative_infinity = false;
                let mut positive_infinity = false;
                if a.is_none() {
                    positive_infinity |= right_signs.below_zero;
                    negative_infinity |= right_signs.above_zero
                }
                if b.is_none() {
                    positive_infinity |= right_signs.above_zero;
                    negative_infinity |= right_signs.below_zero
                }
                if x.is_none() {
                    positive_infinity |= left_signs.below_zero;
                    negative_infinity |= left_signs.above_zero
                }
                if y.is_none() {
                    positive_infinity |= left_signs.above_zero;
                    negative_infinity |= left_signs.below_zero
                }
                let mut lower_bound: Option<Bound<T>> = None;
                let mut upper_bound: Option<Bound<T>> = None;
                vec![
                    combine_opts(a.clone(), x.clone(), |a, b| a * b),
                    combine_opts(a.clone(), y.clone(), |a, b| a * b),
                    combine_opts(b.clone(), x.clone(), |a, b| a * b),
                    combine_opts(b.clone(), y.clone(), |a, b| a * b),
                ].into_iter().for_each(|bound| {
                    if let Some(bound) = bound {
                        lower_bound = Some(match lower_bound.take() {
                            Some(x) => Bound::lower_bound_min(bound.clone(), x),
                            None => bound.clone()
                        });
                        upper_bound = Some(match upper_bound.take() {
                            Some(x) => Bound::upper_bound_max(bound, x),
                            None => bound
                        });
                    }
                });
                Some(Bounds::Range(
                    if negative_infinity { None } else { lower_bound },
                    if positive_infinity { None } else { upper_bound },
                ))
            }
            (Bounds::Exact(a), Bounds::Range(x, y)) | (Bounds::Range(x, y), Bounds::Exact(a)) => {
                if a == T::zero() {
                    return if x.is_none() || y.is_none() {
                        None
                    } else {
                        Some(Bounds::Exact(a))
                    };
                }
                let not_negative = { a >= T::zero() };
                let bound_1 = opt_func(Some(Bound::inclusive(a.clone())), x);
                let bound_2 = opt_func(Some(Bound::inclusive(a)), y);
                let output = if not_negative {
                    Bounds::Range(bound_1, bound_2)
                } else {
                    Bounds::Range(bound_2, bound_1)
                };
                Some(output)
            }
        }
    }
}

impl<T: Neg<Output=T> + Add<T, Output=T> + Clone + Eq + Ord> Sub for Bounds<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl<T: Sub<Output=T> + Clone + Zero> Bounds<T> {
    pub fn size(&self) -> Option<T> {
        match *self {
            Bounds::Exact(_) => {
                Some(T::zero())
            }
            Bounds::Range(None, _) | Bounds::Range(_, None) => None,
            Bounds::Range(Some(ref a), Some(ref b)) => {
                Some(b.value.clone() - a.value.clone())
            }
        }
    }
}


impl<T: Eq + Ord> Bounds<T> {
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Bounds::Exact(a), Bounds::Exact(x)) => {
                match a.cmp(&x) {
                    Ordering::Equal => Bounds::Exact(a),
                    Ordering::Less => Bounds::Range(Some(Bound::inclusive(a)), Some(Bound::inclusive(x))),
                    Ordering::Greater => Bounds::Range(Some(Bound::inclusive(x)), Some(Bound::inclusive(a)))
                }
            }
            (Bounds::Range(a, b), Bounds::Range(x, y)) => {
                debug_assert_bounds_order(&a, &b);
                debug_assert_bounds_order(&x, &y);
                let high = match (b, y) {
                    (None, None) => None,
                    (Some(val), None) | (None, Some(val)) => Some(val),
                    (Some(b), Some(y)) => {
                        Some(Bound::upper_bound_max(b, y))
                    }
                };
                let low = match (a, x) {
                    (None, None) => None,
                    (Some(val), None) | (None, Some(val)) => Some(val),
                    (Some(a), Some(x)) => {
                        Some(Bound::lower_bound_min(a, x))
                    }
                };
                Bounds::Range(low, high)
            }
            (a, b @ Bounds::Exact(_)) => b.merge(a),
            (Bounds::Exact(a), Bounds::Range(x, y)) => {
                let a_bound = Bound::inclusive(a);
                match (x, y) {
                    (None, None) => Bounds::Exact(a_bound.value),
                    (Some(x), Some(y)) => {
                        if a_bound.is_lower_bound_min(&x) {
                            Bounds::Range(Some(a_bound), Some(y))
                        } else if a_bound.is_upper_bound_max(&y) {
                            Bounds::Range(Some(x), Some(a_bound))
                        } else {
                            Bounds::Range(Some(x), Some(y))
                        }
                    }
                    (Some(x), None) => {
                        if a_bound.is_lower_bound_min(&x) {
                            Bounds::Range(Some(a_bound), None)
                        } else {
                            Bounds::Range(Some(x), None)
                        }
                    }
                    (None, Some(y)) => {
                        if a_bound.is_upper_bound_max(&y) {
                            Bounds::Range(None, Some(a_bound))
                        } else {
                            Bounds::Range(None, Some(y))
                        }
                    }
                }
            }
        }
    }

    pub fn range(start: Bound<T>, end: Bound<T>) -> Self {
        Bounds::Range(Some(start), Some(end))
    }
    pub fn intersects(&self, other: &Bounds<T>) -> bool {
        self.compare_to(other) == Comparison::Intersects
    }

    pub fn compare_to(&self, other: &Bounds<T>) -> Comparison {
        match (self, other) {
            (&Bounds::Exact(ref a), &Bounds::Exact(ref x)) => {
                match a.cmp(&x) {
                    Ordering::Equal => Comparison::Intersects,
                    Ordering::Less => Comparison::Less,
                    Ordering::Greater => Comparison::Greater
                }
            }
            (&Bounds::Range(ref a, ref b), &Bounds::Range(ref x, ref y)) => {
                debug_assert_bounds_order(a, b);
                debug_assert_bounds_order(x, y);
                if let (&Some(ref a), &Some(ref y)) = (a, y) {
                    if a.bound_type == Inclusive && y.bound_type == Inclusive {
                        if a.value > y.value { return Comparison::Greater; }
                    } else {
                        if a.value >= y.value { return Comparison::Greater; }
                    }
                }
                if let (&Some(ref b), &Some(ref x)) = (b, x) {
                    if b.bound_type == Inclusive && x.bound_type == Inclusive {
                        if b.value < x.value { return Comparison::Less; }
                    } else {
                        if b.value <= x.value { return Comparison::Less; }
                    }
                }
                Comparison::Intersects
            }
            (a, b @ &Bounds::Exact(_)) => -b.compare_to(a),
            (&Bounds::Exact(ref a), &Bounds::Range(ref x, ref y)) => {
                debug_assert_bounds_order(x, y);
                if let &Some(ref x) = x {
                    match x.bound_type {
                        Inclusive => {
                            if a < &x.value { return Comparison::Less; }
                        }
                        Exclusive => {
                            if a <= &x.value { return Comparison::Less; }
                        }
                    }
                }

                if let &Some(ref y) = y {
                    match y.bound_type {
                        Inclusive => {
                            if a > &y.value { return Comparison::Greater; }
                        }
                        Exclusive => {
                            if a >= &y.value { return Comparison::Greater; }
                        }
                    }
                }

                return Comparison::Intersects;
            }
        }
    }
}

#[inline(always)]
fn debug_assert_bounds_order<T: PartialOrd>(x: &Option<Bound<T>>, y: &Option<Bound<T>>) {
    debug_assert!(match (x, y) {
        (&Some(ref x), &Some(ref y)) => x.value <= y.value,
        _ => true
    }, "invalid range: start must be less than end");
}