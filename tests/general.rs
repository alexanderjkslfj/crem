#[cfg(test)]
mod tests {
    use crem::*;

    #[test]
    fn test_integers() {
        assert_eq!(Op::from(3), Op::from(3));
        assert_ne!(Op::from(2), Op::from(4));
        assert_eq!(Op::from(3).calc(), 3.0);
    }

    #[test]
    fn test_negation() {
        assert_ne!(Op::from(3), Op::from(-3));
        assert_eq!(Op::from(0), Op::from(-0));
        assert_eq!(-Op::from(3), Op::from(-3));
        assert_eq!(Op::from(-3).calc(), -3.0);
    }

    #[test]
    fn test_addition() {
        assert_eq!(Op::from(4) + Op::from(3), Op::from(7));
        assert_eq!(Op::from(0) + Op::from(0), Op::from(0));
        assert_eq!(Op::from(1) + 2.into() + 3.into() + 4.into(), 10.into());
        assert_eq!((Op::from(1) + Op::from(2)).calc(), 3.0);
        assert_eq!(Op::from(5) + Op::from(-3), Op::from(2));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(Op::from(7) - Op::from(4), Op::from(3));
        assert_eq!(Op::from(0) - Op::from(0), Op::from(0));
        assert_eq!(Op::from(10) - 2.into() - 3.into() - 4.into(), 1.into());
        assert_eq!(Op::from(1) - 2.into() - 3.into() - 4.into(), (-8i32).into());
        assert_eq!((Op::from(5) - Op::from(3)).calc(), 2.0);
        assert_eq!((Op::from(3) - Op::from(5)).calc(), -2.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(Op::div(3, 6), Op::div(1, 2));
        assert_eq!(Op::div(3, 6), Op::div(1, 2));
        assert_eq!(Op::div(3, 10).calc(), 0.3);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Op::from(2) * Op::from(3), Op::from(6));
        assert_eq!(Op::from(1) * Op::from(3), Op::from(3));
        assert_eq!(Op::from(0) * Op::from(3), Op::from(0));
    }

    #[test]
    fn test_adding_multiplications() {
        assert_eq!(
            (Op::from(3) * Op::from(6)) + (Op::from(2) * Op::from(5)),
            Op::from(28)
        );
        assert_eq!(
            (Op::from(5) * Op::div(1, 3)) + (Op::from(5) * Op::div(2, 3)),
            Op::from(5)
        );
        assert_eq!(
            (Op::from(5) * Op::div(1, 6)) + (Op::from(5) * Op::div(2, 6)),
            Op::div(5, 2)
        );
    }

    #[test]
    fn test_adding_divisions() {
        assert_eq!(Op::div(1, 10) + Op::div(2, 10), Op::div(3, 10));
        // assert_eq!(0.1 + 0.2, 0.3) would panic
        assert_eq!(
            (Op::div(1, 10) + Op::div(2, 10)).calc(),
            Op::div(3, 10).calc()
        );
        assert_eq!(Op::div(2, 3) + Op::div(1, 6), Op::div(5, 6));
        assert_eq!(Op::div(1, 3) + Op::div(2, 3), Op::from(1));
    }

    #[test]
    fn test_add_div_num() {
        assert_eq!(Op::div(1, 2) + 3.into(), Op::div(7, 2));
        assert_eq!(Op::from(5) + Op::div(1, 2), Op::div(11, 2));
    }

    #[test]
    fn test_nested_divisions() {
        assert_eq!(Op::from(5) / (Op::from(1) / Op::from(2)), Op::from(10));
        assert_eq!(
            (Op::from(3) / Op::from(2)) / Op::from(2),
            Op::from(3) / Op::from(4)
        );
        assert_eq!(Op::div(3, 2) / Op::div(1, 4), Op::from(6));
    }
}
