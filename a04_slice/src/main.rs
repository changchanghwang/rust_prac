fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);

    // s.clear(); // 가변참조자가 필요하나 word의 참조자를 사용하므로 불변참조자가 사용되어 오류 발생

    println!("{}", word);

    string_slice();
}

fn string_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // 0~5 문자열
    let world = &s[6..11]; // 6~11 문자열 포인터가 가리키는 메모리 주소의 인덱스 값이 6~11 범위의 문자열을 가리킴

    println!("{} {}", hello, world);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}
