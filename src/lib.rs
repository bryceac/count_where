pub mod count_where_ext;
pub use crate::count_where_ext::CountWhereExt as CountWhere;

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn has_three_fives() {
        let numbers = [5, 5, 5, 2, 1];

        assert_eq!(3, numbers.iter().count_where(|n| **n == 5))
    }
}
