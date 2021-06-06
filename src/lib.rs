/**
 * Count_Where adds a method to the Iterator trait that allows one to count the numbers of items matching a given predicate, much like a custom function in Swift that filters a sequence and return the count.
 * # Quick Start
 * 
 * The easiest way to get things working, after adding the crate as a dependency would be like this:
 * 
 * ```
 * use count_where::CountWhere;
 * 
 * fn main() {
 *  let numbers = [5, 5, 5, 2, 1];
 *  let number = 5;
 *  println!("{} appears {} times.", number, numbers.iter().count_where(|n| **n == number));
 * }
 * ```
 * 
 * The above goes through a specified array and looks for each instance that the number is 5 and displays that to the user.
 */

pub mod count_where_ext; // make extension trait visible to library

// give trait extension an easy to remember name
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
