use std::collections::HashMap;

fn main() {
    /* Given a list of integers, use a vector and return the median 
     * (when sorted, the value in the middle position) and mode 
     * (the value that occurs most often; a hash map will be helpful here) 
     * of the list.
     */
    let ints = [2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4];

    let mut v: Vec<i32> = Vec::from(ints);

    /* Median */
    v.sort_unstable();
    println!("Median: {}", v[v.len()/2]);

    /* Hash map */
    let mut hash = HashMap::new();

    for int in &ints{
        let count = hash.entry(int).or_insert(0);
        *count += 1;
    }

    /* Mode */
    let mode = hash.iter().max_by_key(|&(_, count)| count).map(|(val, _)| val);
    println!("Mode: {}", mode.unwrap());
}
