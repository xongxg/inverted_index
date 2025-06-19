use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");

    let mut index = InvertedIndex::new();
    index.add(1, "Rust is safe and fast.");
    index.add(2, "Rust is a systems programming language.");
    index.add(3, "Programming in Rust is fun.");

    // query "Rust"
    let results = index.query("Rust");
    for result in results {
        println!("{}", result);
    }

    println!("");

    // query "Programming"
    let results = index.query("Programming");
    for result in results {
        println!("{}", result);
    }
}

/// Define a structure to represent documents for easy access and management.c
struct Document {
    id: usize,
    content: String,
}

/// The InvertedIndex struct manages a set of indexed documents.
struct InvertedIndex {
    /// The in-memory index.
    ///
    /// value is the single `term` of the document's word tokenization.
    /// key is a vector of document ids.
    indexes: HashMap<String, Vec<usize>>,

    /// Stores a mapping of the document id to the original document content
    documents: HashMap<usize, Document>,
}

impl InvertedIndex {
    fn new() -> InvertedIndex {
        Self {
            indexes: HashMap::new(),
            documents: HashMap::new(),
        }
    }

    /// Adds a document to the index.
    ///
    /// # Parameters
    /// - `doc_id`: An identifier for the document.
    /// - `content`: The text content of the document.
    ///
    /// # Notes
    /// This method processes the document by lowercasing and tokenizing the text,
    /// then updates the index to include words found in this document.
    pub fn add(&mut self, id: usize, content: &str) {
        let content_lowercase = content.to_lowercase();
        let words = tokenize(&content_lowercase);
        words.iter().for_each(|word| {
            self.indexes
                .entry(word.to_string())
                .or_insert(Vec::new())
                .push(id);
        });

        self.documents.insert(
            id,
            Document {
                id,
                content: content.to_string(),
            },
        );
    }

    /// Queries the index for documents containing a specified word and highlights them.
    ///
    /// # Parameters
    /// - `term`: The search term (word).
    ///
    /// # Returns
    /// A vector of document contents that contain the term, case-insensitively,
    /// with all occurrences of the term highlighted in purple.
    fn query(&self, term: &str) -> Vec<String> {
        let term_lowercase = term.to_lowercase();
        if let Some(doc_ids) = self.indexes.get(&term_lowercase) {
            doc_ids
                .iter()
                .filter_map(|doc_id| {
                    self.documents
                        .get(&doc_id)
                        .map(|doc| highlight(&term_lowercase, &doc.content))
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

/// Break a string into words
fn tokenize(text: &str) -> Vec<&str> {
    text.split(|ch: char| !ch.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Highlights all occurrences of `term` in `content` with a <font color"purple">purple</font> color.
fn highlight(term: &str, content: &str) -> String {
    let regex = Regex::new(&format!(r"(?i){}", term)).unwrap();
    let highlighted_content = regex
        .replace_all(content, |caps: &regex::Captures| {
            caps[0].to_string().purple().to_string()
        })
        .to_string();

    highlighted_content
}

#[test]
fn tokenize_test() {
    assert_eq!(
        tokenize("This is\nhedon's tokenize function."),
        vec!["This", "is", "hedon", "s", "tokenize", "function"]
    )
}

#[test]
fn highlight_test() {
    assert_eq!(
        highlight("programming", "I like programming with Rust Programming"),
        "I like \u{1b}[35mprogramming\u{1b}[0m with Rust \u{1b}[35mProgramming\u{1b}[0m"
    );
}
