/// Represents a term
#[derive(Debug)]
pub struct Term<'a>(pub &'a str);

/// Represents TF-IDF struct
#[derive(Debug)]
pub struct TfIdf<'a> {
    pub documents: Vec<Vec<Term<'a>>>
}

impl<'a> PartialEq for Term<'a> {
    fn eq(&self, other: &Term) -> bool {
        self.0 == other.0
    }
}

impl<'a> TfIdf<'a> {
    /// Returns a TF-IDF object
    ///
    /// # Examples
    ///
    /// ```
    /// use tfidf::tfidf::TfIdf;
    /// let tfidf = TfIdf::new();
    /// ```
    pub fn new() -> TfIdf<'a> {
        TfIdf { documents: vec![] }
    }

    /// Adding a vector of Terms
    ///
    /// # Examples
    ///
    /// ```
    /// use tfidf::tfidf::TfIdf;
    /// use tfidf::tfidf::Term;
    ///
    /// let mut terms: Vec<Term> = vec![];
    /// terms.push(Term("Hello world"));
    ///
    /// let mut tfidf = TfIdf::new();
    /// tfidf.add_vec(terms);
    /// ```
    pub fn add_vec(&mut self, document: Vec<Term<'a>>) {
        self.documents.push(document);
    }

    /// Adding a new document
    ///
    /// # Examples
    ///
    /// ```
    /// use tfidf::tfidf::TfIdf;
    ///
    /// let mut tfidf = TfIdf::new();
    /// tfidf.add("hello world");
    /// ```
    pub fn add(&mut self, document: &'a str) {
        let mut terms: Vec<Term<'a>> = vec![];

        for word in document.split(' ') {
            terms.push(Term(word));
        }

        self.add_vec(terms);
    }

    /// Returns the count of documents containing a specific Term
    ///
    /// # Examples
    ///
    /// ```
    /// use tfidf::tfidf::TfIdf;
    /// use tfidf::tfidf::Term;
    ///
    /// let tfidf = TfIdf::new();
    /// tfidf.count(&Term("hello"));
    /// ```
    pub fn count(&self, term: &'a Term) -> i32 {
        let mut count: i32 = 0;

        for document in &self.documents {
            let inner_count = document.into_iter().filter(
                |&tx| tx == term
            ).count();

            if inner_count > 0 {
                count += 1;
            }
        }

        count
    }

    /// Returns the IDF of a specific Term
    ///
    /// # Examples
    ///
    /// ```
    /// use tfidf::tfidf::TfIdf;
    /// use tfidf::tfidf::Term;
    ///
    /// let mut tfidf = TfIdf::new();
    /// tfidf.idf(&Term("hello"));
    /// ```
    pub fn idf(&self, term: &Term) -> f32 {
        let docs = &self.documents;
        let _idf: f32 = docs.len() as f32 / (1f32 + self.count(term) as f32);

        _idf.log10()
    }

    /// Returns the TF of a specific Term
    ///
    /// # Examples
    ///
    /// ```
    /// use tfidf::tfidf::TfIdf;
    /// use tfidf::tfidf::Term;
    ///
    /// let mut tfidf = TfIdf::new();
    /// tfidf.add("hello world");
    /// tfidf.tf(&Term("hello"), 0);
    /// ```
    pub fn tf(&self, term: &'a Term, document_index: usize) -> f32 {
        let ref document: Vec<Term<'a>> = self.documents[document_index];

        let counts: f32 = document.into_iter().filter(
            |&dx| dx == term
        ).count() as f32;

        return match counts {
            0.0f32 => 0.0f32,
            _ => {
                counts.log10() + 1.0f32
            }
        };
    }

    /// Returns a specific TF-IDF of a Term
    ///
    /// # Examples
    ///
    /// ```
    /// use tfidf::tfidf::TfIdf;
    /// use tfidf::tfidf::Term;
    ///
    /// let mut tfidf = TfIdf::new();
    /// tfidf.add("hello world");
    /// tfidf.tfidf(&Term("hello"), 0);
    /// ```
    pub fn tfidf(&self, term: &'a Term, document_index: usize) -> f32 {
        &self.tf(term, document_index) * &self.idf(term)
    }
}
