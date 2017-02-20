mod tfidf;

#[cfg(test)]
mod tests {
    use tfidf::Tf;
    use tfidf::Document;

    #[test]
    fn test_tf_one() {
        assert_eq!(Tf::add(
                "hello".to_string(),
                vec!["hello".to_string(), "world".to_string()]
        ), 1.0f32)
    }

    #[test]
    fn test_tf_two() {
        assert_eq!(Tf::add(
                "hello".to_string(),
                vec!["hello".to_string(), "world".to_string(), "hello".to_string()]
        ), 1.30103f32)
    }

    #[test]
    fn test_tf_zero() {
        assert_eq!(Tf::add(
                "boo".to_string(),
                vec!["hello".to_string(), "world".to_string()]
        ), 0.0f32)
    }
}
