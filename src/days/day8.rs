use super::util::read_file_lines;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Result;

pub fn solve_p1(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;
    Ok(lines
        .iter()
        .map(|line| line.split(" | ").collect::<Vec<_>>()[1])
        .flat_map(|digits| digits.split(" ").map(|d| d.len()))
        .filter(|d| *d >= 2 && *d <= 4 || *d == 7)
        .count() as u32)
}

pub fn solve_p2(filename: &str) -> Result<u32> {
    let lines = read_file_lines(filename)?;
    let table = HashMap::from([
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]);

    let eight = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);

    Ok(lines
        .iter()
        .map(|line| {
            let parts = line.split(" | ").collect::<Vec<_>>();
            let codes = to_char_set(parts[0]);
            let digits = parts[1].split(' ').collect::<Vec<_>>();
            (codes, digits)
        })
        .map(|(codes, digits)| {
            let keys = find_keys(codes, &eight);
            decode(digits, &keys, &table)
        })
        .sum())
}

fn to_char_set(digits: &str) -> Vec<HashSet<char>> {
    digits
        .split(' ')
        .map(|d| d.chars().collect::<HashSet<_>>())
        .collect()
}

fn decode(digits: Vec<&str>, keys: &HashMap<char, char>, table: &HashMap<&str, u32>) -> u32 {
    digits
        .iter()
        .map(|s| String::from_iter(s.chars().map(|c| keys[&c])))
        .map(sort_str)
        .map(|s| table[&*s])
        .zip((0..4).rev())
        .map(|(n, i)| n * 10_u32.pow(i))
        .sum()
}

fn sort_str(string: String) -> String {
    let mut vec_str = string.chars().collect::<Vec<_>>();
    vec_str.sort_by(|a, b| a.cmp(b));
    String::from_iter(&vec_str)
}

fn find_keys(codes: Vec<HashSet<char>>, eight: &HashSet<char>) -> HashMap<char, char> {
    let mut keys = HashMap::with_capacity(7);
    let one = codes.iter().filter(|d| d.len() == 2).next().unwrap();
    let four = codes.iter().filter(|d| d.len() == 4).next().unwrap();
    let seven = codes.iter().filter(|d| d.len() == 3).next().unwrap();
    keys.insert(seven.difference(&one).next().unwrap().clone(), 'a');
    let first_found = four.union(seven).map(|e| e.clone()).collect::<HashSet<_>>();
    let nine = codes
        .iter()
        .filter(|d| d.len() == 6 && first_found.is_subset(d))
        .next()
        .unwrap();
    let g_key = *nine.difference(&first_found).next().unwrap();
    keys.insert(g_key, 'g');
    let e_key = *eight.difference(&nine).next().unwrap();
    keys.insert(e_key, 'e');
    let two = codes
        .iter()
        .filter(|d| d.len() == 5 && found_keys(&keys).is_subset(*d))
        .next()
        .unwrap();
    let c_key = *two.intersection(&one).next().unwrap();
    keys.insert(c_key, 'c');
    let f_key = *one.difference(&HashSet::from([c_key])).next().unwrap();
    keys.insert(f_key, 'f');
    let d_key = *two.difference(&found_keys(&keys)).next().unwrap();
    keys.insert(d_key, 'd');
    let b_key = *eight.difference(&found_keys(&keys)).next().unwrap();
    keys.insert(b_key, 'b');

    keys
}

fn found_keys(keys: &HashMap<char, char>) -> HashSet<char> {
    HashSet::from_iter(keys.keys().map(|e| e.clone()))
}
