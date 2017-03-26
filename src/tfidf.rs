/// Represents a document and its associated score
#[derive(Debug)]
pub struct WeightedTerm<'a> {
    /// Document
    pub value: &'a str,
    /// Score of document
    pub score: f32
}

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
    pub fn new() -> TfIdf<'a> {
        TfIdf { documents: vec![] }
    }

    pub fn add_vec(&mut self, document: Vec<Term<'a>>) {
        self.documents.push(document);
    }

    pub fn add(&mut self, document: &'a str) {
        let mut terms: Vec<Term<'a>> = vec![];

        for word in document.split(' ') {
            terms.push(Term(word));
        }

        self.add_vec(terms);
    }

    //pub fn idf(&self, term: &Term, )

    pub fn tf(&self, term: &'a Term, document_index: usize) -> f32 {
        let ref document: Vec<Term<'a>> = self.documents[document_index];

        let counts: f32 = document.into_iter().filter(
            |dx| dx.0 == term.0
        ).count() as f32;

        return match counts {
            0.0f32 => 0.0f32,
            _ => {
                counts.log10() + 1.0f32
            }
        };
    }
}
