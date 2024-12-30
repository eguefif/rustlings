// TODO: Fix the compiler error in this function.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec0 = fill_vec(vec0);
        assert_eq!(vec0, vec![22, 44, 66, 88]);
    }
}
