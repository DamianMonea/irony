mod value;
use value::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ops_test() {
        let x = Value::from(10);
        let y = Value::from(5);
        let mut z = x + y;
        assert_eq!(z.data, 15);
        z = x - y;
        assert_eq!(z.data, 5);
        z = x * y;
        assert_eq!(z.data, 50);
        z = x / y;
        assert_eq!(z.data, 2);
        z = Value::from(2);
        z += y;
        assert_eq!(z.data, 7);
        z = y % x;
        assert_eq!(z.data, 5);
        z.pow(2);
        assert_eq!(z.data, 25);
    }

    #[test]
    fn misc() {
    }
}
