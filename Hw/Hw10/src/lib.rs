fn vflip(v: &[String]) -> Vec<String> {
    let mut result = v.to_vec();
    result.reverse();
    result
}

fn hflip(v: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    for s in v {
        let mut flipped_str = String::new();

        for j in s.chars().rev() {
            match j {
                '<' => flipped_str.push('>'),
                '>' => flipped_str.push('<'),
                _ => flipped_str.push(j),
            }
        }

        result.push(flipped_str);
    }

    result
}

fn vcat(v1: &[String], v2: &[String]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in v1 {
        result.push(i.to_string())
    }
    for i in v2 {
        result.push(i.to_string())
    }
    result
}

fn hcat(left: &[String], right: &[String]) -> Vec<String> {
    let max_height = left.len().max(right.len());
    let mut result = Vec::new();

    for i in 0..max_height {
        let mut row = String::new();

        if i < left.len() {
            row.push_str(&left[i]);
        } else {
            row.push_str(&" ".repeat(left[0].len()));
        }

        if i < right.len() {
            row.push_str(&right[i]);
        } else {
            row.push_str(&" ".repeat(right[0].len()));
        }

        result.push(row);
    }

    result
}

fn main() {}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);
    let data = ["<--", "#####", "<=="].map(|v| v.to_string());
    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);
    assert_eq!(
        vcat(&data, &data),
        ["<--", "#####", "<==", "<--", "#####", "<=="]
    );
    assert_eq!(hcat(&data, &data[..2]), ["<--<--", "##########", "<==   "]);
    assert_eq!(hcat(&data[..2], &data), ["<--<--", "##########", "   <=="]);
}

#[test]
fn test_img_flip() {
    let emp: Vec<String> = Vec::new();
    assert_eq!(vflip(&emp), emp.clone());
    assert_eq!(hflip(&emp), emp);

    let data: Vec<String> = vec!["<--".to_string(), "#####".to_string(), "<==".to_string()];

    assert_eq!(
        vflip(&data),
        vec!["<==".to_string(), "#####".to_string(), "<--".to_string(),]
    );

    assert_eq!(
        hflip(&data),
        vec!["-->".to_string(), "#####".to_string(), "==>".to_string(),]
    );
}
