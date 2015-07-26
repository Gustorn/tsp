macro_rules! pre_crossover {
    ($rng: expr, $rate: expr, $($parent: expr),+) => {
        if $rng.next_f64() >= $rate {
            return vec![$($parent.clone()),+];
        }
    };
}

macro_rules! join {
    ($source0: expr, $($source: expr),*) => {{
        $source0.iter()$(.chain($source.iter()))*
            .cloned().collect::<Vec<_>>()
    }};
}

macro_rules! test_crossover {
    ($Name: ident, $Type: ty, $crossover: expr,
     $(parent($($parent: expr),+)),+
     $(,child($($child: expr),*))*) => {

        #[test]
        fn $Name() {
            use super::*;
            use crossover::Crossover;

            let parents: Vec<Vec<$Type>> = vec![$(vec![$($parent),+]),+];
            let expected: Vec<Vec<$Type>> = vec![$(vec![$($child),*]),*];
            let actual = $crossover.cross(&parents, 1.0);
            if !expected.is_empty() {
                assert!(expected.len() == actual.len());
            }
            assert!(expected == actual);
        }
    };
}

macro_rules! test_crossover_panic {
    ($Name: ident, $Type: ty, $crossover: expr,
     $(parent($($parent: expr),+)),+
     $(,child($($child: expr),*))*) => {

        #[test]
        #[should_panic]
        fn $Name() {
            use super::*;
            use crossover::Crossover;

            let parents: Vec<Vec<$Type>> = vec![$(vec![$($parent),+]),+];
            let expected: Vec<Vec<$Type>> = vec![$(vec![$($child),*]),*];
            let actual = $crossover.cross(&parents, 1.0);
            if !expected.is_empty() {
                assert!(expected.len() == actual.len());
            }
            assert!(expected == actual);
        }
    };
}

macro_rules! test_crossover_passthrough {
    ($Name: ident, $Type: ty, $crossover: expr,
     $(parent($($parent: expr),+)),+) => {

        #[test]
        fn $Name() {
            use super::*;
            use crossover::Crossover;

            let parents: Vec<Vec<$Type>> = vec![$(vec![$($parent),+]),+];
            let actual = $crossover.cross(&parents, 0.0);
            assert!(parents == actual);
        }
    };
}

macro_rules! bench_crossover {
    ($Name: ident, $Type: ty, $crossover: expr,
     $(parent($($parent: expr),+)),+) => {
        #[bench]
        fn $Name(b: &mut ::test::Bencher) {
            use super::*;
            use crossover::Crossover;

            let parents: Vec<Vec<$Type>> = vec![$(vec![$($parent),+]),+];
            let cross = $crossover;
            b.iter(|| {
                ::test::black_box(cross.cross(&parents, 1.0));
            });
        }
    };
}
