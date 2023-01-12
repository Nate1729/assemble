/// Return the 2nd element in the iterator, only when the iterator
/// has length 2
pub fn parse_args(iter: impl Iterator<Item = String>) -> Result<String, &'static str> {
    let args: Vec<String> = iter.collect();
    if args.len() == 1 {
        return Err("No file name recieved!");
    } else if args.len() > 2 {
        return Err("Unrecognized command. Too many inputs were provided!");
    }

    return Ok(args[1].clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    /// An iterator of length one returns and `Err`
    #[test]
    fn test_parse_args_iterator_size_1() {
        // Arrange
        let test_iter = vec![String::from("Hi")].into_iter();
        let expected: Result<String, &'static str> = Err("No file name recieved!");

        // Act
        let result = parse_args(test_iter);

        // Assert
        assert_eq!(expected, result);
    }

    /// An iterator with length 2 should return an `Ok`
    /// containing the 2nd member as a string
    #[test]
    fn test_parse_args_iterator_size_2() {
        // Arrange
        let test_iter = vec![String::from("Hi"), String::from("foobar")].into_iter();
        let expected: Result<String, &'static str> = Ok(String::from("foobar"));

        // Act
        let result = parse_args(test_iter);

        // Assert
        assert_eq!(result, expected);
    }

    /// An iterator with length 3 should return an `Err`
    #[test]
    fn test_parse_args_iterator_size_3() {
        // Arrange
        let test_iter = vec![
            String::from("Hi"),
            String::from("foobar"),
            String::from("crunch"),
        ]
        .into_iter();
        let expected: Result<String, &'static str> =
            Err("Unrecognized command. Too many inputs were provided!");

        // Act
        let result = parse_args(test_iter);

        // Assert
        assert_eq!(result, expected);
    }
}
