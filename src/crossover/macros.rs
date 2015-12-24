macro_rules! join {
    ($source0: expr, $($source: expr),*) => {{
        $source0.iter()$(.chain($source.iter()))*
            .cloned().collect()
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
            let actual = $crossover.cross(&parents);
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
            let actual = $crossover.cross(&parents);
            if !expected.is_empty() {
                assert!(expected.len() == actual.len());
            }
            assert!(expected == actual);
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
            use problem::{Problem, Permutation};

            //let parents: Vec<Vec<$Type>> = vec![$(vec![$($parent),+]),+];
            let permutation = Permutation::from(0..10000);
            let parents = permutation.generate_population(2);
            let cross = $crossover;
            b.iter(|| {
                ::test::black_box(cross.cross(&parents));
            });
        }
    };
}

