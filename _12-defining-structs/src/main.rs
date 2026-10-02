#[allow(dead_code)]
fn main() {
    // DEFINING STRUCTS
    // Structs are very similar to tuples, but have named properties.
    // this is a tuple.
    let _rect = (200, 500);

    let mut user1 = User{
        _active: true,
        _username: String::from("myuser"),
        email: String::from("my@email.com"),
        _sign_in_count: 1
    };

    user1.email = String::from("another.email.com");
    println!("User email is {}.", user1.email);

    // Create instance from other instance
    let _user2 = User {
        email: String::from("another.email.com"),
        ..user1
    };

    // Tuple Structs
    struct _Color(i32, i32, i32);
    struct _Point(i32, i32, i32);

    let _black = _Color(0, 0, 0);
    let _white = _Color(255, 255, 255);

    // Unit-like struct
    struct _AlwaysEqual;
    let _subject = _AlwaysEqual;
}

// this is a struct.
#[allow(dead_code)]
struct _Book {
    title: String,
    author: String,
    pages: i32,
    available: bool
}

struct User {
    _active: bool,
    _username: String,
    email: String,
    _sign_in_count: u64
}
