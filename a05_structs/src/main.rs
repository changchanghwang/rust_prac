struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 일부필드만 가변성으로 만들 수 없음.
    let mut user = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user.email = String::from("anotheremail@example.com");

    let user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2 // user2의 email을 제외한 나머지 필드를 가져옴
    };

    // println!("{}", user2.username); // compile error. user2.username는 이미 이동되었기 때문에 사용할 수 없음.
    println!("{}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
