use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug)]
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
            documents: VecDeque::<Document>::new(),
        }
    }

    // this is terrible
    // note to self: does this pushes to the top of the stack or the enqueues it ?!
    // TODO: Sort documents by name in the queue
    pub fn load(&mut self) {
        if Path::new(&self.source).is_dir() {
            for entry in fs::read_dir(&self.source).expect("Unable to read directory") {
                let entry = entry.expect("Unable to read entry");
                let path = entry.path();
                let document_id = Uuid::new_v4().to_string();
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
        } else if Path::new(&self.source).is_file() {
            let document_id = Uuid::new_v4().to_string();
            let document_content = fs::read_to_string(&self.source).expect("Unable to read file");
            let document_name = Path::new(&self.source)
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

    pub fn next(&mut self) -> Option<Document> {
        self.documents.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write, path::PathBuf};

    use fs::File;

    use super::*;

    fn setup_test_directory() -> PathBuf {
        let test_dir = PathBuf::from("test_documents");
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).expect("Unable to remove test directory");
        }
        fs::create_dir(&test_dir).expect("Unable to create test directory");

        let mut file1 = File::create(test_dir.join("doc1.txt")).expect("Unable to create file");
        file1
            .write_all(b"Content of document 1")
            .expect("Unable to write to file");

        let mut file2 = File::create(test_dir.join("doc2.txt")).expect("Unable to create file");
        file2
            .write_all(b"Content of document 2")
            .expect("Unable to write to file");

        test_dir
    }

    #[test]
    fn test_load_from_directory() {
        // what is going on here
        let test_dir = setup_test_directory();
        let mut loader = DocumentLoader::new(test_dir.to_str().unwrap().to_string());
        loader.load();

        assert_eq!(loader.documents.len(), 2);

        let doc1 = loader.next().unwrap();
        assert_eq!(doc1.document_name, "doc2.txt");
        assert_eq!(doc1.document_content, "Content of document 2");

        let doc2 = loader.next().unwrap();
        assert_eq!(doc2.document_name, "doc1.txt");
        assert_eq!(doc2.document_content, "Content of document 1");

        fs::remove_dir_all(test_dir).expect("Unable to remove test directory");
    }

    #[test]
    fn test_load_from_file() {
        let test_file = PathBuf::from("test_document.txt");
        let mut file = File::create(&test_file).expect("Unable to create file");
        file.write_all(b"Content of the test document")
            .expect("Unable to write to file");

        let mut loader = DocumentLoader::new(test_file.to_str().unwrap().to_string());
        loader.load();

        assert_eq!(loader.documents.len(), 1);

        let doc = loader.next().unwrap();
        assert_eq!(doc.document_name, "test_document.txt");
        assert_eq!(doc.document_content, "Content of the test document");

        fs::remove_file(test_file).expect("Unable to remove test file");
    }
}
