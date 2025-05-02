/**
 * 가변 참조자는 하나만 가질 수 있다.
 * 불변 참조자는 여러개를 가질 수 있다.
 * 참조자는 항상 유효해야 한다.
 */

fn main() {
    immutable_reference();
    mutable_reference();
    single_mutable_borrow();
    mutable_borrow_scope();
    dangling_reference();
}

fn immutable_reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 는 s1의 참조자로 s1의 포인터만 참조한다.

    println!("The length of '{}' is {}.", s1, len); // calculate_length로 s1의 소유권이 넘어간게 아니기 때문에 s1을 사용할 수 있다.
}

fn mutable_reference() {
    let mut s = String::from("hello");
    change(&mut s); // 가변 참조자. 위의 코드처럼 그냥 참조자를 넘기면 수정할 수 없다.
    println!("Changed string: {}", s);
}

fn single_mutable_borrow() {
    let mut ss = String::from("hello");
    let r1 = &mut ss;
    // let r2 = &mut ss; // Compile Error. 어떤 값에 대한 가변 참조자는 한번만 참조할 수 있다.
    // println!("{} {}", r1, r2);
    println!("First mutable borrow: {}", r1);
}

fn mutable_borrow_scope() {
    let mut sss = String::from("hello");
    {
        let r2 = &mut sss;
        println!("Inner scope mutable borrow: {}", r2);
    } // r2가 스코프를 벗어나면 버려지기 때문에 가변참조자를 다시 만들 수 있다.
    let r3 = &mut sss;
    println!("Outer scope mutable borrow after inner scope: {}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn dangling_reference() {
    // let reference_to_nothing = dangle();
    let no_dangle = no_dangle();
    println!("{}", no_dangle);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // s가 스코프를 벗어나면 버려지기 때문에 참조자를 반환할 수 없다.
// }

fn no_dangle() -> String {
    String::from("hello")
}
