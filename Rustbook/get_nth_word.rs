#[allow(unused_variables)]

use std::io;
use std::io::Write;

fn first_word(s:  &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str {
    let str_bytes = s.as_bytes();
    let mut word_start = None;

    for (i, &item) in str_bytes.iter().enumerate() {
        if item == b' ' {
            if let Some(start) = word_start {
                return &s[start..i];
            }
            word_start = Some(i + 1);
        }
    }

    word_start.map_or("", |start| &s[start..])
}

fn nth_word(s: &str, n: usize) -> Option<&str> {
    let bytes = s.as_bytes();
    let mut word_start = None;
    let mut word_count = 1;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if let Some(start) = word_start {
                word_count += 1;
                if word_count == n {
                    return Some(&s[start..i]);
                }
            }
            word_start = Some(i + 1);
        }
    }

    word_start.and_then(|start| {
        word_count += 1;
        if word_count == n {
            Some(&s[start..])
        } else {
            None
        }
    })
}

fn main() {
    let mut s = String::from("Man hello world");
    let _word = first_word(&s);
    let _sec_word = second_word(&s);

    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
    writeln!(handle, "first word is {}", _word); // add `?` if you care about errors here
    writeln!(handle, "second word is {}", _sec_word); // add `?` if you care about errors here
    let third_word = nth_word(&s, 3).map_or("", |mystr_some| mystr_some);
    writeln!(handle, "third word is {}", third_word);
    s.clear();
}
