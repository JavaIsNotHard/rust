struct User {
    activity: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        activity: false,
        username: String::from("jibesh"),
        email: String::from("jibeshshrestha00@gmail.com"),
        sign_in_count: 9864477586,
    };

    // user1.active = true; // this is not possible becuase user1 is not mutable

    // are we moving here was well? no not at all
    let mut user2 = User {
        activity: true,
        username: String::from("jibesh"),
        email: String::from("jibeshshrestha00@gmail.com"),
        sign_in_count: 9864477586,
    };

    // we are moving the value of user1 to user3 which means that now user1's username field is no longer the available since user3 is the new owner
    let user3 = User {
        email: String::from("james"),
        ..user1
    };

    user2.activity = false;
}

fn build_user(username: String, email: String) -> User {
    User {
        activity: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// field init shorthand where we do not need to specify the parameter name in the structure field value if they match
fn build_user_two(username: String, email: String) -> User {
    User {
        activity: true,
        username,
        email,
        sign_in_count: 1,
    }
}
