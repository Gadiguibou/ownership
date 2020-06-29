use std::cmp;

fn main() {
    let s = String::from("hello, world! My name is Gabriel.");

    println!("The first word of '{}' is '{}'.", s, find_nth_word(&s, 1));
    println!("The second word of '{}' is '{}'.", s, find_nth_word(&s, 2));
    println!("Word #{} is '{}'.", 6, find_nth_word(&s, 6));
    println!("Word #{} is '{}'.", 12, find_nth_word(&s, 12));

}

fn find_nth_word (s: &str, n: u32) -> &str {
    let bytes = s.as_bytes();

    fn find_first_space (b: &[u8]) -> usize {
        for (i, &item) in b.iter().enumerate() {
            if item == b' ' {
                return i
            }
        }
        b.len()
    }

    let mut word_start: usize = 0;
    let mut word_end: usize = 0;

    for i in 0..n {

        if i != 0 {
            word_start = cmp::min(word_end + 1, bytes.len());
        }

        word_end = word_start + find_first_space(&bytes[word_start..]);

    }

    &s[word_start..word_end]
}