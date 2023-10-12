use std::fs;
use std::path::Path;

fn process_documents(file_paths: &[&str]) -> Vec<(String, String)> {
    let mut documents: Vec<(String, String)> = Vec::new();

    for file_path in file_paths {
        if let Ok(contents) = fs::read_to_string(file_path) {
            let document_name = Path::new(file_path)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(file_path)
                .to_string();
            let document_text = contents;
            documents.push((document_name, document_text));
        } else {
            eprintln!("Failed to read file: {}", file_path);
        }
    }

    documents
}

fn make_html(documents: &[(String, String)]) -> String {
    let mut html = String::new();
    html.push_str("<html><head>");
    html.push_str("<style>table {border-collapse: collapse; width: 50%; margin: 20px auto;}th, td {border: 1px solid black; padding: 8px; text-align: left;}th {background-color: #f2f2f2;}</style>");
    html.push_str("</head><body>");

    let mut html_table = String::from("<table><tr><th>Document</th><th>Paragraphs</th></tr>");

    for (document_name, paragraph_count) in documents {
        html_table.push_str(&format!(
            "<tr><td>{}</td><td>{}</td></tr>",
            document_name, paragraph_count
        ));
    }

    html_table.push_str("</table>");

    html.push_str(&html_table);
    html.push_str("</body></html>");

    html
}

fn rank_documents(docs: Vec<(String, String)>) -> Vec<(String, String)> {
    let mut temp_result: Vec<(String, String)> = Vec::new();
    for (doc_name, doc_text) in &docs {
        let doc_text_normalized = doc_text.replace("\r\n", "\n").replace("\r", "\n");
        let count_paragraphs: Vec<&str> = doc_text_normalized.split("\n\n").collect();
        let count = count_paragraphs.len();

        temp_result.push((doc_name.clone(), count.to_string()));
    }
    temp_result.sort_by(|a, b| {
        let count_a = a.1.parse::<i32>().unwrap_or(0);
        let count_b = b.1.parse::<i32>().unwrap_or(0);
        count_b.cmp(&count_a)
    });
    temp_result
}

fn main() {
    let file_paths: Vec<&str> = vec!["fox.txt", "para3.txt", "bustle.txt"];
    let info = process_documents(&file_paths);
    let ranked_info = rank_documents(info);
    let table = make_html(&ranked_info);
    println!("{}", table);
}
