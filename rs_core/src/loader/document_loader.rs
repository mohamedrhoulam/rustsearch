use lopdf::Document as LopdfDocument;
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

    pub fn load(&mut self) {
        if Path::new(&self.source).is_dir() {
            for entry in fs::read_dir(&self.source).expect("Unable to read directory") {
                let entry = entry.expect("Unable to read entry");
                let path = entry.path();
                if path.is_file() {
                    self.file(&path);
                }
            }
        } else if Path::new(&self.source).is_file() {
            let path = Path::new(&self.source).to_path_buf();
            self.file(&path);
        }
    }

    pub fn next(&mut self) -> Option<Document> {
        self.documents.pop_front()
    }

    fn file(&mut self, path: &Path) {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            match extension.to_lowercase().as_str() {
                "txt" => self.from_txt(path),
                "pdf" => self.from_pdf(path),
                _ => eprintln!("Unsupported file type: {}", extension),
            }
        }
    }

    fn from_txt(&mut self, path: &Path) {
        let content = fs::read_to_string(&path).expect("Unable to read file");
        self.document(path, content);
    }

    fn from_pdf(&mut self, path: &Path) {
        let content = match LopdfDocument::load(path) {
            Ok(doc) => {
                let mut extracted_content = String::new();
                for page_id in doc.get_pages().values() {
                    if let Ok(page_content) = doc.extract_text(&[page_id.0]) {
                        extracted_content.push_str(&page_content);
                        extracted_content.push('\n');
                    }
                }
                extracted_content
            }
            Err(_) => String::from("[Failed to extract content from PDF]"),
        };
        self.document(path, content);
    }

    fn document(&mut self, path: &Path, content: String) {
        let document_id = Uuid::new_v4().to_string();
        let document_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.to_string())
            .expect("Unable to get file name");

        self.documents.push_back(Document {
            document_id,
            document_name,
            document_content: content,
        });
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
