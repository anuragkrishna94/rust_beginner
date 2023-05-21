// Definition of User struct
struct User {
     active: bool,
     username: String,
     email: String,
     sign_in_count: u64
}

pub fn chapter5_main() {
     // For a struct instance to be mutable, entire instance must be labeled as mutable.
     let mut user1 = User {
          active: true,
          username: String::from("anuragkrishna94"),
          email: String::from("abc@xyz.com"),
          sign_in_count: 1
     };
     println!("User's email is : {0}", user1.email);
     user1.email = String::from("new_email@xyz.com");
     println!("User's new email is : {0}", user1.email);

     // build a user with a function
     let user2 = build_user("email@abc.com", "username123");
     println!("User 2's email is : {0}", user2.email);
}

fn build_user(email: &str, username: &str) -> User {
     User {
          active: true,
          email: String::from(email),
          username: String::from(username),
          sign_in_count: 1
     }
}