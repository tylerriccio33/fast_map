
pub mod map;
pub mod fn_dicts;

// This is an example use of map
fn main() {
    let xs  = &[1, 2, 3];
    let ys = &[1, 2, 3];

    
    let result = map::map_int(xs, ys, fn_dicts::coalesce_int);
    
    for x in result {
        println!("{}", x);
      }
    
}
