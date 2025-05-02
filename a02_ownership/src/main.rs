/**
 * 러스트의 소유권은 주로 힙 메모리 영역의 데이터 관리
 * 각각의 값은 소유자가 정해져 있음 (owner)
 * 한 값의 소유자는 여럿이 될 수 없음
 * 소유자가 스코프 밖으로 벗어나면 값이 버려짐 (drop)
 * scala값은 스택에 저장되므로 복사됨 -> 단순한 값들. 정수형, 불리언, 문자열, 또는 이들로 구성된 튜플
 */

fn main() {
    ownership_scope();
    ownership_move();
    ownership_copy();
    ownership_function();
    ownership_with_tuple();
}

fn ownership_scope() {
    {
        // s는 아직 선언되지 않아 여기서는 유효하지 않음
        let s = String::from("hello"); // 이 지점부터 s가 유효
        println!("{}", s);
    } // 스코프가 종료되어 s가 여기서부터는 유효하지 않아 여기서 버려짐. drop 함수가 자동으로 호출됨 (C++에서의 RAII, Resource Acquisition Is Initialization)
    // println!("s: {}", s); // compile error. s는 더 이상 유효하지 않아 버려짐
}

fn ownership_move() {
    let s1 = String::from("hello"); // 이때 s1에서 실제 hello를 포인터로 저장하고 있음. 길이는 String의 내용이 현재 사용하고 있는 메모리를 바이트 단위로 나타낸 것이고, 용량은 String에 할당한 메모리 양
    let s2 = s1; // s1의 소유권이 s2로 이전됨. 힙메모리의 데이터는 복사되지 않고 포인터만 복사됨. 러스트는 절대 자동으로 ‘깊은’ 복사로 데이터를 복사하는 일이 없음.
    // println!("s1: {}", s1); // compile error. s1은 더 이상 유효하지 않아 버려짐
    println!("s2: {}", s2); // s2는 이제 hello를 가리키고 있음

    // 깊은 복사
    let s3 = String::from("hello");
    let s4 = s3.clone(); // s3의 데이터를 복사하여 s4에 할당
    println!("s3: {}, s4: {}", s3, s4); // s3와 s4는 같은 값을 가지므로 둘 다 hello
}

fn ownership_copy() {
    let x = 5; // 정수형 값은 스택에 저장되므로 복사됨
    let y = x; // x의 값이 복사되어 y에 저장됨
    println!("x: {}, y: {}", x, y); // x와 y는 같은 값을 가지므로 둘 다 5
}

fn ownership_function() {
    let s = String::from("hello");
    takes_ownership(s); // s의 소유권이 takes_ownership 함수로 이전됨
    // println!("s: {}", s); // compile error. s는 더 이상 유효하지 않아 버려짐

    let x = 5;
    makes_copy(x); // x의 값이 makes_copy 함수로 복사됨
    println!("x: {}", x); // x는 여전히 유효하므로 값을 출력할 수 있음

    let s1 = gives_ownership(); // gives_ownership에서 소유권을 가져옴
    let s2 = takes_ownership_and_returns(s1); // s1의 소유권이 takes_ownership_and_returns 함수로 이전되고, 반환된 값이 s2에 할당됨
    // println!("s1: {}", s1); // compile error. s1은 더 이상 유효하지 않아 버려짐
    println!("s2: {}", s2); // s2는 이제 hello를 가리키고 있음
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s // s의 소유권이 반환됨
}
fn makes_copy(x: i32) {
    println!("{}", x);
}
fn takes_ownership(s: String) {
    println!("{}", s);
}
fn takes_ownership_and_returns(s: String) -> String {
    println!("{}", s);
    s
}

fn ownership_with_tuple() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("s2: {}, len: {}", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
