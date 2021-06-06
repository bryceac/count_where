/// trait extension that adds count_where functionality to iterators.
pub trait CountWhereExt: Iterator {
	fn count_where<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> usize;
}

// apply trait to all types implementing the Iterator trait.
impl<I> CountWhereExt for I where I: Iterator {
    
    /** 
     * count elements matching given predicate.
     * # Example
     * let numbers = [5, 5, 5, 2, 1];
     * assert_eq!(3, numbers.iter().count_where(|n| **n == 5)); */ 
	fn count_where<P: FnMut(&Self::Item) -> bool>(self, predicate: P)-> usize {
		
        // count up items found in filter.
		return self.filter(predicate).fold(0, |tally, _| tally + 1);
		
	}
}