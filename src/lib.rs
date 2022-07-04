// extern crate rayon;

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use pyo3::prelude::*;
// These traits let us use `par_lines` and `map`.
// use rayon::str::ParallelString;
// use rayon::iter::ParallelIterator;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn count_most_common(contents: &str) -> (u32, u32, f64)
{
    let mut word_counter = 0;
    let mut char_counter = 0;
    let mut word_map: HashMap<&str, i64> = HashMap::new();
    
    for token in contents.split_whitespace(){
        // let proper_token = token.to_lowercase();
        if !word_map.contains_key(token) 
        {
            let first = 1;
            word_map.insert( token, first);
        }
        else
        {
            *word_map.get_mut(token).unwrap() += 1;
        }
        word_counter+=1;
        char_counter += token.chars().count();
    }

    let avg_word_len: f64 = (char_counter as f64)/(word_counter as f64);
    return (word_counter as u32, char_counter as u32, avg_word_len);
}

#[pyfunction]
fn get_word_hashmap(contents: &str) -> HashMap<String, i64>
{
    let mut word_map: HashMap<String, i64> = HashMap::new();
    
    for token in contents.split_whitespace(){
        // let proper_token = token.to_lowercase();
        if !word_map.contains_key(token) 
        {
            let first = 1;
            word_map.insert( (*token).to_string(), first);
        }
        else
        {
            *word_map.get_mut(token).unwrap() += 1;
        }
    }
    return word_map;
}



#[pyfunction]
fn count_shortest_word(contents: &str) -> (u32, String)
{
    let mut word_counter = 0;
    let mut char_counter = 0;
    let mut min_length= 100;
    let mut min_length_name = "";
    for token in contents.split_whitespace(){
        if word_counter ==1
        {
            min_length = char_counter;
            min_length_name = token;
        }
        if min_length> token.chars().count(){
            min_length_name = token;
            min_length = token.chars().count();            
        }
        word_counter+=1;
        char_counter += token.chars().count();
    }
    return (min_length as u32, min_length_name.to_string() as String);

}


#[pyfunction]
fn parallel_text_analysis(contents: String) ->  (HashMap<String, i64>, f64, String)
{
//     let (tx_word_counter, rx_word_counter) = mpsc::channel();
//     let (tx_char_counter, rx_char_counter) = mpsc::channel();
    let (tx_avg_word_len, rx_avg_word_len) = mpsc::channel();
    // let (tx_shortest_word_char_num, rx_shortest_word_char_num) = mpsc::channel();
    let (tx_shortest_word, rx_shortes_word) = mpsc::channel();
    let (tx_word_hashmap, rx_word_hashmap) = mpsc::channel();
    // let mut shortest_word: String = String::from("");
    // let mut word_counter = 0;
    // let mut char_counter = 0;
    // let mut avg_word_len: f64 = 0.0;
    // let mut shortest_word_char_num = 0;
    let contents2 = contents.clone();
    let contents3 = contents.clone();

    let thread_handle_1 = thread::spawn(move ||{
        let (_, _, avg_word_len) = count_most_common(&contents);
        tx_avg_word_len.send(avg_word_len).unwrap();
    }
    );

    let thread_handle_2 = thread::spawn(move ||{
        let (_, shortest_word) = count_shortest_word(&contents2);
        // tx_shortest_word_char_num.send(shortest_word_char_num).unwrap();
        tx_shortest_word.send(shortest_word).unwrap();
    }
    );

    let thread_handle_3 = thread::spawn(move ||{
        let word_hashmap = get_word_hashmap(&contents3);
        tx_word_hashmap.send(word_hashmap).unwrap();
    }
    );

    thread_handle_1.join().unwrap();
    thread_handle_2.join().unwrap();
    thread_handle_3.join().unwrap();

    let avg_word_len = rx_avg_word_len.recv().unwrap();
    // let shortest_word_char_num = rx_shortest_word_char_num.recv().unwrap();
    let shortest_word = rx_shortes_word.recv().unwrap();
    let word_hashmap = rx_word_hashmap.recv().unwrap();
    return (word_hashmap, avg_word_len, shortest_word);
}


/// INTRESTING

/// Count the occurrences of needle in line, case insensitive
// fn count_line(line: &str, needle: &str) -> usize {
//     let mut total = 0;
//     for word in line.split(' ') {
//         if word == needle {
//             total += 1;
//         }
//     }
//     total
// }

// #[pyfunction]
// fn search_word(contents: &str, needle: &str) -> usize {
//     contents
//         .par_lines()
//         .map(|line| count_line(line, needle))
//         .sum()
// }

/// A Python module implemented in Rust.
#[pymodule]
fn text_analyzer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(count_shortest_word, m)?)?;
    m.add_function(wrap_pyfunction!(get_word_hashmap, m)?)?;
    m.add_function(wrap_pyfunction!(count_most_common, m)?)?;
    m.add_function(wrap_pyfunction!(parallel_text_analysis, m)?)?;
    Ok(())
}