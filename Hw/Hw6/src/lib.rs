fn min_max_avg(v: &[i32]) -> Vec<f32>{
    let mut result: Vec<f32> = Vec::new();
    if v.is_empty(){
        return result;
    }
    let mut iterv = v.iter();
    let mut max = 0;
    let mut min = v[0];
    let mut sum = 0;
    let count = iterv.len() as f32;
    while let Some(n) = iterv.next(){
        if n >= &max{
            max = *n
        }
        if *n <= min{
            min = *n
        }
        sum += n;
    }
    let avg = sum as f32/count; 
    result.push(min as f32);
    result.push(max as f32);
    result.push(avg as f32);
    result
}

fn cal_partial_sums(v : &[i32]) -> Vec<i32>{
    let mut result : Vec<i32> = Vec::new();
    let mut iter = v.iter();
    if v.is_empty(){
        return result;
    }
    let mut sum = 0;
    while let Some(n) = iter.next(){
        sum += n;
        result.push(sum);
    }
    result
}

fn pack_number_tuples3(tup1 :&[i32], tup2 : &[i32], tup3: &[i32]) -> Vec<(i32, i32, i32)>{
    fn max_num(a: usize, b: usize, c: usize) -> usize {
        if a >= b && a >= c {
            a
        } else if b >= a && b >= c {
            b
        } else {
            c
        }
    }
    let mut result = Vec::new();
    let max = max_num(tup1.len(), tup2.len(), tup3.len());
    for count in 0..max{
        let num1 = if count < tup1.len() {tup1[count]} else {0};
        let num2 = if count < tup2.len() {tup2[count]} else {0};
        let num3 = if count < tup3.len() {tup3[count]} else {0};
        result.push((num1, num2, num3))
    }
    result
}

fn pack_number_tuples_s3(tup1 :&[i32], tup2 : &[i32], tup3: &[i32]) -> Vec<(i32, i32, i32)>{
    let mut result = Vec::new();
    let mut iter1 = tup1.iter();
    let mut iter2 = tup2.iter();
    let mut iter3 = tup3.iter();
    while let (Some(&x), Some(&y), Some(&z)) = (iter1.next(), iter2.next(), iter3.next()) {
        result.push((x, y, z));
    }
    result
}

fn unpack_number_tuples(v : &[(i32, i32)]) -> (Vec<i32>, Vec<i32>){
    let mut result1 = Vec::new();
    let mut result2 = Vec::new();
    let mut iter = v.iter();
    while let Some(&(x, y)) = iter.next(){
            result1.push(x);
            result2.push(y);
    }
    (result1, result2)
}

fn unpack_number_tuples3(v : &[(i32, i32, i32)]) -> (Vec<i32>, Vec<i32>, Vec<i32>){
    let mut result1 = Vec::new();
    let mut result2 = Vec::new();
    let mut result3 = Vec::new();
    let mut iter = v.iter();
    while let Some(&(x, y, z)) = iter.next(){
            result1.push(x);
            result2.push(y);
            result3.push(z);
    }
    (result1, result2, result3)
}




#[test]
fn test_min_max_avg() {
    assert_eq!(min_max_avg(&[2, 4, 6, 10, 8]), [2., 10., 6.,]);
    assert_eq!(min_max_avg(&[]), []);
    assert_eq!(min_max_avg(&[2]), [2., 2., 2.]);
}

#[test]
fn test_cal_partial_sums() {
    assert_eq!(cal_partial_sums(&[2, 4, 6, 10, 8]), [2, 6, 12, 22, 30]);
    assert_eq!(cal_partial_sums(&[]), []);
    assert_eq!(cal_partial_sums(&[2]), [2]);
}

#[test]
fn test_pack_number_tuples3() {
    assert_eq!(pack_number_tuples3(&[1, 2], &[4, 3], &[5]), [(1, 4, 5), (2, 3, 0)]);
    assert_eq!(pack_number_tuples3(&[], &[], &[]), []);
}

#[test]
fn test_pack_number_tuples_s3() {
    assert_eq!(pack_number_tuples_s3(&[1, 2], &[4, 3], &[5]),  [(1, 4, 5)]);
    assert_eq!(pack_number_tuples_s3(&[1, 2], &[4, 3], &[]),  []);
    assert_eq!(pack_number_tuples_s3(&[], &[], &[]), []);
}

#[test]
fn test_unpack_number_tuples(){
    assert_eq!(unpack_number_tuples(&[]), (vec![], vec![]));
    assert_eq!(unpack_number_tuples(&[(1, 2)]), (vec![1], vec![2]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (2, 5), (3, 6)]), (vec![1, 2, 3], vec![4, 5, 6]));
    assert_eq!(unpack_number_tuples(&[(1, 4), (3, 2), (2,1)]),  (vec![1, 3, 2], vec![4, 2, 1]));
    assert_eq!(unpack_number_tuples(&[(-1, -4), (-2, -5), (-3, -6)]), (vec![-1, -2, -3], vec![-4, -5, -6]));
}

#[test]
fn test_unpack_number_tuples3(){
    assert_eq!(unpack_number_tuples3(&[]), (vec![], vec![], vec![]));
    assert_eq!(unpack_number_tuples3(&[(1, 2, 3)]), (vec![1], vec![2], vec![3]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1)]), (vec![1, 2], vec![4, 2], vec![5, 1]));
    assert_eq!(unpack_number_tuples3(&[(-1, -4, -5), (-2, -2, -1)]), (vec![-1, -2], vec![-4, -2], vec![-5, -1]));
    assert_eq!(unpack_number_tuples3(&[(1, 4, 5), (2, 2, 1), (4, 5, 6)]), (vec![1, 2, 4], vec![4, 2, 5], vec![5, 1, 6]));
}

