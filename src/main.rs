pub struct Term<'a>(pub &'a str);

pub struct TfIdf<'a> {
    pub documents: Vec<Vec<Term<'a>>>,
}

impl<'a> PartialEq for Term<'a> {
    fn eq(&self, other: &Term) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl<'a> TfIdf<'a> {
    pub fn new() -> Self {
        Self { documents: vec![] }
    }

    pub fn add_document_vec(&mut self, document: Vec<Term<'a>>) {
        self.documents.push(document);
    }

    pub fn add_document_str(&mut self, document: &'a str) {
        let mut terms: Vec<Term<'a>> = vec![];

        for word in document.split(' ') {
            terms.push(Term(word));
        }

        self.add_document_vec(terms);
    }

    pub fn count_documents_containing_term(&self, term: &'a Term) -> i32 {
        let mut count: i32 = 0;

        for document in &self.documents {
            let inner_count = document.into_iter().filter(|&tx| tx == term).count();

            if inner_count > 0 {
                count += 1;
            }
        }

        count
    }

    pub fn idf_score(&self, term: &Term) -> f32 {
        if self.count_documents_containing_term(term) > 0 {
            (self.documents.len() as f32 / self.count_documents_containing_term(term) as f32)
                .log10()
        } else {
            0f32
        }
    }

    pub fn tf_score(&self, term: &Term, document_index: usize) -> f32 {
        let ref document: Vec<Term<'a>> = self.documents[document_index];

        let counts: f32 = document.into_iter().filter(|&dx| dx == term).count() as f32;

        //TODO: Rewrite this somehow, floating point pattern matching is going away
        return match counts {
            0f32 => 0f32,
            _ => counts.log10() + 1f32,
        };
    }

    pub fn tf_idf_score(&self, term: &'a Term, document_index: usize) -> f32 {
        self.tf_score(term, document_index) * self.idf_score(term)
    }

    pub fn similarities(&self, term: &'a Term) -> Vec<(usize, f32)> {
        let mut values: Vec<(usize, f32)> = vec![];

        for i in 0usize..self.documents.len() {
            values.push((i, self.tf_idf_score(term, i)));
        }

        values
    }

    pub fn sort_by_similarities(&self, term: &'a Term) {
        let mut sortedDocuments: Vec<Vec<Term<'a>>> = vec![];
        let similaritiesVec = self.similarities(term);

        todo!()
    }
}

fn main() {
    let d1 = "Rust is a good programming language. Rust has many good features.";
    let d2 = "Variables are by default immutable in Rust .";
    let d3 = "This is a test text.";
    let d4 = "Rust has a strong static type system.";

    let search_query = "Rust";
    let mut tf_idf = TfIdf::new();
    tf_idf.add_document_str(d1);
    tf_idf.add_document_str(d2);
    tf_idf.add_document_str(d3);
    tf_idf.add_document_str(d4);

    println!("{:#?}", tf_idf.similarities(&Term(search_query)));
}
