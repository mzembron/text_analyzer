// extern crate rayon;

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use pyo3::prelude::*;
use std::time::Instant;
// These traits let us use `par_lines` and `map`.
// use rayon::str::ParallelString;
// use rayon::iter::ParallelIterator;

#[pyfunction]
fn get_word_hashmap(contents: &str) -> HashMap<String, i64>
{
    let mut word_map: HashMap<String, i64> = HashMap::new();
    
    for word in contents.split_whitespace()
    {
        // let proper_token = word.to_lowercase();
        if !word_map.contains_key(word) 
        {
            let first = 1;
            word_map.insert((*word).to_string(), first);
        }
        else
        {
            *word_map.get_mut(word).unwrap() += 1;
        }
    }
    return word_map;
}

#[pyfunction]
fn calculate_words_parameters(contents: &str) -> (String, String, f64)
{
    let mut word_counter = 0;
    let mut char_counter = 0;
    let mut min_length = 0;
    let mut shortes_word = "";
    let mut max_length = 0;
    let mut longest_word = "";

    for word in contents.split_whitespace()
    {
        let word_length = word.chars().count();
        if word_counter == 1
        {
            min_length = char_counter;
            shortes_word = word;
        }
        else if word_length < min_length
        {
            shortes_word = word;
            min_length = word_length;            
        }
        else if word_length > max_length
        {
            longest_word = word;
            max_length = word_length; 
        }
        word_counter+=1;
        char_counter += word_length;
    }

    return  (shortes_word.to_string() as String, longest_word.to_string(),
                (char_counter as f64) / (word_counter as f64));
}

#[pyfunction]
fn sequential_text_analysis(contents: String) ->  (HashMap<String, i64>, f64, String, String)
{
    let start = Instant::now();

    let mut word_counter = 0;
    let mut char_counter = 0;
    let mut min_length = 0;
    let mut shortes_word = "";
    let mut max_length = 0;
    let mut longest_word = "";

    let mut word_map: HashMap<String, i64> = HashMap::new();
    
    for word in contents.split_whitespace()
    {
        // let proper_token = word.to_lowercase();
        if !word_map.contains_key(word) 
        {
            let first = 1;
            word_map.insert((*word).to_string(), first);
        }
        else
        {
            *word_map.get_mut(word).unwrap() += 1;
        }

        let word_length = word.chars().count();
        if word_counter == 1
        {
            min_length = char_counter;
            shortes_word = word;
        }
        else if word_length < min_length
        {
            shortes_word = word;
            min_length = word_length;            
        }
        else if word_length > max_length
        {
            longest_word = word;
            max_length = word_length; 
        }
        word_counter+=1;
        char_counter += word_length;
    }

    let duration = start.elapsed();
    println!("Time elapsed in sequentional computation: {:?}", duration);
    return  (word_map, (char_counter as f64) / (word_counter as f64), shortes_word.to_string() as String,
            longest_word.to_string());
}

#[pyfunction]
fn parallel_text_analysis(contents: String) ->  (HashMap<String, i64>, f64, String, String)
{
    let start = Instant::now();
    
    let (tx_avg_word_len, rx_avg_word_len) = mpsc::channel();
    let (tx_shortest_word, rx_shortes_word) = mpsc::channel();
    let (tx_longest_word, rx_longest_word) = mpsc::channel();
    let (tx_word_hashmap, rx_word_hashmap) = mpsc::channel();
    let contents_copy = contents.clone();

    let thread_handle_1 = thread::spawn(move ||{
        let (shortest_word, longest_word, avg_word_len) = calculate_words_parameters(&contents);
        tx_shortest_word.send(shortest_word).unwrap();
        tx_longest_word.send(longest_word).unwrap();
        tx_avg_word_len.send(avg_word_len).unwrap();
    }
    );

    let thread_handle_2 = thread::spawn(move ||{
        let word_hashmap = get_word_hashmap(&contents_copy);
        tx_word_hashmap.send(word_hashmap).unwrap();
    }
    );

    thread_handle_1.join().unwrap();
    thread_handle_2.join().unwrap();

    let avg_word_len = rx_avg_word_len.recv().unwrap();
    let shortest_word = rx_shortes_word.recv().unwrap();
    let longest_word = rx_longest_word.recv().unwrap();
    let word_hashmap = rx_word_hashmap.recv().unwrap();

    let duration = start.elapsed();
    println!("Time elapsed in parallel computation: {:?}", duration);

    return (word_hashmap, avg_word_len, shortest_word, longest_word);
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
    m.add_function(wrap_pyfunction!(calculate_words_parameters, m)?)?;
    m.add_function(wrap_pyfunction!(get_word_hashmap, m)?)?;
    m.add_function(wrap_pyfunction!(parallel_text_analysis, m)?)?;
    m.add_function(wrap_pyfunction!(sequential_text_analysis, m)?)?;
    Ok(())
}