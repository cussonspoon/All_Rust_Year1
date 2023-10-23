use std::fs;
use std::path::Path;

fn make_document(text: &str) -> String {
    let para: Vec<&str> = text.split("\n\n").collect();
    let mut result = String::new();

    for i in para {
        result.push_str("\t");
        result.push_str(i);
        result.push_str("\n\n")
    }
    result
}

fn rank_documents<'a>(docs: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut temp_result: Vec<(String, usize)> = Vec::new();

    for doc in &docs {
        let count_paragraphs: Vec<&str> = doc.split("\n\n").collect();
        let count = count_paragraphs.len();

        temp_result.push((doc.clone(), count));
    }

    temp_result.sort_by(|a, b| b.1.cmp(&a.1));

    for (paragraph, _) in temp_result {
        result.push(paragraph);
    }

    result
}


fn main(){}



#[test]
fn test_both_documents() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
    The bustle in a house\n\
    The morning after death\n\
    Is solemnest of industries\n\
    Enacted upon earth,—\n\
    \n\
    The sweeping up the heart,\n\
    And putting love away\n\
    We shall not want to use again\n\
    Until eternity.\n\
    ";
    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs
    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(docs);
    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}

