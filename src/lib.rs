mod tfidf;

#[cfg(test)]
mod tests {
    use tfidf::TfIdf;
    use tfidf::Term;

    #[test]
    fn test_tf_one() {
        let mut tfidf = TfIdf::new();
        let mut terms: Vec<Term> = vec![];
        terms.push(Term("hello".to_string()));
        terms.push(Term("hello".to_string()));
        terms.push(Term("hello".to_string()));
        terms.push(Term("world".to_string()));
        terms.push(Term("city".to_string()));

        tfidf.add(terms);

        assert_eq!(tfidf.tf(&Term("hello".to_string()), 0), 1.4771212f32);
        assert_eq!(tfidf.tf(&Term("world".to_string()), 0), 1f32);
        assert_eq!(tfidf.tf(&Term("city".to_string()), 0), 1f32);
        assert_eq!(tfidf.tf(&Term("nothing".to_string()), 0), 0f32);
    }
}
