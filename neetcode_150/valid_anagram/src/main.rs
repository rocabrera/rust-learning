use std::collections::HashMap;

fn is_valid_anagram(s: String, t: String) -> bool {

    if s.len() != t.len() {
        return false;
    }

    let mut s_map: HashMap<char, i32> = HashMap::new();
    let mut t_map: HashMap<char, i32> = HashMap::new();

    for (s, t) in s.chars().zip(t.chars()) {
        
        if s_map.contains_key(&s) {
            let count = s_map.get(&s).unwrap();
            s_map.insert(s, count + 1);
        } else {
            s_map.insert(s, 1);
        }

        if t_map.contains_key(&t) {
            let count = t_map.get(&t).unwrap();
            t_map.insert(t, count + 1);
        } else {
            t_map.insert(t, 1);
        }
    }

    for (key, count) in s_map.iter() {

        if t_map.contains_key(key) {
            let t_count = t_map.get(key).unwrap();
            if count != t_count {
                return false;
            }
        } else {
            return false;
        }
    }

    true
    
}

fn main() {

    let s = String::from("racecar");
    let t = String::from("carrace");

    let is_valid = is_valid_anagram(s, t);
    println!("Is valid anagram: {}", is_valid);
}
