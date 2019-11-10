mod bound;
mod bounds;
mod comparison;
pub(crate) mod sign_bounds;

pub use crate::bound::{Bound, BoundType};
pub use crate::bounds::Bounds;
pub use crate::comparison::Comparison;

#[cfg(test)]
mod test;

#[macro_export]
macro_rules! bounds {
    ($val:expr) => {
        $crate::Bounds::Exact($val)
    };
    ($a:expr,$b:expr) => {
        $crate::Bounds::Range(
            Some($crate::Bound::inclusive($a)),
            Some($crate::Bound::inclusive($b)),
        )
    };
    (~$a:expr,$b:expr) => {
        $crate::Bounds::Range(
            Some($crate::Bound::exclusive($a)),
            Some($crate::Bound::inclusive($b)),
        )
    };
    ($a:expr,~$b:expr) => {
        $crate::Bounds::Range(
            Some($crate::Bound::inclusive($a)),
            Some($crate::Bound::exclusive($b)),
        )
    };
    (~$a:expr,~$b:expr) => {
        $crate::Bounds::Range(
            Some($crate::Bound::exclusive($a)),
            Some($crate::Bound::exclusive($b)),
        )
    };
    (,$b:expr) => {
        $crate::Bounds::Range(None, Some($crate::Bound::inclusive($b)))
    };
    (,~$b:expr) => {
        $crate::Bounds::Range(None, Some($crate::Bound::exclusive($b)))
    };
    ($a:expr,) => {
        $crate::Bounds::Range(Some($crate::Bound::inclusive($a)), None)
    };
    (~$a:expr,) => {
        $crate::Bounds::Range(Some($crate::Bound::exclusive($a)), None)
    };
    (,) => {
        $crate::Bounds::Range(None, None)
    };
}

#[test]
fn test_macro() {
    assert_eq!(bounds!(3), Bounds::Exact(3));
    assert_eq!(
        bounds!(1, 2),
        Bounds::Range(Some(Bound::inclusive(1)), Some(Bound::inclusive(2)))
    );
    assert_eq!(
        bounds!(~1, 2),
        Bounds::Range(Some(Bound::exclusive(1)), Some(Bound::inclusive(2)))
    );
    assert_eq!(
        bounds!(1, ~2),
        Bounds::Range(Some(Bound::inclusive(1)), Some(Bound::exclusive(2)))
    );
    assert_eq!(
        bounds!(~1, ~2),
        Bounds::Range(Some(Bound::exclusive(1)), Some(Bound::exclusive(2)))
    );
    assert_eq!(bounds!(,2), Bounds::Range(None, Some(Bound::inclusive(2))));
    assert_eq!(bounds!(,~2), Bounds::Range(None, Some(Bound::exclusive(2))));
    assert_eq!(bounds!(1,), Bounds::Range(Some(Bound::inclusive(1)), None));
    assert_eq!(bounds!(,), Bounds::<u32>::Range(None, None));
}
