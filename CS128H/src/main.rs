use std::collections::HashMap;

// Define a struct to hold question, answer, explanation, and metadata
#[derive(Debug, Clone)]
struct QAEntry {
    question: String,
    answer: String,
    explanation: String,
    metadata: HashMap<String, String>,  // Can store question type, difficulty, etc.
}

// Define a struct to represent the dataset
#[derive(Debug, Clone)]
struct QADataset {
    entries: Vec<QAEntry>,
}

impl QADataset {
    // Function to add a new entry to the dataset
    fn add_entry(&mut self, question: String, answer: String, explanation: String, metadata: HashMap<String, String>) {
        let entry = QAEntry {
            question,
            answer,
            explanation,
            metadata,
        };
        self.entries.push(entry);
    }

    // Function to initialize a new dataset
    fn new() -> QADataset {
        QADataset { entries: Vec::new() }
    }
}

fn main() {
    let mut dataset = QADataset::new();

    // Example of adding an entry
    let metadata = HashMap::from([
        ("question_type".to_string(), "fact-based".to_string()),
        ("difficulty".to_string(), "medium".to_string()),
    ]);

    dataset.add_entry(
        "What is the capital of France?".to_string(),
        "Paris".to_string(),
        "The capital of France is Paris, which is a well-known fact.".to_string(),
        metadata,
    );

    println!("{:?}", dataset);
}
