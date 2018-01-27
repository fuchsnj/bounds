use std::ops::{Range, RangeTo, RangeFrom, RangeFull, Sub, Neg};
use std::cmp::{PartialOrd, Ordering, Ord};

#[derive(Debug, PartialEq, Eq)]
pub enum BoundType {
	Inclusive,
	Exclusive,
}

use self::BoundType::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Bound<T> {
	pub bound_type: BoundType,
	pub value: T,
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

impl<T: Ord> Bound<T> {
	pub fn is_max(&self, other: &Self) -> bool {
		match self.value.cmp(&other.value) {
			Ordering::Greater => true,
			Ordering::Less => false,
			Ordering::Equal => {
				if self.bound_type == BoundType::Inclusive {
					true
				} else {
					false
				}
			}
		}
	}

	pub fn max(self, other: Self) -> Self {
		if self.is_max(&other) {
			self
		} else {
			other
		}
	}
	pub fn is_min(&self, other: &Self) -> bool {
		match self.value.cmp(&other.value) {
			Ordering::Less => true,
			Ordering::Greater => false,
			Ordering::Equal => {
				if self.bound_type == BoundType::Inclusive {
					true
				} else {
					false
				}
			}
		}
	}

	pub fn min(self, other: Self) -> Self {
		if self.is_min(&other) {
			self
		} else {
			other
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum Comparison {
	Less,
	Greater,
	Intersects,
}

impl Neg for Comparison {
	type Output = Comparison;

	fn neg(self) -> Self::Output {
		match self {
			Comparison::Less => Comparison::Greater,
			Comparison::Intersects => Comparison::Intersects,
			Comparison::Greater => Comparison::Less
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum Bounds<T> {
	Exact(T),
	Range(Option<Bound<T>>, Option<Bound<T>>),
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

impl<T: Neg<Output=T>> Neg for Bounds<T> {
	type Output = Bounds<T>;

	fn neg(self) -> Self::Output {
		match self {
			Bounds::Exact(x) => Bounds::Exact(-x),
			Bounds::Range(a, b) => Bounds::Range(b.map(Neg::neg), a.map(Neg::neg))
		}
	}
}

impl<T: Sub<Output=T> + Clone> Bounds<T> {
	pub fn size(&self) -> Option<T> {
		match *self {
			Bounds::Exact(ref x) => {
				//TODO: use Zero trait once it is stable
				Some(x.clone() - x.clone())
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
						Some(b.max(y))
					}
				};
				let low = match (a, x) {
					(None, None) => None,
					(Some(val), None) | (None, Some(val)) => Some(val),
					(Some(a), Some(x)) => {
						Some(a.min(x))
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
						if a_bound.is_min(&x) {
							Bounds::Range(Some(a_bound), Some(y))
						} else if a_bound.is_max(&y) {
							Bounds::Range(Some(x), Some(a_bound))
						} else {
							Bounds::Range(Some(x), Some(y))
						}
					}
					(Some(x), None) => {
						if a_bound.is_min(&x) {
							Bounds::Range(Some(a_bound), None)
						} else {
							Bounds::Range(Some(x), None)
						}
					}
					(None, Some(y)) => {
						if a_bound.is_max(&y) {
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
						BoundType::Inclusive => {
							if a < &x.value { return Comparison::Less; }
						}
						BoundType::Exclusive => {
							if a <= &x.value { return Comparison::Less; }
						}
					}
				}

				if let &Some(ref y) = y {
					match y.bound_type {
						BoundType::Inclusive => {
							if a > &y.value { return Comparison::Greater; }
						}
						BoundType::Exclusive => {
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


#[test]
fn test_intersection() {
	assert!(Bounds::Exact(42).intersects(&Bounds::Exact(42)));
	assert!(!Bounds::Exact(42).intersects(&Bounds::Exact(0)));
	assert!(Bounds::Exact(1).intersects(&Bounds::range(Bound::inclusive(1), Bound::inclusive(3))));
	assert!(Bounds::Exact(2).intersects(&Bounds::range(Bound::inclusive(1), Bound::inclusive(3))));
	assert!(Bounds::Exact(3).intersects(&Bounds::range(Bound::inclusive(1), Bound::inclusive(3))));

	assert!(!Bounds::Exact(1).intersects(&Bounds::range(Bound::exclusive(1), Bound::exclusive(3))));
	assert!(Bounds::Exact(2).intersects(&Bounds::range(Bound::exclusive(1), Bound::exclusive(3))));
	assert!(!Bounds::Exact(3).intersects(&Bounds::range(Bound::exclusive(1), Bound::exclusive(3))));

	assert!(!Bounds::range(Bound::exclusive(1), Bound::exclusive(3)).intersects(
		&Bounds::range(Bound::exclusive(3), Bound::exclusive(5))
	));

	assert!(!Bounds::range(Bound::exclusive(1), Bound::inclusive(3)).intersects(
		&Bounds::range(Bound::exclusive(3), Bound::exclusive(5))
	));

	assert!(Bounds::range(Bound::exclusive(1), Bound::inclusive(3)).intersects(
		&Bounds::range(Bound::inclusive(3), Bound::exclusive(5))
	));

	assert!(Bounds::from(1..3).intersects(&Bounds::from(1..3)));
	assert!(Bounds::from(2..4).intersects(&Bounds::from(1..3)));
	assert!(!Bounds::from(3..5).intersects(&Bounds::from(1..3)));

	assert!(!Bounds::from(1..3).intersects(&Bounds::from(..0)));
	assert!(!Bounds::from(1..3).intersects(&Bounds::from(..1)));
	assert!(Bounds::from(1..3).intersects(&Bounds::from(..2)));

	assert!(Bounds::from(1..3).intersects(&Bounds::from(2..)));
	assert!(!Bounds::from(1..3).intersects(&Bounds::from(3..)));
	assert!(!Bounds::from(1..3).intersects(&Bounds::from(4..)));

	assert!(!Bounds::from(2..).intersects(&Bounds::from(..1)));
	assert!(!Bounds::from(2..).intersects(&Bounds::from(..2)));
	assert!(Bounds::from(2..).intersects(&Bounds::from(..3)));

	assert!(Bounds::from(..2).intersects(&Bounds::from(1..)));
	assert!(!Bounds::from(..2).intersects(&Bounds::from(2..)));
	assert!(!Bounds::from(..2).intersects(&Bounds::from(3..)));

	assert!(Bounds::from(..2).intersects(&Bounds::from(..1)));
	assert!(Bounds::from(..2).intersects(&Bounds::from(..2)));
	assert!(Bounds::from(..2).intersects(&Bounds::from(..3)));

	assert!(Bounds::from(2..).intersects(&Bounds::from(1..)));
	assert!(Bounds::from(2..).intersects(&Bounds::from(2..)));
	assert!(Bounds::from(2..).intersects(&Bounds::from(3..)));

	assert!(Bounds::from(1..3).intersects(&Bounds::from(..)));
	assert!(Bounds::<i32>::from(..).intersects(&Bounds::from(..)));
}

#[test]
fn test_size() {
	assert_eq!(Bounds::from(0..2).size(), Some(2));
	assert_eq!(Bounds::from(-1..1).size(), Some(2));
	assert_eq!(Bounds::from(1..3).size(), Some(2));
	assert_eq!(Bounds::from(0..0).size(), Some(0));
	assert_eq!(Bounds::from(1..1).size(), Some(0));
	assert_eq!(Bounds::from(1..).size(), None);
	assert_eq!(Bounds::from(..1).size(), None);
	assert_eq!(Bounds::<u32>::from(..).size(), None);
}

#[test]
fn test_compare() {
	assert_eq!(Bounds::from(1..3).compare_to(&Bounds::from(2..4)), Comparison::Intersects);
	assert_eq!(Bounds::from(2..3).compare_to(&Bounds::from(0..2)), Comparison::Greater);
	assert_eq!(Bounds::from(0..2).compare_to(&Bounds::from(2..3)), Comparison::Less);
	assert_eq!(Bounds::Exact(2).compare_to(&Bounds::from(3..4)), Comparison::Less);
	assert_eq!(Bounds::from(3..4).compare_to(&Bounds::Exact(2)), Comparison::Greater);
}

#[test]
fn test_merge() {
	assert_eq!(Bounds::from(1..3).merge(Bounds::Exact(2)), Bounds::from(1..3));
	assert_eq!(Bounds::from(1..3).merge(Bounds::Exact(1)), Bounds::from(1..3));
	assert_eq!(Bounds::from(1..3).merge(Bounds::Exact(3)),
	           Bounds::Range(Some(Bound::inclusive(1)), Some(Bound::inclusive(3))));
	assert_eq!(Bounds::from(1..3).merge(Bounds::Exact(0)),
	           Bounds::Range(Some(Bound::inclusive(0)), Some(Bound::exclusive(3))));
	assert_eq!(Bounds::from(1..3).merge(Bounds::Exact(4)),
	           Bounds::Range(Some(Bound::inclusive(1)), Some(Bound::inclusive(4))));

	assert_eq!(Bounds::from(1..4).merge(Bounds::from(2..3)), Bounds::from(1..4));
	assert_eq!(Bounds::from(1..4).merge(Bounds::from(0..2)), Bounds::from(0..4));
	assert_eq!(Bounds::from(1..4).merge(Bounds::from(3..5)), Bounds::from(1..5));
	assert_eq!(Bounds::from(1..4).merge(Bounds::from(0..5)), Bounds::from(0..5));
}

#[test]
fn test_neg() {
	assert_eq!(-Bounds::from(1..3), Bounds::Range(Some(Bound::exclusive(-3)), Some(Bound::inclusive(-1))));
	assert_eq!(-Bounds::Exact(2), Bounds::Exact(-2));
	assert_eq!(-Bounds::from(1..), Bounds::Range(None, Some(Bound::inclusive(-1))));
	assert_eq!(-Bounds::from(..2), Bounds::Range(Some(Bound::exclusive(-2)), None));
	assert_eq!(-Bounds::<i32>::from(..), Bounds::<i32>::from(..));
}