mod tfidf;

#[cfg(test)]
mod tests {
    use tfidf::TfIdf;
    use tfidf::Term;

    fn setup<'a>() -> TfIdf<'a> {
        let mut tfidf = TfIdf::new();
        tfidf.add("hello hello hello world");

        tfidf
    }

    #[test]
    fn test_tf() {
        let tfidf = setup();

        assert_eq!(tfidf.tf(&Term("hello"), 0), 1.4771212f32);
        assert_eq!(tfidf.tf(&Term("world"), 0), 1f32);
        assert_eq!(tfidf.tf(&Term("nothing"), 0), 0f32);
    }
}
