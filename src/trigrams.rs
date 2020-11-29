use std::collections::HashMap;
const DEL: &str = "$";

pub struct Trigrams {
    index: Index,
    documents: Documents,
}

impl Trigrams {
    pub fn new() -> Self {
        Self {
            index: Index::new(),
            documents: Documents::new(),
        }
    }

    pub fn add_doc(&mut self, value: String) {
        let trigrams = Trigram::extract(&value);

        let doc_id = self.documents.add(value);

        for trigram in trigrams.into_iter() {
            self.index.add(trigram, doc_id);
        }
    }

    pub fn find_doc<'a>(&'a mut self, query: String) -> Option<&'a str> {
        let trigrams = Trigram::extract(&query);
        let mut document_frequencies: HashMap<DocId, u32> = HashMap::new();

        for trigram in trigrams.iter() {
            // if let Some(doc_ids) = self.index.get(trigram) {
            for doc_id in self.index.get(trigram).iter() {
                *document_frequencies.entry(*doc_id).or_insert(0) += 1;
            }
            // }
        }

        document_frequencies
            .into_iter()
            .max_by(|(_doc1_id, freq1), (_doc2_id, freq2)| freq1.cmp(freq2))
            .map(move |(doc_id, _freq)| self.documents.get(doc_id).unwrap())
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct DocId(u32);

impl DocId {
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Trigram(String);

impl Trigram {
    pub fn extract(value: &str) -> Vec<Self> {
        let mut v = String::from(DEL);
        v.push_str(&value.to_lowercase());
        v.push_str(DEL);

        let char_vec = v.chars().collect::<Vec<_>>();

        if char_vec.len() < 3 {
            vec![]
        } else if char_vec.len() == 3 {
            vec![Trigram(v)]
        } else {
            (0..char_vec.len() - 2)
                .map(|i| Trigram(char_vec[i..i + 3].iter().collect()))
                .collect()
        }
    }
}

struct Documents {
    table: HashMap<DocId, String>,
    next_id: DocId,
}

impl Documents {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            next_id: DocId(1),
        }
    }

    pub fn add(&mut self, value: String) -> DocId {
        let doc_id = self.next_id;
        self.table.insert(doc_id, value);
        self.next_id.inc();
        doc_id
    }

    pub fn get<'a>(&'a self, doc_id: DocId) -> Option<&'a str> {
        self.table.get(&doc_id).map(|s| s.as_str())
    }
}

struct Index(HashMap<Trigram, Vec<DocId>>);

impl Index {
    const EMPTY_VEC: &'static Vec<DocId> = &vec![];
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn add(&mut self, trigram: Trigram, doc_id: DocId) {
        self.0.entry(trigram).or_insert(vec![]).push(doc_id)
    }

    fn get(&self, trigram: &Trigram) -> &Vec<DocId> {
        self.0.get(trigram).unwrap_or(Self::EMPTY_VEC)
    }
}

#[cfg(test)]
mod test {
    use crate::trigrams::{DocId, Documents, Index, Trigram, Trigrams};

    #[test]
    fn extract_trigrams_test() {
        let s = "Hello";
        let expected: Vec<Trigram> = vec!["$he", "hel", "ell", "llo", "lo$"]
            .iter()
            .map(|s| Trigram(s.to_string()))
            .collect();

        assert_eq!(Trigram::extract(s), expected);
    }

    #[test]
    fn extract_trigrams_from_empty_string_test() {
        assert_eq!(Trigram::extract(""), vec![]);
    }

    #[test]
    fn extract_trigrams_from_one_symbol_string_test() {
        assert_eq!(Trigram::extract("@"), vec![Trigram("$@$".to_string())]);
    }

    #[test]
    fn extract_trigrams_from_multibyte_string_test() {
        let s = "Привет";
        let expected: Vec<Trigram> = vec!["$пр", "при", "рив", "иве", "вет", "ет$"]
            .iter()
            .map(|s| Trigram(s.to_string()))
            .collect();

        assert_eq!(Trigram::extract(s), expected);
    }

    #[test]
    fn documents_test() {
        let mut documents = Documents::new();

        let doc1_id = documents.add("one".to_string());
        assert_eq!(doc1_id, DocId(1));

        let doc2_id = documents.add("two".to_string());
        assert_eq!(doc2_id, DocId(2));

        assert_eq!(documents.get(doc1_id), Some("one"));
        assert_eq!(documents.get(doc2_id), Some("two"));
        assert_eq!(documents.get(DocId(3)), None);
    }

    #[test]
    fn index_test() {
        let mut index = Index::new();

        let trigram_one = Trigram("one".to_string());
        let trigram_two = Trigram("two".to_string());
        let trigram_three = Trigram("three".to_string());
        let trigram_common = Trigram("common".to_string());

        index.add(trigram_one.clone(), DocId(1));
        index.add(trigram_two.clone(), DocId(2));
        index.add(trigram_common.clone(), DocId(1));
        index.add(trigram_common.clone(), DocId(2));

        assert_eq!(*index.get(&trigram_common), vec![DocId(1), DocId(2)]);
        assert_eq!(*index.get(&trigram_one), vec![DocId(1)]);
        assert_eq!(*index.get(&trigram_two), vec![DocId(2)]);
        assert_eq!(*index.get(&trigram_three), vec![]);
    }

    #[test]
    fn trigrams_test() {
        let mut trigrams = Trigrams::new();

        trigrams.add_doc("cat eats fish".to_string());
        trigrams.add_doc("dog eats dish".to_string());

        assert_eq!(
            trigrams.find_doc("cat beats fish".to_string()),
            Some("cat eats fish")
        );
        assert_eq!(
            trigrams.find_doc("dog beats dish".to_string()),
            Some("dog eats dish")
        );
        assert_eq!(trigrams.find_doc("hello".to_string()), None);
    }
}
