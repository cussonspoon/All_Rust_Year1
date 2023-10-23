fn count_vowels(n: &str) -> usize {
    let mut count = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    for i in n.chars() {
        if vowels.contains(&i) {
            count += 1
        }
    }
    count
}

fn count_vowels_r(n: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    if n.is_empty() {
        return 0;
    }
    let first_n = n.chars().next().unwrap();
    let remaining_n = &n[1..];
    if vowels.contains(&first_n) {
        return 1 + count_vowels_r(remaining_n);
    } else {
        count_vowels_r(remaining_n)
    }
}
fn count_vowels_v2(n: &str) -> Vec<(String, usize)> {
    let mut result = Vec::new();
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    for word in n.split_whitespace() {
        let mut count = 0;
        for i in word.chars() {
            if vowels.contains(&i) {
                count += 1
            }
        }
        result.push((word.to_string(), count));
    }
    result
}

fn split_grade(grades: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let mut pass_grades = Vec::new();
    let mut fail_grades = Vec::new();

    for grade in grades {
        match grade {
            "A+" | "A" | "A-" | "B+" | "B" | "B-" | "C+" | "C" | "C-" => pass_grades.push(grade),
            _ => fail_grades.push(grade),
        }
    }

    (pass_grades, fail_grades)
}

fn split_scores(n: Vec<usize>) -> Vec<(&'static str, usize)> {
    let mut result = Vec::new();

    for num in n {
        let grade: &str;
        if num <= 49 {
            grade = "F";
        } else if num <= 60 {
            grade = "D";
        } else if num <= 70 {
            grade = "C";
        } else if num <= 80 {
            grade = "B";
        } else if num <= 94 {
            grade = "A";
        } else if num <= 100 {
            grade = "A+";
        } else {
            grade = "Invalid score";
        }

        result.push((grade, num));
    }

    result
}

fn extract_quoted_words(input: &str) -> Vec<&str> {
    let mut result = Vec::new();

    for word in input.split_whitespace() {
        match word {
            "" => {}
            word if word.starts_with('*') && word.ends_with('*') => {
                result.push(&word[1..word.len() - 1]);
            }
            _ => {}
        }
    }

    result
}

fn extract_quoted_words_r(input: &str) -> Vec<String> {
    fn recursive<'a>(
        mut words: std::str::SplitWhitespace<'a>,
        mut result: Vec<String>,
    ) -> Vec<String> {
        if let Some(word) = words.next() {
            if word.starts_with('*') && word.ends_with('*') {
                result.push(word[1..word.len() - 1].to_string());
            }
            recursive(words, result)
        } else {
            result
        }
    }

    let words = input.split_whitespace();
    recursive(words, Vec::new())
}

#[test]
fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
}

#[test]
fn test_vowels_count1_r() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
}

#[test]
fn test_vowels_count2() {
    assert_eq!(count_vowels_v2(""), []);
    assert_eq!(
        count_vowels_v2("ab12Exey5 7x8U3y5z"),
        [
            ("ab12Exey5".to_string(), 3), // 'a', 'E', 'e'
            ("7x8U3y5z".to_string(), 1)   // 'U'
        ]
    );
}

#[test]
fn test_split_grade() {
    assert_eq!(split_grade(vec![]), (vec![], vec![]));
    assert_eq!(
        split_grade(vec!["B", "F", "A+", "D", "C"]),
        (vec!["B", "A+", "C"], vec!["F", "D"])
    );
}

#[test]
fn test_split_scores() {
    assert_eq!(split_scores(vec![]), vec![]);
    assert_eq!(
        split_scores(vec![75, 42, 98, 54, 63]),
        vec![("B", 75), ("F", 42), ("A+", 98), ("D", 54), ("C", 63),]
    );
}

#[test]
fn test_extract_quoted_words() {
    assert_eq!(extract_quoted_words(""), Vec::<&str>::new());
    assert_eq!(
        extract_quoted_words("C ** *C++* *Java *Python* Rust*"),
        vec!["", "C++", "Python"]
    );
}

#[test]
fn test_extract_quoted_words_r() {
    assert_eq!(extract_quoted_words_r(""), Vec::<&str>::new());
    assert_eq!(
        extract_quoted_words_r("C ** *C++* *Java *Python* Rust*"),
        vec!["", "C++", "Python"]
    );
}
