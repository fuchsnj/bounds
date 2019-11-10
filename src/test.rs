use crate::bounds;
use bound::Bound;
use bounds::Bounds;
use comparison::Comparison;

#[test]
fn test_intersection() {
    assert!(bounds!(42).intersects(&bounds!(42)));
    assert!(!bounds!(42).intersects(&bounds!(0)));

    assert!(bounds!(1).intersects(&bounds!(1, 3)));
    assert!(bounds!(2).intersects(&bounds!(1, 3)));
    assert!(bounds!(3).intersects(&bounds!(1, 3)));

    assert!(!bounds!(1).intersects(&bounds!(~1, ~3)));
    assert!(bounds!(2).intersects(&bounds!(~1, ~3)));
    assert!(!bounds!(3).intersects(&bounds!(~1, ~3)));

    assert!(!bounds!(~1, ~3).intersects(&bounds!(~3, ~5)));

    assert!(!bounds!(~1, 3).intersects(&bounds!(~3, ~5)));

    assert!(bounds!(~1, 3).intersects(&bounds!(3, ~5)));

    assert!(bounds!(1,~3).intersects(&bounds!(1,~3)));
    assert!(Bounds::from(2..4).intersects(&bounds!(1,~3)));
    assert!(!bounds!(3,~5).intersects(&bounds!(1,~3)));

    assert!(!bounds!(1,~3).intersects(&Bounds::from(..0)));
    assert!(!bounds!(1,~3).intersects(&Bounds::from(..1)));
    assert!(bounds!(1,~3).intersects(&bounds!(,~2)));

    assert!(bounds!(1,~3).intersects(&Bounds::from(2..)));
    assert!(!bounds!(1,~3).intersects(&Bounds::from(3..)));
    assert!(!bounds!(1,~3).intersects(&Bounds::from(4..)));

    assert!(!Bounds::from(2..).intersects(&Bounds::from(..1)));
    assert!(!Bounds::from(2..).intersects(&bounds!(,~2)));
    assert!(Bounds::from(2..).intersects(&Bounds::from(..3)));

    assert!(bounds!(,~2).intersects(&bounds!(1,)));
    assert!(!bounds!(,~2).intersects(&Bounds::from(2..)));
    assert!(!bounds!(,~2).intersects(&Bounds::from(3..)));

    assert!(bounds!(,~2).intersects(&Bounds::from(..1)));
    assert!(bounds!(,~2).intersects(&bounds!(,~2)));
    assert!(bounds!(,~2).intersects(&Bounds::from(..3)));

    assert!(Bounds::from(2..).intersects(&bounds!(1,)));
    assert!(Bounds::from(2..).intersects(&Bounds::from(2..)));
    assert!(Bounds::from(2..).intersects(&Bounds::from(3..)));

    assert!(bounds!(1,~3).intersects(&Bounds::from(..)));
    assert!(Bounds::<i32>::from(..).intersects(&Bounds::from(..)));
}

#[test]
fn test_size() {
    assert_eq!(bounds!(0,~2).size(), Some(2));
    assert_eq!(Bounds::from(-1..1).size(), Some(2));
    assert_eq!(bounds!(1,~3).size(), Some(2));
    assert_eq!(Bounds::from(0..0).size(), Some(0));
    assert_eq!(Bounds::from(1..1).size(), Some(0));
    assert_eq!(bounds!(1,).size(), None);
    assert_eq!(Bounds::from(..1).size(), None);
    assert_eq!(Bounds::<u32>::from(..).size(), None);
}

#[test]
fn test_compare() {
    assert_eq!(
        bounds!(1,~3).compare_to(&bounds!(2,~4)),
        Comparison::Intersects
    );
    assert_eq!(
        bounds!(2,~3).compare_to(&bounds!(0,~2)),
        Comparison::Greater
    );
    assert_eq!(bounds!(0,~2).compare_to(&bounds!(2,~3)), Comparison::Less);
    assert_eq!(bounds!(2).compare_to(&bounds!(3,~4)), Comparison::Less);
    assert_eq!(bounds!(3,~4).compare_to(&bounds!(2)), Comparison::Greater);
}

#[test]
fn test_merge() {
    assert_eq!(bounds!(1,~3).merge(bounds!(2)), bounds!(1,~3));
    assert_eq!(bounds!(1,~3).merge(bounds!(1)), bounds!(1,~3));
    assert_eq!(bounds!(1,~3).merge(bounds!(3)), bounds!(1, 3));
    assert_eq!(bounds!(1,~3).merge(bounds!(0)), bounds!(0, ~3));
    assert_eq!(bounds!(1,~3).merge(bounds!(4)), bounds!(1, 4));

    assert_eq!(bounds!(1,~4).merge(bounds!(2,~3)), bounds!(1,~4));
    assert_eq!(bounds!(1,~4).merge(bounds!(0,~2)), bounds!(0,~4));
    assert_eq!(bounds!(1,~4).merge(bounds!(3,~5)), bounds!(1,~5));
    assert_eq!(bounds!(1,~4).merge(bounds!(0,~5)), bounds!(0,~5));
    assert_eq!(bounds!(,-1).merge(bounds!(3,)), bounds!(,));
}

