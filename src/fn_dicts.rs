/*
Integer Functions: =============================
*/
pub(crate) fn sum_int(a: &i32, b: &i32) -> i32 {
    return a + b;
}
pub(crate) fn dif_int(a: &i32, b: &i32) -> i32 {
    return a - b;
}
pub(crate) fn dot_int(a: &i32, b: &i32) -> i32 {
    return a * b;
}
pub(crate) fn power_int(a: &i32, b: &i32) -> i32 {
    return a ^ b;
}
pub(crate) fn coalesce_int(a: &i32, b: &i32) -> i32 {
    if *a != 0 {
        return *a;
    }
    return *b;
}