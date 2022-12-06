use std::collections::HashSet;

const PACKET_MARKER_SIZE: usize = 4;
const MESSAGE_MARKER_SIZE: usize = 14;

// Finds the marker index ('initial marker index' + 'marker size')
fn find_marker_index(datastream: &str, marker_size: usize) -> i32 {
    if datastream.is_empty() { // Datastream is empty, index -1 by default
        -1
    }
    else {
        let datastream = datastream.chars().collect::<Vec<char>>(); // Vec of chars from datastream
        let (marker_idx, _) = datastream
            .windows(marker_size) // Creating an iterator over all contiguous windows of length 'marker_size' (the windows overlap!)
            .enumerate() // Using enumerate() to register the starting index of each window
            
            /* Filtering by the first window whose HashSet is the same size of 'marker_size'.
             * Therefore, all characters from that window are different and they constitute a valid marker */
            .filter(|(_, w)| HashSet::<&char>::from_iter(w.iter()).len() == marker_size)
            .next()
            .unwrap();
        (marker_size + marker_idx) as i32
    }
}

fn main() {
    println!(" --- Day 6: Tuning Trouble --- \n");
    println!("\t --- Part One --- \n");

    let datastream = include_str!("../../input/day6.txt");

    let packet_marker = find_marker_index(datastream, PACKET_MARKER_SIZE);
    println!("{packet_marker} characters need to be processed before the first start-of-packet marker is detected.\n");

    println!("\t --- Part Two --- \n");

    let message_marker = find_marker_index(datastream, MESSAGE_MARKER_SIZE);
    println!("{message_marker} characters need to be processed before the first start-of-message marker is detected.");
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_marker_indexes() {
        let input = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
        ];

        for (datastream, packet_idx, message_idx) in input {
            assert_eq!(find_marker_index(datastream, PACKET_MARKER_SIZE), packet_idx);
            assert_eq!(find_marker_index(datastream, MESSAGE_MARKER_SIZE), message_idx);
        }
    }

}