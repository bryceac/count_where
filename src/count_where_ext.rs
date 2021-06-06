/// trait extension that adds count_where functionality
pub trait CountWhereExt: Iterator {
	fn count_where<P: FnMut(&Self::Item) -> bool>(self, predicate: P) -> usize;
}

// apply trait to all types implementing the Iterator trait.
impl<I> CountWhereExt for I where I: Iterator {
    
    /// count elements matching given predicate. the parameter in the closure is of &&T, so it must be dereferenced to do comparisons.
	fn count_where<P: FnMut(&Self::Item) -> bool>(self, predicate: P)-> usize {
		
        // count up items found in filter.
		return self.filter(predicate).fold(0, |tally, _| tally + 1);
		
	}
}