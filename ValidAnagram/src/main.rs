use std::collections::HashSet;

fn is_anagram(s: &'static str, t: &'static str) -> bool {
    let mut s_set:HashSet<char> = HashSet::new();
    let mut t_set:HashSet<char> = HashSet::new();

    for ss in s.chars() {
        s_set.insert(ss);
    }

    for tt in t.chars() {
        t_set.insert(tt);
    }

    if s_set == t_set {
        true
    } else {
        false
    }
}

fn main() {
    println!("{}", is_anagram("anagram", "nagaram"));
    println!("{}", is_anagram("anagram", "abcd"));
}
