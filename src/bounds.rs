use bound::Bound;
use bound::BoundType::*;
use comparison::Comparison;
use num::Zero;
use sign_bounds::SignBounds;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::RangeFrom;
use std::ops::RangeFull;
use std::ops::RangeTo;
use std::ops::Sub;
use std::ops::{Div, Range};
use BoundType;

#[derive(Eq, PartialEq, Clone)]
pub enum Bounds<T> {
    Exact(T),
    Range(Option<Bound<T>>, Option<Bound<T>>),
}

impl<T: Debug> fmt::Debug for Bounds<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bounds::Exact(x) => write!(f, "{:?}", x),
            Bounds::Range(a, b) => {
                let left = match a {
                    Some(a) => match a.bound_type {
                        BoundType::Inclusive => format!("[{:?}", a.value),
                        BoundType::Exclusive => format!("({:?}", a.value),
                    },
                    None => "(-∞".to_owned(),
                };
                let right = match b {
                    Some(b) => match b.bound_type {
                        BoundType::Inclusive => format!("{:?}]", b.value),
                        BoundType::Exclusive => format!("{:?})", b.value),
                    },
                    None => "∞)".to_owned(),
                };
                write!(f, "{}, {}", left, right)
            }
        }
    }
}

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

impl<T: Neg<Output = T>> Neg for Bounds<T> {
    type Output = Bounds<T>;

    fn neg(self) -> Self::Output {
        match self {
            Bounds::Exact(x) => Bounds::Exact(-x),
            Bounds::Range(a, b) => Bounds::Range(b.map(Neg::neg), a.map(Neg::neg)),
        }
    }
}

fn combine_opts<T, F: FnOnce(T, T) -> T>(a: Option<T>, b: Option<T>, func: F) -> Option<T> {
    match (a, b) {
        (Some(a), Some(b)) => Some(func(a, b)),
        _ => None,
    }
}

impl<T: Add<T, Output = T> + Clone + Eq + Ord> Add for Bounds<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let opt_func = |a: Option<Bound<T>>, b: Option<Bound<T>>| -> Option<Bound<T>> {
            combine_opts(a, b, |a, b| a.combine(b, |x, y| x + y))
        };
        match (self, other) {
            (Bounds::Exact(a), Bounds::Exact(x)) => Bounds::Exact(a + x),
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

impl<T: Mul<T, Output = T> + Clone + Eq + Ord + Zero + Debug> Mul for Bounds<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let opt_func = |a: Option<Bound<T>>, b: Option<Bound<T>>| -> Option<Bound<T>> {
            combine_opts(a, b, |a, b| a.combine(b, |x, y| x * y))
        };
        match (self, other) {
            (Bounds::Exact(a), Bounds::Exact(x)) => Bounds::Exact(a * x),
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
                ]
                .into_iter()
                .for_each(|bound| {
                    if let Some(bound) = bound {
                        lower_bound = Some(match lower_bound.take() {
                            Some(x) => Bound::lower_bound_min(bound.clone(), x),
                            None => bound.clone(),
                        });
                        upper_bound = Some(match upper_bound.take() {
                            Some(x) => Bound::upper_bound_max(bound, x),
                            None => bound,
                        });
                    }
                });
                Bounds::Range(
                    if negative_infinity { None } else { lower_bound },
                    if positive_infinity { None } else { upper_bound },
                )
            }
            (Bounds::Exact(a), Bounds::Range(x, y)) | (Bounds::Range(x, y), Bounds::Exact(a)) => {
                if a.is_zero() {
                    return Bounds::Exact(a);
                }
                let positive = { a >= T::zero() };
                let bound_1 = opt_func(Some(Bound::inclusive(a.clone())), x);
                let bound_2 = opt_func(Some(Bound::inclusive(a)), y);
                if positive {
                    Bounds::Range(bound_1, bound_2)
                } else {
                    Bounds::Range(bound_2, bound_1)
                }
            }
        }
    }
}

impl<T: Div<T, Output = T> + Clone + Eq + Ord + Zero + Debug> Div for Bounds<T> {
    type Output = Option<Self>;

