#[cfg(test)]
mod tests {
    use crem::*;

    #[test]
    fn test_integers() {
        assert_eq!(Term::from(3), Term::from(3));
        assert_ne!(Term::from(2), Term::from(4));
        assert_eq!(Term::from(3).calc::<f64>(), 3.0);
    }

    #[test]
    fn test_negation() {
        assert_ne!(-Term::from(3), Term::from(3));
        assert_eq!((-Term::from(3)).calc::<f64>(), -3.0);
        assert_eq!(Term::from(-3).calc::<f64>(), -3.0);
    }

    #[test]
    fn test_addition() {
        assert_eq!(Term::from(4) + Term::from(3), Term::from(7));
        assert_eq!(Term::from(0) + Term::from(0), Term::from(0));
        assert_eq!(Term::from(1) + 2.into() + 3.into() + 4.into(), 10.into());
        assert_eq!((Term::from(1) + Term::from(2)).calc::<f64>(), 3.0);
        assert_eq!(Term::from(5) + Term::from(-3), Term::from(2));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(Term::from(7) - Term::from(4), Term::from(3));
        assert_eq!(Term::from(0) - Term::from(0), Term::from(0));
        assert_eq!(Term::from(10) - 2.into() - 3.into() - 4.into(), 1.into());
        assert_eq!(
            Term::from(1) - 2.into() - 3.into() - 4.into(),
            -Term::from(8)
        );
        assert_eq!((Term::from(5) - Term::from(3)).calc::<f64>(), 2.0);
        assert_eq!((Term::from(3) - Term::from(5)).calc::<f64>(), -2.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(Term::div(3, 6), Term::div(1, 2));
        assert_eq!(Term::div(3, 6).calc::<f64>(), Term::div(1, 2).calc::<f64>());
        assert_eq!(Term::div(3, 10).calc::<f64>(), 0.3);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(Term::from(2) * Term::from(3), Term::from(6));
        assert_eq!(Term::from(1) * Term::from(3), Term::from(3));
        assert_eq!(Term::from(0) * Term::from(3), Term::from(0));
    }

    #[test]
    fn test_assign() {
        {
            let mut a = Term::from(3);
            a += Term::from(4);
            assert_eq!(a, Term::from(7));
        }
        {
            let mut a = Term::from(3);
            a -= Term::from(4);
            assert_eq!(a, -Term::from(1));
        }
        {
            let mut a = Term::from(3);
            a *= Term::from(4);
            assert_eq!(a, Term::from(12));
        }
        {
            let mut a = Term::from(8);
            a /= Term::from(2);
            assert_eq!(a, Term::from(4));
        }
    }

    #[test]
    fn test_adding_multiplications() {
        assert_eq!(
            (Term::from(3) * Term::from(6)) + (Term::from(2) * Term::from(5)),
            Term::from(28)
        );
        assert_eq!(
            (Term::from(5) * Term::div(1, 3)) + (Term::from(5) * Term::div(2, 3)),
            Term::from(5)
        );
        assert_eq!(
            (Term::from(5) * Term::div(1, 6)) + (Term::from(5) * Term::div(2, 6)),
            Term::div(5, 2)
        );
    }

    #[test]
    fn test_adding_divisions() {
        assert_eq!(Term::div(1, 10) + Term::div(2, 10), Term::div(3, 10));
        // assert_eq!(0.1 + 0.2, 0.3) would panic
        assert_eq!(
            (Term::div(1, 10) + Term::div(2, 10)).calc::<f64>(),
            Term::div(3, 10).calc::<f64>()
        );
        assert_eq!(Term::div(2, 3) + Term::div(1, 6), Term::div(5, 6));
        assert_eq!(Term::div(1, 3) + Term::div(2, 3), Term::from(1));
    }

    #[test]
    fn test_multiplying_divisions() {
        assert_eq!(Term::div(1, 2) * Term::div(1, 2), Term::div(1, 4));
        assert_eq!(Term::div(6, 7) * Term::div(1, 2), Term::div(3, 7));
    }

    #[test]
    fn test_add_div_num() {
        assert_eq!(Term::div(1, 2) + 3.into(), Term::div(7, 2));
        assert_eq!(Term::from(5) + Term::div(1, 2), Term::div(11, 2));
    }

    #[test]
    fn test_nested_divisions() {
        assert_eq!(
            Term::from(5) / (Term::from(1) / Term::from(2)),
            Term::from(10)
        );
        assert_eq!(
            (Term::from(3) / Term::from(2)) / Term::from(2),
            Term::from(3) / Term::from(4)
        );
        assert_eq!(Term::div(3, 2) / Term::div(1, 4), Term::from(6));
    }

    #[test]
    fn test_set_variable() {
        {
            let mut a = Term::new_variable("a");
            a.set_variable("a", &Term::from(5));
            assert_eq!(a, Term::from(5)); // check if simple variable setting works
        }
        {
            let mut a = Term::new_variable("a");
            a.set_variable("b", &Term::from(5));
            assert_ne!(a, Term::from(5)); // check if setting wrong name fails
        }
        {
            let mut term = Term::from(5)
                + (Term::from(3) * (Term::from(4) / (Term::from(7) - Term::new_variable("x"))));
            term.set_variable("x", &Term::from(5));
            assert_eq!(term.calc::<f64>(), 11.0); // check if deep variable setting works
            assert_eq!(term, Term::from(11)); // check if deep variable setting simplifies correctly
        }
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Term::try_from("5").unwrap(), Term::from(5));
        assert_eq!(Term::try_from("3 + 4").unwrap(), Term::from(7));
        assert_eq!(
            Term::try_from("0.1 + 0.2").unwrap(),
            Term::try_from("3 / 10").unwrap()
        );
        assert_eq!(Term::try_from("10 + 8 / 2").unwrap(), Term::from(14));
        assert_eq!(Term::try_from("5+3*3+5").unwrap(), Term::from(19));
        assert_eq!(Term::try_from("3(4+5)").unwrap(), Term::from(27));
        assert_eq!(Term::try_from("8*----2").unwrap(), Term::from(16));
        assert_eq!(Term::try_from("8*-----2").unwrap(), -Term::from(16));
    }

    #[test]
    fn test_convert() {
        assert_eq!(Term::from(3i64), Term::from(3u32).convert());
        assert_ne!(Term::from(2i64), Term::from(3u32).convert());
    }
}
