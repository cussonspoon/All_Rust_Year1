fn count_digits(n: &str) -> usize {
    let mut digit = 0;
    for d in n.chars(){
      if d.is_digit(10){
        digit +=1
      }
    }
 digit
}



fn count_digits_recursion(n: &str) -> usize {
  let mut digit = 0;
   if n.is_empty() {
    return 0
   }
   let first_n = n.chars().next().unwrap();
   let remaining_n = &n[1..];
   if first_n.is_digit(10) {
    return 1 + count_digits_recursion(remaining_n);
   } 
   else {
    return count_digits_recursion(remaining_n);
   }
}


fn count_digits_v2(n: &str) -> Vec<(String, usize)> {
    let mut new = Vec::new();
    for half in n.split_whitespace() {
        let mut digit = 0;
        for c in half.chars() {
            if c.is_digit(10) {
                digit += 1;
        }

    }

    new.push((half.to_string(), digit));

    }

    new
}

fn extract_non_negatives( v: &[f32]) -> Vec<f32>{
    let mut new = Vec :: new();
    for &num in v{
      if num > 0.0{
        new.push(num)
      }
    }
    new
}

fn extract_non_negatives_recursion( v: &[f32]) -> Vec<f32>{
    let mut new = Vec :: new();
    if v.is_empty(){
        return new
    }
    else if v[0] > 0.0 {
        new.push(v[0])
    }
    new.extend_from_slice(&extract_non_negatives_recursion(&v[1..]));

    new
}

fn split_non_negatives( v: &[f32]) -> (Vec<f32>, Vec<f32>){
    let mut positive = Vec :: new();
    let mut negative = Vec :: new();
    for &num in v{
      if num > 0.0{
        positive.push(num);
      }
      else if num < 0.0 {
        negative.push(num);
    }
    }
    return (positive, negative)
}





#[test]
fn test_digits_count1() {
assert_eq!(count_digits(""), 0);
assert_eq!(count_digits("abcd"), 0);
assert_eq!(count_digits("ab12xy5 7x83y5z"), 7);
assert_eq!(count_digits("ab12xy5 7x83y5z j49w8jg8ewgew0gewjgw9jdgfifdjw0ej0gf9d0s9fhgjsidjhg09"), 20);
}

#[test]
fn test_digits_count1_recursion() {
assert_eq!(count_digits_recursion(""), 0);
assert_eq!(count_digits_recursion("abcd"), 0);
assert_eq!(count_digits_recursion("ab12xy5 7x83y5z"), 7);
}

#[test]
fn test_digits_count2() {
assert_eq!(count_digits_v2(""), []);
assert_eq!(count_digits_v2("ab12xy5 7x83y5z"),
[("ab12xy5".to_string(), 3),("7x83y5z".to_string(), 4)]);
}

#[test]
fn test_extract_non_negatives() {
assert_eq!(extract_non_negatives(&mut[]), []);
assert_eq!(
extract_non_negatives(&mut[0.8, -5.1, 1.6, -6.5, 10.5]),
[0.8, 1.6, 10.5]
);
}

#[test]
fn test_extract_non_negatives_recursion() {
assert_eq!(extract_non_negatives(&mut[]), []);
assert_eq!(
extract_non_negatives(&mut[0.8, -5.1, 1.6, -6.5, 10.5]),
[0.8, 1.6, 10.5]
);
}



#[test]
fn test_split_non_negatives() {
assert_eq!(split_non_negatives(&[]), (vec![], vec![]));
assert_eq!(
split_non_negatives(&[0.8, -5.1, 1.6, -6.5, 10.5]),
(
 vec![0.8, 1.6, 10.5],
 vec![-5.1, -6.5]
)
);
}