#[test]
fn test_neg() {
    assert_eq!(-bounds!(1,~3), bounds!(~-3,-1));
    assert_eq!(-bounds!(2), bounds!(-2));
    assert_eq!(-bounds!(1,), bounds!(,-1));
    assert_eq!(-bounds!(,~2), bounds!(~-2,));
    assert_eq!(-Bounds::<i32>::from(..), Bounds::<i32>::from(..));
}

#[test]
fn test_add() {
    assert_eq!(bounds!(1,~3) + bounds!(2,~3), Bounds::from(3..6));
    assert_eq!(bounds!(1) + bounds!(1,~3), Bounds::from(2..4));
    assert_eq!(bounds!(1,~3) + bounds!(1), Bounds::from(2..4));
    assert_eq!(bounds!(1,~3) + Bounds::from(..), Bounds::from(..));
    assert_eq!(bounds!(1,~3) + bounds!(1,), Bounds::from(2..));
    assert_eq!(bounds!(1,~3) + Bounds::from(..3), Bounds::from(..6));
    assert_eq!(Bounds::from(..) + bounds!(1,~3), Bounds::from(..));
}

#[test]
fn test_sub() {
    assert_eq!(bounds!(1,~3) - bounds!(2,~3), bounds!(~-2, ~1));
    assert_eq!(bounds!(1) - bounds!(1,~3), bounds!(~-2, 0));
    assert_eq!(bounds!(1,~3) - bounds!(1), bounds!(0,~2));
    assert_eq!(bounds!(1,~3) - Bounds::from(..), Bounds::from(..));
    assert_eq!(bounds!(1,~3) - bounds!(1,), bounds!(,~2));
    assert_eq!(bounds!(1,~3) - Bounds::from(..3), bounds!(~-2,));
}

#[test]
fn test_mul() {
    assert_eq!(bounds!(1) * bounds!(1,~3), bounds!(1,~3));
    assert_eq!(bounds!(1) * Bounds::from(2..), Bounds::from(2..));
    assert_eq!(bounds!(1) * bounds!(,~2), bounds!(,~2));
    assert_eq!(Bounds::Exact(-1) * bounds!(1,~3), bounds!(~-3,-1));
    assert_eq!(Bounds::Exact(-1) * Bounds::from(2..), bounds!(,-2));
    assert_eq!(Bounds::Exact(-1) * bounds!(,~2), bounds!(~-2,));
    assert_eq!(bounds!(0) * bounds!(2,~3), bounds!(0));
    assert_eq!(bounds!(0) * Bounds::from(..), bounds!(0));
    assert_eq!(bounds!(0) * Bounds::from(2..), bounds!(0));
    assert_eq!(bounds!(0) * Bounds::from(..3), bounds!(0));

    assert_eq!(Bounds::<i32>::Range(None, None) * bounds!(,), bounds!(,));
    assert_eq!(bounds!(,) * Bounds::from(1..2), bounds!(,));
    assert_eq!(Bounds::from(1..2) * Bounds::from(..), Bounds::from(..));
    assert_eq!(Bounds::Exact(-1) * bounds!(1,), bounds!(,-1));
    assert_eq!(bounds!(1,~3) * bounds!(2,~3), Bounds::from(2..9));
    assert_eq!(bounds!(1,~3) * bounds!(1), bounds!(1,~3));
    assert_eq!(bounds!(1,~3) * Bounds::from(..), Bounds::from(..));
    assert_eq!(Bounds::from(..) * bounds!(1,~3), Bounds::from(..));
    assert_eq!(bounds!(1,~3) * bounds!(1,), bounds!(1,));
    assert_eq!(bounds!(1,) * bounds!(1,~3), bounds!(1,));
    assert_eq!(bounds!(1,~3) * Bounds::from(..3), Bounds::from(..9));
    assert_eq!(bounds!(1,~3) * bounds!(0), bounds!(0));
    assert_eq!(bounds!(1,) * bounds!(1), bounds!(1,));
    assert_eq!(Bounds::from(-1..1) * bounds!(1,), Bounds::from(..));
    assert_eq!(Bounds::from(..1) * bounds!(1,), Bounds::from(..));
    assert_eq!(Bounds::from(-1..) * bounds!(1,), Bounds::from(..));
    assert_eq!(Bounds::from(-1..1) * Bounds::from(..), Bounds::from(..));
}

