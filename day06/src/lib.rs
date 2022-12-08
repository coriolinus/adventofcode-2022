use aoclib::parse;
use std::path::Path;

const PACKET_SIZE: usize = 4;
const MESSAGE_SIZE: usize = 14;

fn find_start(data: &[u8], size: usize) -> Result<usize, Error> {
    // we could have used a more complicated counting array to keep track of the
    // state, but mapping to bitflags is working quite well enough.
    let window_idx = data
        .windows(size)
        .position(|window| {
            window
                .iter()
                .fold(0_u128, |acc, byte| acc | 1 << byte)
                .count_ones() as usize
                == size
        })
        .ok_or(Error::NoSolution)?;
    Ok(window_idx + size)
}

pub fn part1(input: &Path) -> Result<(), Error> {
    for (idx, input) in parse::<String>(input)?.enumerate() {
        let packet_start = find_start(input.as_bytes(), PACKET_SIZE)?;
        println!("pt. 1 idx {idx} packet start: {packet_start}");
    }
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    for (idx, input) in parse::<String>(input)?.enumerate() {
        let message_start = find_start(input.as_bytes(), MESSAGE_SIZE)?;
        println!("pt. 1 idx {idx} packet start: {message_start}");
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn start_of_packet_examples(#[case] input: &str, #[case] expect: usize) {
        let have = find_start(input.as_bytes(), PACKET_SIZE).expect("found a message");
        assert_eq!(have, expect);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn start_of_message_examples(#[case] input: &str, #[case] expect: usize) {
        let have = find_start(input.as_bytes(), MESSAGE_SIZE).expect("found a message");
        assert_eq!(have, expect);
    }
}
