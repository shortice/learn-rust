pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    pub fn print(&self) {
        println!(
            "User: {0}\nEmail: {1}\nActive: {2}\nSign in count: {3}",
            self.username,
            self.email,
            self.active,
            self.sign_in_count
        );
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

pub fn struct_rustics(){
    let user = build_user(
        String::from("Danila"),
        String::from("me@me.com")
    );

    user.print();
}
