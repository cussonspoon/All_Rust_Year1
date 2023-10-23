fn fahr_to_cel_v( v: &mut[i64]) -> Vec<i64>  {
    for n in 0..v.len(){
      (*v)[n] = ((5.0 / 9.0) * ((*v)[n] as f64 - 32.0)) as i64;
     }
     v.to_vec()
      }

fn fahr_to_cel_recursive( v: &mut[i64]) -> Vec<i64> {
     if !v.is_empty(){
          v[0] = ((5.0 / 9.0) * (v[0] as f64 - 32.0)) as i64;
          fahr_to_cel_recursive(&mut v[1..]);
      }
      v.to_vec()
     }

fn make_grades<'a>( v: &'a mut[&str]) -> Vec<&'a str> {
  for n in 0..v.len(){
    let score: i64 = match v[n].parse() {
      Ok(num) => num,
      Err(_) => -1,
  };
      if score < 0{
        (*v)[n] = "Invalid score"
      }
      else if score >= 0 && score <= 49 {
        (*v)[n] = "Failed with F"
      }
      else if score >= 50  && score <= 60 {
        (*v)[n] = "D"
      }
      else if score >= 61  && score <= 70 {
        (*v)[n] = "C"
      }
      else if score >= 71  && score <= 80 {
        (*v)[n] = "B"
      }
      else if score >= 81  && score <= 94 {
        (*v)[n] = "A"
      }
      else if score >= 95  && score <= 100 {
        (*v)[n] = "Excellent with A+"
      }
      else{
        (*v)[n] = "Invalid score"
      } 
  }
  v.to_vec()
}

fn make_grades_recursion<'a>( v: &'a mut[&str]) -> Vec<&'a str> {
  if !v.is_empty(){
    let score: i64 = match v[0].parse() {
      Ok(num) => num,
      Err(_) => -1,
  };
      if score < 0{
        (*v)[0] = "Invalid score"
      }
      else if score >= 0 && score <= 49 {
        (*v)[0] = "Failed with F"
      }
      else if score >= 50  && score <= 60 {
        (*v)[0] = "D"
      }
      else if score >= 61  && score <= 70 {
        (*v)[0] = "C"
      }
      else if score >= 71  && score <= 80 {
        (*v)[0] = "B"
      }
      else if score >= 81  && score <= 94 {
        (*v)[0] = "A"
      }
      else if score >= 95  && score <= 100 {
        (*v)[0] = "Excellent with A+"
      }
      else{
        (*v)[0] = "Invalid score"
      }
      make_grades_recursion(&mut v[1..]);
  }
  v.to_vec()
}






// fn triangle_loop(v: &mut [&str]) -> Vec<&'static str> {
//   let mut result: Vec<String> = Vec::new();

//   for n in 0..v.len() {
//       let input: i64 = match v[n].parse() {
//           Ok(num) => num,
//           Err(_) => -1,
//       };
//       let mut triangle_rows = String::new();

     
//       for i in 1..=input {
//           let row = "*".repeat(i as usize);
//           triangle_rows.push_str(&row);
//           triangle_rows.push('\n');
//       }

      
//       for i in (1..input).rev() {
//           let row = "*".repeat(i as usize);
//           triangle_rows.push_str(&row);
//           triangle_rows.push('\n');
//       }

//       result.push(triangle_rows);
//   }

//   let mut static_result: Vec<&'static str> = Vec::new();
//   for s in &result {
//       static_result.push(Box::leak(s.clone().into_boxed_str()));
//   }

//   static_result
// }

fn make_arrow1(n: usize) -> String {
  let mut result = String::new();
  
  for i in 1..=n {
      for _ in 0..i {
          result.push('*');
      }
      result.push('\n');
  }
  
  for i in (1..n).rev() {
      for _ in 0..i {
          result.push('*');
      }
      result.push('\n');
  }

  result
}

fn make_arrow2(n: usize) -> String {
  let mut result = String::new();
  
  for i in 1..=n {
      for _ in i..n {
        result.push(' ');
    }
      for j in 0..i {
        result.push('*');
    }
    result.push('\n');
  }
  
  for i in (1..n).rev() {
      for _ in i..n {
        result.push(' ');
    }
      for j in 0..i {
        result.push('*');
    }
    result.push('\n');
  }

  result
}

fn make_arrow1_recursion(n: usize) -> String {
  fn build_row(pattern: &mut String, num_asterisks: usize) {
    if num_asterisks > 0 {
      pattern.push('*');
      build_row(pattern, num_asterisks - 1);
  } else {
      pattern.push('\n');
  }
}

  fn build_arrow_up(pattern: &mut String, current_row: usize, max_rows: usize) {
      if current_row > max_rows {
          return;
      }
      build_row(pattern, current_row);
      build_arrow_up(pattern, current_row + 1, max_rows);
  }

  fn build_arrow_down(pattern: &mut String, current_row: usize, max_rows: usize) {
      if current_row <= 0 {
          return;
      }
      build_row(pattern, current_row);
      build_arrow_down(pattern, current_row - 1, max_rows);
  }

  let mut result = String::new();
  build_arrow_up(&mut result, 1, n);
  build_arrow_down(&mut result, n - 1, n - 1);

  result
}


