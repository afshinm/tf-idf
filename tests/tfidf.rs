extern crate tfidf;

#[cfg(test)]
mod tests {
    use tfidf::tfidf::TfIdf;
    use tfidf::tfidf::Term;

    fn setup<'a>() -> TfIdf<'a> {
        let mut tfidf = TfIdf::new();
        tfidf.add("hello hello hello world");
        tfidf.add("soft kitty warm kitty");
        tfidf.add("we have to find a leader for this world");

        tfidf
    }

    #[test]
    fn test_tf() {
        let tfidf = setup();

        assert_eq!(tfidf.tf(&Term("hello"), 0), 1.4771212f32);
        assert_eq!(tfidf.tf(&Term("world"), 0), 1f32);
        assert_eq!(tfidf.tf(&Term("nothing"), 0), 0f32);
        assert_eq!(tfidf.tf(&Term("warm"), 0), 0f32);
    }

    #[test]
    fn test_count() {
        let tfidf = setup();

        assert_eq!(tfidf.count(&Term("hello")), 1);
        assert_eq!(tfidf.count(&Term("world")), 2);
        assert_eq!(tfidf.count(&Term("kitty")), 1);
        assert_eq!(tfidf.count(&Term("nothing")), 0);
    }

    #[test]
    fn test_idf() {
        let tfidf = setup();

        assert_eq!(tfidf.idf(&Term("soft")), 0.17609125f32);
        assert_eq!(tfidf.idf(&Term("hello")), 0.17609125f32);
        assert_eq!(tfidf.idf(&Term("nothing")), 0.47712126f32);
    }

    #[test]
    fn test_tfidf() {
        let tfidf = setup();

        assert_eq!(tfidf.tfidf(&Term("soft"), 0), 0f32);
        assert_eq!(tfidf.tfidf(&Term("soft"), 1), 0.17609125f32);
        assert_eq!(tfidf.tfidf(&Term("hello"), 0), 0.26010814f32);
        assert_eq!(tfidf.tfidf(&Term("nothing"), 1), 0f32);
    }
}
