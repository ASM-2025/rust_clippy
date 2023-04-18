#![allow(unused, clippy::match_like_matches_macro)]
#![warn(clippy::has_loop)]

fn no_warn(a: &[i32]) -> i32 {
    *a.iter().min().unwrap()
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn main() {
    if true {
        println!("don't warn me");
    }
    no_warn(&[1, 2, 10, -5, -18, 23]);
    assert!(is_vowel('a'));
}
