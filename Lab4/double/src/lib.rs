
        // fn doubles( v: &mut[i64]) -> Vec<i64>  {
        //         for n in 0..v.len(){
        //             (*v)[n]*=2
        //         }
        //         v.to_vec()
        //         }
fn doubles( v: &mut[i64]) -> Vec<i64> {
 if !v.is_empty(){
     v[0] *=2;
     doubles(&mut v[1..]);
 }
 v.to_vec()
 
}

    
#[test]
    fn test_doubles() {
    assert_eq!(doubles(&mut[5, -4, -5, 0, 4, 8, 28, 1, 2, 10]), [10, -8, -10, 0, 8, 16, 56, 2, 4, 20]);
    }
    