fn make_arrow2_recursion(n: usize) -> String {
  fn build_spaces(pattern: &mut String, num_spaces: usize, max_spaces: usize) {
      if num_spaces <= max_spaces {
          pattern.push(' ');
          build_spaces(pattern, num_spaces + 1, max_spaces);
      }
  }

  fn build_row(pattern: &mut String, num_asterisks: usize) {
      if num_asterisks > 0 {
          pattern.push('*');
          build_row(pattern, num_asterisks - 1);
      } else {
          pattern.push('\n');
      }
  }

  fn build_arrow_up(pattern: &mut String, current_row: usize, max_rows: usize, n: usize) {
      if current_row <= max_rows {
          build_spaces(pattern, 1, n - current_row);
          build_row(pattern, current_row);
          build_arrow_up(pattern, current_row + 1, max_rows, n);
      }
  }

  fn build_arrow_down(pattern: &mut String, current_row: usize, max_rows: usize, n: usize) {
      if current_row > 0 {
          build_spaces(pattern, 1, n - current_row);
          build_row(pattern, current_row);
          build_arrow_down(pattern, current_row - 1, max_rows, n);
      }
  }

  let mut result = String::new();
 
  build_arrow_up(&mut result, 1, n, n);
  build_arrow_down(&mut result, n - 1, n - 1, n);
  result
}

 #[test]
fn test_fahr_to_cel_v() {
assert_eq!(fahr_to_cel_v(&mut[5, -4, -5, 0, 4, 8, 28, 1, 2, 10]), [-15, -20, -20, -17, -15, -13, -2, -17, -16, -12]);
assert_eq!(fahr_to_cel_v(&mut[10, 20, 30, 40]), [-12, -6, -1, 4]);
assert_eq!(fahr_to_cel_v(&mut[10]), [-12]);

 }

 #[test]
fn test_fahr_to_cel_recursive() {
assert_eq!(fahr_to_cel_recursive(&mut[5, -4, -5, 0, 4, 8, 28, 1, 2, 10]), [-15, -20, -20, -17, -15, -13, -2, -17, -16, -12]);
assert_eq!(fahr_to_cel_recursive(&mut[10, 20, 30, 40]), [-12, -6, -1, 4]);
assert_eq!(fahr_to_cel_v(&mut[10]), [-12]);
 }

 #[test]
fn test_make_grades() {
assert_eq!(make_grades(&mut["10", "20", "-5", "0", "50", "400", "60", "70", "90", "30"]), 
["Failed with F", "Failed with F", "Invalid score", "Failed with F", "D", "Invalid score", "D", "C", "A", "Failed with F"]);
assert_eq!(make_grades(&mut["71"]), 
["B"]);
 }

 #[test]
 fn test_make_grades_recursion() {
  assert_eq!(make_grades_recursion(&mut["10", "20", "-5", "0", "50", "400", "60", "70", "90", "30"]), 
  ["Failed with F", "Failed with F", "Invalid score", "Failed with F", "D", "Invalid score", "D", "C", "A", "Failed with F"]);
  assert_eq!(make_grades_recursion(&mut["71"]), 
  ["B"]);
}


#[test]
fn test_make_arrow1() {
  assert_eq!(make_arrow1(5), "*\n**\n***\n****\n*****\n****\n***\n**\n*\n");
  assert_eq!(make_arrow1(4), "*\n**\n***\n****\n***\n**\n*\n");
  assert_eq!(make_arrow1(3), "*\n**\n***\n**\n*\n");
}

#[test]
fn test_make_arrow2() {
  assert_eq!(make_arrow2(5), "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n");
  assert_eq!(make_arrow2(4), "   *\n  **\n ***\n****\n ***\n  **\n   *\n");
  assert_eq!(make_arrow2(3), "  *\n **\n***\n **\n  *\n");
}

#[test]
fn test_make_arrow1_recursion() {
    assert_eq!(make_arrow1_recursion(5), "*\n**\n***\n****\n*****\n****\n***\n**\n*\n");
    assert_eq!(make_arrow1_recursion(4), "*\n**\n***\n****\n***\n**\n*\n");
    assert_eq!(make_arrow1_recursion(3), "*\n**\n***\n**\n*\n");
}

#[test]
fn test_make_arrow2_recursion() {
  assert_eq!(make_arrow2_recursion(5), "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n");
  assert_eq!(make_arrow2_recursion(4), "   *\n  **\n ***\n****\n ***\n  **\n   *\n");
  assert_eq!(make_arrow2_recursion(3), "  *\n **\n***\n **\n  *\n");
}




// #[test]
// fn test_triangle_loop() {
//  assert_eq!(triangle_loop(&mut["1", "2", "3", "4", "5", "6"]), 
//  ["*\n", "*\n**\n*\n", "*\n**\n***\n**\n*\n", "*\n**\n***\n****\n***\n**\n*\n"
//  , "*\n**\n***\n****\n*****\n****\n***\n**\n*\n", "*\n**\n***\n****\n*****\n******\n*****\n****\n***\n**\n*\n"]);
// }

