
use extendr_api::prelude::*;
/// @export
#[extendr]
fn map_int(x: &[i32], y: &[i32]) {
  let mut vec = Vec::new();
  // TODO: use dictionary or switch
  // use sum for now
  for (a, b) in x.iter().zip(y.iter()) {
    vec.push(a + b);
  }
  return vec
}

// fn map_int(x: i32, y: &[i32], func: fn(&i32, &i32) -> i32) -> Vec<i32> {

//     let mut vec = Vec::new();
  
//     for (a, b) in x.iter().zip(y.iter()) {
//       vec.push(func(a, b));
//     }
  
//     vec
//   }


extendr_module! {
    mod map;
    fn map_int;
}
