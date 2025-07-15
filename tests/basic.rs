#[cfg(test)]
mod tests {

    use factoryizer::Factory;

    #[derive(Factory, Default, Clone)]
    struct Point<T: Default> {
        x: i32,
        y: i32,
        #[skip]
        z: i32,
        t: T,
    }

    #[derive(Default, Clone)]
    struct Value(String);
    impl Into<Value> for &str {
        fn into(self) -> Value {
            Value(self.to_string())
        }
    }

    #[derive(Factory, Default, Clone)]
    struct Structure {
        value: Value,
    }

    #[test]
    fn it_works() {
        let point = Point::<usize>::new().x(1).y(2).clone();
        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }
}