struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("username@user.com"),
        sign_in_count: 1,
    };

    println!("user1.email: {}", user1.email);
    user1.email = String::from("anotherusername@user.com");
    println!("user1.email: {}", user1.email);

    let user2 = build_user("djose1164".to_string(), "djose1164@gmail.com".to_string());
    println!("{} {}", user2.username, user2.email);

    // Update syntax
    let user3 = User {
        email: String::from("user3@email.com"),
        ..user1
    };
    println!("user3: {} {} {}", user3.email, user3.username, user3.sign_in_count);
}
