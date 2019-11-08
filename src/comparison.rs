use std::ops::Neg;

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
            Comparison::Greater => Comparison::Less,
        }
    }
}
