/// Represents a document and its associated score
#[derive(Debug)]
pub struct Document {
    /// Document
    pub doc: String,
    /// Score of document
    pub score: f32
}

/// Represents TF struct, vector of Documents
#[derive(Debug)]
pub struct Tf(pub Vec<Document>);

impl PartialEq for Document {
    fn eq(&self, other: &Document) -> bool {
        self.doc == other.doc && self.score == other.score
    }
}

impl PartialEq for Tf {
    fn eq(&self, other: &Tf) -> bool {
        self.0 == other.0
    }
}

impl Tf {
    pub fn add(doc: String, raw_docs: Vec<String>) -> f32 {
        let counts: f32 = raw_docs.into_iter().filter(
            |dx| dx.to_string() == doc.to_string()
        ).count() as f32;

        return match counts {
            0.0f32 => 0.0f32,
            _ => {
                counts.log10() + 1.0f32
            }
        };
    }
}
