#[cfg(test)]
mod tests {

    use factorizer::Factory;

    #[derive(Factory, Default, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]

    fn it_works() {
        let point = Point::new().x(1).y(2).clone();

        assert_eq!(point.x, 1);
        assert_eq!(point.y, 2);
    }
}
