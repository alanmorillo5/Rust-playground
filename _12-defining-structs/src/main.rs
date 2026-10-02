fn main() {
    // DEFINING STRUCTS
    // Structs are very similar to tuples, but have named properties.
    // this is a tuple.
    let rect = (200, 500);

    let mut user1 = User{
        active: true,
        username: String::from("myuser"),
        email: String::from("my@email.com"),
        sign_in_count: 1
    };

    user1.email = String::from("another.email.com");
    println!("User email is {}.", user1.email);

    fn build_user(email: String, username: String) -> User {
        User{
            active: true,
            email, // field init shorthand
            username,
            sign_in_count: 1
        }
    }

    // Create instance from other instance
    let user2 = User {
        email: String::from("another.email.com"),
        ..user1
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // Unit-like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

// this is a struct.
struct Book {
    title: String,
    author: String,
    pages: i32,
    available: bool
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}