macro_rules! assert_approx_eq {
    ($lhs:expr, $rhs:expr, $tol: expr) => {{
        assert!(::approx::eq(&$lhs, &$rhs, ::approx::Abs::tol($tol)));
    }};
}

macro_rules! forward_as {
    ($Name: ident, $Type: ty, $field: ident) => {
        impl<T> AsRef<[$Type]> for $Name<T> {
            fn as_ref(&self) -> &[$Type] {
                &self.$field
            }
        }

        impl<T> AsRef<$Name<T>> for $Name<T> {
            fn as_ref(&self) -> &$Name<T> {
                self
            }
        }

        impl<T> AsMut<[$Type]> for $Name<T> {
            fn as_mut(&mut self) -> &mut [$Type] {
                &mut self.$field
            }
        }

        impl<T> AsMut<$Name<T>> for $Name<T> {
            fn as_mut(&mut self) -> &mut $Name<T> {
                self
            }
        }

        impl<T> ::std::ops::Deref for $Name<T> {
            type Target = [$Type];

            fn deref(&self) -> &[$Type] {
                &self.$field
            }
        }

        impl<T> ::std::ops::DerefMut for $Name<T> {
            fn deref_mut(&mut self) -> &mut [$Type] {
                &mut self.$field
            }
        }
    };
}

macro_rules! forward_index {
    ($Name: ident, $field: ident, Index($Index: ty, $Out: ty)) => {
        impl<T> ::std::ops::Index<$Index> for $Name<T> {
            type Output = $Out;

            fn index(&self, index: $Index) -> &$Out {
                &self.$field[index]
            }
        }

        impl<T> ::std::ops::IndexMut<$Index> for $Name<T> {
            fn index_mut(&mut self, index: $Index) -> &mut $Out {
                &mut self.$field[index]
            }
        }
    };

    ($Name: ident, $Type: ty, $field: ident) => {
        forward_index!($Name, $field, Index(usize, $Type));
        forward_index!($Name, $field, Index(::std::ops::Range<usize>, [$Type]));
        forward_index!($Name, $field, Index(::std::ops::RangeFrom<usize>, [$Type]));
        forward_index!($Name, $field, Index(::std::ops::RangeTo<usize>, [$Type]));
        forward_index!($Name, $field, Index(::std::ops::RangeFull, [$Type]));
    };
}

macro_rules! forward_into_iter {
    ($Name: ident, $Type: ty, $field: ident) => {
        impl<T> IntoIterator for $Name<T> {
            type Item = $Type;
            type IntoIter = ::std::vec::IntoIter<$Type>;

            fn into_iter(self) -> ::std::vec::IntoIter<$Type> {
                self.$field.into_iter()
            }
        }

        impl<'a, T> IntoIterator for &'a $Name<T> {
            type Item = &'a $Type;
            type IntoIter = ::std::slice::Iter<'a, $Type>;

            fn into_iter(self) -> ::std::slice::Iter<'a, $Type> {
                self.$field.iter()
            }
        }

        impl<'a, T> IntoIterator for &'a mut $Name<T> {
            type Item = &'a mut $Type;
            type IntoIter = ::std::slice::IterMut<'a, $Type>;

            fn into_iter(self) -> ::std::slice::IterMut<'a, $Type> {
                self.$field.iter_mut()
            }
        }
    };
}

macro_rules! samples {
    ($from: expr, $to: expr, $rng: expr) => {{
        use ::rand::Rng;
        ::itertools::RepeatCall::new(move || $rng.gen_range($from, $to))
    }};
    ($from: expr, $to: expr) => {{
        samples!($from, $to, ::rand::thread_rng())
    }};
}

macro_rules! unique_samples {
    ($from: expr, $to: expr, $rng: expr) => {{
        use itertools::Itertools;
        samples!($from, $to, $rng).unique()
    }};
    ($from: expr, $to: expr) => {{
        unique_samples!($from, $to, ::rand::thread_rng())
    }};
}