    fn div(self, other: Self) -> Self::Output {
        let opt_func = |a: Option<Bound<T>>, b: Option<Bound<T>>| -> Option<Bound<T>> {
            combine_opts(a, b, |a, b| a.combine(b, |x, y| x / y))
        };
        match (self, other) {
            (Bounds::Exact(a), Bounds::Exact(x)) => {
                if x.is_zero() {
                    None
                } else {
                    Some(Bounds::Exact(a / x))
                }
            }
            (Bounds::Range(a, b), Bounds::Range(x, y)) => {
                if let (Some(a), Some(b)) = (a, b) {
                    let a_bound_type = a.bound_type;
                    let b_bound_type = b.bound_type;
                    let mut left = (Bounds::Exact(a.value) / Bounds::Range(x.clone(), y.clone()))
                        .map(|c| {
                            if a_bound_type == BoundType::Exclusive {
                                c.to_exclusive()
                            } else {
                                c
                            }
                        });
                    let right =
                        (Bounds::Exact(b.value) / Bounds::Range(x.clone(), y.clone())).map(|c| {
                            if b_bound_type == BoundType::Exclusive {
                                c.to_exclusive()
                            } else {
                                c
                            }
                        });;
                    if let (Some(left), Some(right)) = (left, right) {
                        return Some(left.merge(right));
                    } else {
                        return None;
                    }
                }
                //TODO: implement
                unimplemented!()
            }
            (Bounds::Exact(a), Bounds::Range(x, y)) => {
                if SignBounds::from_bounds(&x, &y).zero {
                    return None;
                }
                if a.is_zero() {
                    return Some(Bounds::Exact(T::zero()));
                }
                if x == Some(Bound::exclusive(T::zero())) {
                    let positive = a >= T::zero();
                    let bound = Some(match y {
                        Some(y) => Bound::inclusive(a.clone()).combine(y, |c, d| c / d),
                        None => Bound::exclusive(T::zero()),
                    });
                    return Some(if positive {
                        Bounds::Range(bound, None)
                    } else {
                        Bounds::Range(None, bound)
                    });
                }
                if y == Some(Bound::exclusive(T::zero())) {
                    let positive = a >= T::zero();
                    let bound = Some(match x {
                        Some(x) => Bound::inclusive(a.clone()).combine(x, |c, d| c / d),
                        None => Bound::exclusive(T::zero()),
                    });
                    return Some(if positive {
                        Bounds::Range(None, bound)
                    } else {
                        Bounds::Range(bound, None)
                    });
                }
                let positive = a >= T::zero();
                let bound_1 = opt_func(Some(Bound::inclusive(a.clone())), x);
                let bound_2 = opt_func(Some(Bound::inclusive(a)), y);
                Some(if positive {
                    Bounds::Range(bound_2, bound_1)
                } else {
                    Bounds::Range(bound_1, bound_2)
                })
            }
            (Bounds::Range(x, y), Bounds::Exact(a)) => {
                if a.is_zero() {
                    return None;
                }
                let positive = a >= T::zero();
                let bound_1 = opt_func(x, Some(Bound::inclusive(a.clone())));
                let bound_2 = opt_func(y, Some(Bound::inclusive(a)));
                Some(if positive {
                    Bounds::Range(bound_1, bound_2)
                } else {
                    Bounds::Range(bound_2, bound_1)
                })
            }
        }
    }
}

impl<T: Neg<Output = T> + Add<T, Output = T> + Clone + Eq + Ord> Sub for Bounds<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

impl<T> Bounds<T> {
    fn to_exclusive(self) -> Bounds<T> {
        match self {
            Bounds::Exact(x) => Bounds::Exact(x),
            Bounds::Range(a, b) => {
                Bounds::Range(a.map(|x| x.to_exclusive()), b.map(|x| x.to_exclusive()))
            }
        }
    }
}

impl<T: Sub<Output = T> + Clone + Zero> Bounds<T> {
    pub fn size(&self) -> Option<T> {
        match *self {
            Bounds::Exact(_) => Some(T::zero()),
            Bounds::Range(None, _) | Bounds::Range(_, None) => None,
            Bounds::Range(Some(ref a), Some(ref b)) => Some(b.value.clone() - a.value.clone()),
        }
    }
}

impl<T: Eq + Ord> Bounds<T> {
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Bounds::Exact(a), Bounds::Exact(x)) => match a.cmp(&x) {
                Ordering::Equal => Bounds::Exact(a),
                Ordering::Less => {
                    Bounds::Range(Some(Bound::inclusive(a)), Some(Bound::inclusive(x)))
                }
                Ordering::Greater => {
                    Bounds::Range(Some(Bound::inclusive(x)), Some(Bound::inclusive(a)))
                }
            },
            (Bounds::Range(a, b), Bounds::Range(x, y)) => {
                debug_assert_bounds_order(&a, &b);
                debug_assert_bounds_order(&x, &y);
                let high = match (b, y) {
                    (None, None) => None,
                    (Some(val), None) => None,
                    (None, Some(val)) => Some(val),
                    (Some(b), Some(y)) => Some(Bound::upper_bound_max(b, y)),
                };
                let low = match (a, x) {
                    (None, None) => None,
                    (Some(val), None) => Some(val),
                    (None, Some(val)) => None,
                    (Some(a), Some(x)) => Some(Bound::lower_bound_min(a, x)),
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
            (&Bounds::Exact(ref a), &Bounds::Exact(ref x)) => match a.cmp(&x) {
                Ordering::Equal => Comparison::Intersects,
                Ordering::Less => Comparison::Less,
                Ordering::Greater => Comparison::Greater,
            },
            (&Bounds::Range(ref a, ref b), &Bounds::Range(ref x, ref y)) => {
                debug_assert_bounds_order(a, b);
                debug_assert_bounds_order(x, y);
                if let (&Some(ref a), &Some(ref y)) = (a, y) {
                    if a.bound_type == Inclusive && y.bound_type == Inclusive {
                        if a.value > y.value {
                            return Comparison::Greater;
                        }
                    } else {
                        if a.value >= y.value {
                            return Comparison::Greater;
                        }
                    }
                }
                if let (&Some(ref b), &Some(ref x)) = (b, x) {
                    if b.bound_type == Inclusive && x.bound_type == Inclusive {
                        if b.value < x.value {
                            return Comparison::Less;
                        }
                    } else {
                        if b.value <= x.value {
                            return Comparison::Less;
                        }
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
                            if a < &x.value {
                                return Comparison::Less;
                            }
                        }
                        Exclusive => {
                            if a <= &x.value {
                                return Comparison::Less;
                            }
                        }
                    }
                }

                if let &Some(ref y) = y {
                    match y.bound_type {
                        Inclusive => {
                            if a > &y.value {
                                return Comparison::Greater;
                            }
                        }
                        Exclusive => {
                            if a >= &y.value {
                                return Comparison::Greater;
                            }
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
    debug_assert!(
        match (x, y) {
            (&Some(ref x), &Some(ref y)) => x.value <= y.value,
            _ => true,
        },
        "invalid range: start must be less than end"
    );
}
