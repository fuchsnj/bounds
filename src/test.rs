use bounds::Bounds;
use bound::Bound;
use comparison::Comparison;

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

#[test]
fn test_add() {
    assert_eq!(Bounds::from(1..3) + Bounds::from(2..3), Bounds::from(3..6));
    assert_eq!(Bounds::Exact(1) + Bounds::from(1..3), Bounds::from(2..4));
    assert_eq!(Bounds::from(1..3) + Bounds::Exact(1), Bounds::from(2..4));
    assert_eq!(Bounds::from(1..3) + Bounds::from(..), Bounds::from(..));
    assert_eq!(Bounds::from(1..3) + Bounds::from(1..), Bounds::from(2..));
    assert_eq!(Bounds::from(1..3) + Bounds::from(..3), Bounds::from(..6));
    assert_eq!(Bounds::from(..) + Bounds::from(1..3), Bounds::from(..));
}

#[test]
fn test_sub() {
    assert_eq!(Bounds::from(1..3) - Bounds::from(2..3),
               Bounds::Range(Some(Bound::exclusive(-2)), Some(Bound::exclusive(1))));
    assert_eq!(Bounds::Exact(1) - Bounds::from(1..3),
               Bounds::Range(Some(Bound::exclusive(-2)), Some(Bound::inclusive(0))));
    assert_eq!(Bounds::from(1..3) - Bounds::Exact(1), Bounds::from(0..2));
    assert_eq!(Bounds::from(1..3) - Bounds::from(..), Bounds::from(..));
    assert_eq!(Bounds::from(1..3) - Bounds::from(1..),
               Bounds::Range(None, Some(Bound::exclusive(2))));
    assert_eq!(Bounds::from(1..3) - Bounds::from(..3),
               Bounds::Range(Some(Bound::exclusive(-2)), None));
}

#[test]
fn test_mul() {
    assert_eq!(Bounds::Exact(1) * Bounds::from(1..3), Bounds::from(1..3));
    assert_eq!(Bounds::Exact(1) * Bounds::from(2..), Bounds::from(2..));
    assert_eq!(Bounds::Exact(1) * Bounds::from(..2), Bounds::from(..2));
    assert_eq!(Bounds::Exact(-1) * Bounds::from(1..3), Bounds::Range(Some(Bound::exclusive(-3)), Some(Bound::inclusive(-1))));
    assert_eq!(Bounds::Exact(-1) * Bounds::from(2..), Bounds::Range(None, Some(Bound::inclusive(-2))));
    assert_eq!(Bounds::Exact(-1) * Bounds::from(..2), Bounds::Range(Some(Bound::exclusive(-2)), None));
    assert_eq!(Bounds::Exact(0) * Bounds::from(2..3), Bounds::Exact(0));
    assert_eq!(Bounds::Exact(0) * Bounds::from(..), Bounds::Exact(0));
    assert_eq!(Bounds::Exact(0) * Bounds::from(2..), Bounds::Exact(0));
    assert_eq!(Bounds::Exact(0) * Bounds::from(..3), Bounds::Exact(0));
//
    assert_eq!(Bounds::<i32>::Range(None, None) * Bounds::Range(None, None), Bounds::Range(None, None));
    assert_eq!(Bounds::Range(None, None) * Bounds::from(1..2), Bounds::Range(None, None));
    assert_eq!(Bounds::from(1..2) * Bounds::from(..), Bounds::from(..));
    assert_eq!(Bounds::Exact(-1) * Bounds::from(1..), Bounds::Range(None, Some(Bound::inclusive(-1))));
    assert_eq!(Bounds::from(1..3) * Bounds::from(2..3), Bounds::from(2..9));
    assert_eq!(Bounds::from(1..3) * Bounds::Exact(1), Bounds::from(1..3));
    assert_eq!(Bounds::from(1..3) * Bounds::from(..), Bounds::from(..));
    assert_eq!(Bounds::from(..) * Bounds::from(1..3), Bounds::from(..));
    assert_eq!(Bounds::from(1..3) * Bounds::from(1..), Bounds::from(1..));
    assert_eq!(Bounds::from(1..) * Bounds::from(1..3), Bounds::from(1..));
    assert_eq!(Bounds::from(1..3) * Bounds::from(..3), Bounds::from(..9));
    assert_eq!(Bounds::from(1..3) * Bounds::Exact(0), Bounds::Exact(0));
    assert_eq!(Bounds::from(1..) * Bounds::Exact(1), Bounds::from(1..));
    assert_eq!(Bounds::from(-1..1) * Bounds::from(1..), Bounds::from(..));
    assert_eq!(Bounds::from(..1) * Bounds::from(1..), Bounds::from(..));
    assert_eq!(Bounds::from(-1..) * Bounds::from(1..), Bounds::from(..));
    assert_eq!(Bounds::from(-1..1) * Bounds::from(..), Bounds::from(..));
}