#[test]
fn test_div() {
    assert_eq!(bounds!(4) / bounds!(2), Some(bounds!(2)));
    assert_eq!(bounds!(-4) / bounds!(2), Some(bounds!(-2)));
    assert_eq!(bounds!(4) / bounds!(-2), Some(bounds!(-2)));
    assert_eq!(bounds!(1) / bounds!(0), None);

    assert_eq!(bounds!(2,~6) / bounds!(2), Some(bounds!(1,~3)));
    assert_eq!(bounds!(-2,~6) / bounds!(2), Some(Bounds::from(-1..3)));
    assert_eq!(bounds!(-6,~-2) / bounds!(2), Some(Bounds::from(-3..-1)));
    assert_eq!(bounds!(2,~6) / bounds!(-2), Some(bounds!(~-3,-1)));
    assert_eq!(bounds!(-2,~6) / bounds!(-2), Some(bounds!(~-3,1)));
    assert_eq!(bounds!(-6,~-2) / bounds!(-2), Some(bounds!(~1, 3)));
    assert_eq!(bounds!(2,~6) / bounds!(0), None);
    assert_eq!(Bounds::from(2..) / bounds!(2), Some(bounds!(1,)));
    assert_eq!(Bounds::from(2..) / bounds!(-2), Some(bounds!(,-1)));
    assert_eq!(Bounds::from(-2..) / bounds!(2), Some(Bounds::from(-1..)));
    assert_eq!(Bounds::from(-2..) / bounds!(-2), Some(bounds!(,1)));
    assert_eq!(bounds!(,~2) / bounds!(2), Some(Bounds::from(..1)));
    assert_eq!(bounds!(,~2) / bounds!(-2), Some(bounds!(~-1,)));
    assert_eq!(bounds!(,~-2) / bounds!(2), Some(Bounds::from(..-1)));
    assert_eq!(bounds!(,~-2) / bounds!(-2), Some(bounds!(~1,)));

    assert_eq!(bounds!(2) / Bounds::from(..), None);
    assert_eq!(bounds!(0) / Bounds::from(1..2), Some(bounds!(0)));
    assert_eq!(bounds!(6) / bounds!(2,~3), Some(bounds!(~2,3)));
    assert_eq!(bounds!(-6) / bounds!(2,~3), Some(Bounds::from(-3..-2)));
    assert_eq!(bounds!(6) / Bounds::from(-3..-2), Some(bounds!(~-3,-2)));
    assert_eq!(bounds!(-6) / Bounds::from(-3..-2), Some(bounds!(2,~3)));
    assert_eq!(bounds!(6) / Bounds::from(2..), Some(bounds!(,3)));
    assert_eq!(bounds!(6) / Bounds::from(-2..), None);
    assert_eq!(bounds!(6) / bounds!(,~2), None);
    assert_eq!(bounds!(6) / bounds!(,~-2), Some(bounds!(~-3,)));

    assert_eq!(bounds!(-6) / Bounds::from(2..), Some(Bounds::from(-3..)));
    assert_eq!(bounds!(-6) / Bounds::from(-2..), None);
    assert_eq!(bounds!(-6) / bounds!(,~2), None);
    assert_eq!(bounds!(-6) / bounds!(,~-2), Some(Bounds::from(..3)));

    assert_eq!(bounds!(2) / bounds!(~0,1), Some(Bounds::from(2..)));
    assert_eq!(bounds!(-2) / bounds!(~0,1), Some(bounds!(,-2)));
    assert_eq!(bounds!(2) / bounds!(~0,), Some(bounds!(~0,)));
    assert_eq!(bounds!(-2) / bounds!(~0,), Some(Bounds::from(..0)));
    assert_eq!(bounds!(0) / bounds!(~0,), Some(bounds!(0)));
    assert_eq!(bounds!(2) / Bounds::from(-1..0), Some(bounds!(,-2)));
    assert_eq!(bounds!(-2) / Bounds::from(-1..0), Some(Bounds::from(2..)));
    assert_eq!(bounds!(2) / Bounds::from(..0), Some(Bounds::from(..0)));
    assert_eq!(bounds!(-2) / bounds!(,~0), Some(bounds!(~0,)));

    assert_eq!(bounds!(1, 2) / bounds!(-1, 1), None);
    assert_eq!(bounds!(2, 6) / bounds!(-2,~0), Some(bounds!(, -1)));
    assert_eq!(bounds!(2, 6) / bounds!(-2, 0), None);
    assert_eq!(bounds!(2, 6) / bounds!(~0, 2), Some(bounds!(1,)));
    assert_eq!(bounds!(2, 6) / bounds!(~0, ~2), Some(bounds!(~1,)));
    assert_eq!(bounds!(-2, 6) / bounds!(~0, 2), Some(bounds!(,)));
    assert_eq!(bounds!(-6, -2) / bounds!(~0, 2), Some(bounds!(,-1)));
    assert_eq!(bounds!(~2, 6) / bounds!(~0, 2), Some(bounds!(~1,)));
    assert_eq!(bounds!(2, ~6) / bounds!(~0, 2), Some(bounds!(1,)));
    assert_eq!(bounds!(~2, ~6) / bounds!(~0, 2), Some(bounds!(~1,)));
}
