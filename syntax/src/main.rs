fn main() {
    shadowing();
    println!("--------------------------------");
    data_types("42");
    println!("--------------------------------");
    func();
    println!("--------------------------------");
    control_flow();
    println!("--------------------------------");
    iteration();
}

/**
 * 쉐도윙은 새 변수를 이전 변수명과 같은 이름으로 선언하는 것
 * 이전 변수와 다른 타입이어도 된다.
 */
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = "hello";
        println!("x in the inner scope is: {x}");
    }

    println!("x is: {x}");
}

fn data_types(num_str: &str) {
    let number: u32 = num_str.parse().expect("Not a number!");
    println!("string number is: {number}");

    let _hex = 0x1234; // 16진수
    let _octal = 0o1234; // 8진수
    let _binary = 0b1010; // 2진수
    let _byte = b'A'; // 바이트
    let _float = 0.3; // 부동소수점
    let _float32: f32 = 0.3; // 32비트 부동소수점
    let _f = false; // 불리언
    let _c = 'A'; // 문자
    let _s = "Hello, world!"; // 문자열
    let _tuple = (1, 2, 3); // 튜플
    let array = [1, 2, 3, 4, 5]; // 배열
    let _slice = &array[0..2]; // 슬라이스
}

fn func() {
    let y = {
        let x = 3;
        x + 1 // 마지막 표현식이 함수의 반환값이 된다. 세미콜론이 없으면 반환값이 된다.
    };
    println!("y is: {y}");
}

fn control_flow() {
    let num = 3;
    if num < 5 {
        println!("number({num}) was under 5");
    } else {
        println!("number({num}) was over 5");
    }

    let condition = num < 5;
    let under_five = if condition { "true" } else { "False" };
    println!("number is under 5: {under_five}");
}

fn iteration() {
    // 무한 루프
    // loop {
    //     println!("loop");
    // }

    // loop 반환값
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 9 {
            break count + 1;
        }
    };
    println!("result is: {result}");

    // label loop
    let mut c = 0;
    'counting_up: loop {
        println!("c = {c}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // 레이블이 없는 브레이크
            }
            if c == 2 {
                break 'counting_up; // 레이블을 사용한 브레이크
            }
            remaining -= 1;
        }

        c += 1;
    }
    println!("End count = {c}");

    // while loop
    let mut nn = 3;
    while nn != 0 {
        println!("{nn}!");
        nn -= 1;
    }
    println!("LIFTOFF!!!");

    // for loop -> collection
    let arr = [10, 20, 30, 40, 50];
    for el in arr {
        println!("el is: {el}");
    }

    // for loop -> range
    // rev() -> 역순
    for num in (1..4).rev() {
        println!("num is: {num}");
    }
}
