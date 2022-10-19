fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    // s.clear(); // 错误!

    println!("the first word is: {}", word);
}

// 不使用 slice，没办法返回字符串，只能返回索引。
fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}