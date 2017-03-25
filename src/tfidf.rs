/// Represents a document and its associated score
#[derive(Debug)]
pub struct WeightedTerm {
    /// Document
    pub value: String,
    /// Score of document
    pub score: f32
}

/// Represents a term
#[derive(Debug)]
pub struct Term(pub String);

/// Represents TF-IDF struct
#[derive(Debug)]
pub struct TfIdf {
    pub documents: Vec<Vec<Term>>
}

impl PartialEq for Term {
    fn eq(&self, other: &Term) -> bool {
        self.0 == other.0
    }
}

impl TfIdf {
    pub fn new() -> TfIdf {
        TfIdf { documents: vec![] }
    }

    pub fn add(&mut self, document: Vec<Term>) {
        self.documents.push(document);
    }

    pub fn tf(&self, term: &Term, document_index: usize) -> f32 {
        let ref document: Vec<Term> = self.documents[document_index];

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
