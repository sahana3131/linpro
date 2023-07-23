
#[cfg(test)]
mod tests {

    use linpro::some_function;

    #[test]
    fn test_some_function() {

        assert_eq!(some_function(2), 4);
        assert_eq!(some_function(0), 0);
        assert_eq!(some_function(-5), 25);
    }
}

