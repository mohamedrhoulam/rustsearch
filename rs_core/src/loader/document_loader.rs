use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub struct Document {
    pub document_id: String,
    pub document_name: String,
    pub document_content: String,
}

pub struct DocumentLoader {
    pub source: String,
    pub documents: VecDeque<Document>,
}

impl DocumentLoader {
    pub fn new(source: String) -> Self {
        Self {
            source,
            documents: VecDeque::new(),
        }
    }

    pub fn load_document(&mut self) {
        if Path::new(&self.source).is_dir() {
            for entry in fs::read_dir(&self.source).expect("Unable to read directory") {
                let entry = entry.expect("Unable to read entry");
                let path = entry.path();
                let document_id = Uuid::max().simple().to_string();
                if path.is_file() {
                    let document_content = fs::read_to_string(&path).expect("Unable to read file");
                    let document_name = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .map(|name| name.to_string())
                        .expect("Unable to get file name");
                    self.documents.push_back(Document {
                        document_id,
                        document_name,
                        document_content,
                    });
                }
            }
        } else {
        }
    }
}
