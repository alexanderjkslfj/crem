#[cfg(test)]
mod tests {
    use crem::*;

    #[test]
    fn test_addition() {
        {
            assert_eq!(Op::from(4) + Op::from(3), Op::from(7));
            assert_eq!(Op::from(0) + Op::from(0), Op::from(0));
            assert_eq!(
                Op::from(1) + 2.into() + 3.into() + 4.into(),
                10.into()
            );
            assert_eq!((Op::from(1) + Op::from(2)).calc(), 3.0);
        }
    }

    #[test]
    fn test_division() {
        assert_eq!(
            Op::div(3, 5),
            Op::Division(Division {
                divident: Box::new(3.into()),
                divisor: Box::new(5.into())
            })
        );
        assert_eq!(Op::div(3, 6), Op::div(1, 2));
        assert_eq!(
            Op::div(3, 6),
            Op::Division(Division {
                divident: Box::new(1.into()),
                divisor: Box::new(2.into())
            })
        );
        assert_eq!(Op::div(3, 10).calc(), 0.3);
    }

    #[test]
    fn test_adding_divisions() {
        assert_eq!(
            Op::div(1, 10) + Op::div(2, 10),
            Op::div(3, 10)
        );
        // assert_eq!(0.1 + 0.2, 0.3) would panic
        assert_eq!(
            (Op::div(1, 10) + Op::div(2, 10)).calc(),
            Op::div(3, 10).calc()
        );
        assert_eq!(
            Op::div(2, 3) + Op::div(1, 6),
            Op::div(5, 6)
        );
    }
}
