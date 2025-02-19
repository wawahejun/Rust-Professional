use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut distinct_elements = HashSet::new();

    // Split the input string by commas and iterate over the parts
    for element in input_str.split(',') {
        distinct_elements.insert(element); // Insert each element into the HashSet
    }

    distinct_elements.len() // Return the number of elements in the HashSet (which are unique)
}