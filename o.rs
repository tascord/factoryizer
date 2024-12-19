#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[cfg(test)]
mod tests {
    use factoryizer::Factory;
    struct Point<T: Default> {
        x: i32,
        y: i32,
        #[skip]
        z: i32,
        t: T,
    }
    impl<T: Default> Point<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn x(mut self, value: i32) -> Self {
            self.x = value;
            self
        }
        pub fn y(mut self, value: i32) -> Self {
            self.y = value;
            self
        }
        pub fn t(mut self, value: T) -> Self {
            self.t = value;
            self
        }
    }
    #[automatically_derived]
    impl<T: ::core::default::Default + Default> ::core::default::Default for Point<T> {
        #[inline]
        fn default() -> Point<T> {
            Point {
                x: ::core::default::Default::default(),
                y: ::core::default::Default::default(),
                z: ::core::default::Default::default(),
                t: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone + Default> ::core::clone::Clone for Point<T> {
        #[inline]
        fn clone(&self) -> Point<T> {
            Point {
                x: ::core::clone::Clone::clone(&self.x),
                y: ::core::clone::Clone::clone(&self.y),
                z: ::core::clone::Clone::clone(&self.z),
                t: ::core::clone::Clone::clone(&self.t),
            }
        }
    }
    struct Value(String);
    #[automatically_derived]
    impl ::core::default::Default for Value {
        #[inline]
        fn default() -> Value {
            Value(::core::default::Default::default())
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Value {
        #[inline]
        fn clone(&self) -> Value {
            Value(::core::clone::Clone::clone(&self.0))
        }
    }
    impl Into<Value> for &str {
        fn into(self) -> Value {
            Value(self.to_string())
        }
    }
    #[into]
    struct Structure {
        value: Value,
    }
    impl Structure {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn value<T>(&mut self, value: T) -> &mut Self
        where
            T: Into<Value>,
        {
            self.value = Into::into(value);
            self
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Structure {
        #[inline]
        fn default() -> Structure {
            Structure {
                value: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Structure {
        #[inline]
        fn clone(&self) -> Structure {
            Structure {
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::it_works"]
    pub const it_works: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::it_works"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/basic.rs",
            start_line: 30usize,
            start_col: 8usize,
            end_line: 30usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(it_works()),
        ),
    };
    fn it_works() {
        let point = Point::<usize>::new().x(1).y(2).clone();
        match (&point.x, &1) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (&point.y, &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    extern crate test;
    #[cfg(test)]
    #[rustc_test_marker = "tests::into"]
    pub const into: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("tests::into"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/basic.rs",
            start_line: 37usize,
            start_col: 8usize,
            end_line: 37usize,
            end_col: 12usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::IntegrationTest,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(into())),
    };
    fn into() {
        let structure = Structure::new().value("Hello").clone();
    }
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&into, &it_works])
}
