pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }

}

#[cfg(test)]
mod tests {
    use crate::either::Either;
    use crate::either::Either::{Left, Right};

    #[test]
    fn test() {
        let mut e: Either<bool, Option<usize>> = Right(Some(5));
        assert!(e.is_right());
        match e {
            Right(Some(x)) => assert_eq!(x, 5),
            _ => unreachable!(),
        }

        e = Left(true);
        assert!(e.is_left());
        match e {
            Left(b) => assert!(b),
            _ => unreachable!(),
        }

    }
}