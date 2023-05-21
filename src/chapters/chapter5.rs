// Definition of User struct
struct User {
     active: bool,
     username: String,
     email: String,
     sign_in_count: u64
}

// Definitions of tuple structs
struct Color(u32, u32, u32);
struct PositivePoint(u32, u32, u32);

// Rectangle struct
struct Rectangle {
     width: u32,
     height: u32
}

// This function is intended to mimic the main function for chapter 5 of rust book
pub fn chapter5_main() {
     // For a struct instance to be mutable, entire instance must be labeled as mutable.
     let mut user1: User = User {
          active: true,
          username: String::from("anuragkrishna94"),
          email: String::from("abc@xyz.com"),
          sign_in_count: 1
     };
     println!("User's email is : {0}", user1.email);
     user1.email = String::from("new_email@xyz.com");
     println!("User's new email is : {0}", user1.email);

     // build a user with a function
     let user2: User = build_user("email@abc.com", "username123");
     println!("User 2's email is : {0}", user2.email);

     let user3 = build_user_with_shorthand(String::from("new_email@abc.com"), String::from("new_username123"));
     println!("User 3's email is : {0}", user3.email);

     // Instantiate a new user3 instance using .. syntax. (Similar to javascript's spread operator)
     let user4: User = User {
          email: String::from("user3@abc.com"),
          ..user3
     };
     println!("User 4's  email: {0}, username: {1}", user4.email, user4.username);

     // Uncomment the following line to see error that gets generated. Since new email value was created for user4
     // and username (being String) was copied over into user4, user3.username has been moved and cannot be used.
     // println!("User 3's new email is : {0}", user3.username);

     // Color and PositivePoint structs cannot be interchanged. They are instances of 2 different structs.
     let white = Color(255,255,255);
     let origin = PositivePoint(0,0,0);

     // Calculate area of rectangle without structs.
     let width1 = 30;
     let height1 = 50;

     println!("Area of rectangle is: {}", react_area(width1, height1));

     // Calcuate area of retangle with Tuple.
     // Point to note here is there is no distinction between width and height.
     let rect1 = (30, 50);
     println!("Area of rectangle is: {}", react_area_with_tuple(rect1));

     // Calculate area of rectangle defined by struct
     let rect2 = Rectangle {
          width: 30,
          height: 50
     };
     println!("Area of rectangle is: {}", react_area_with_struct(&rect2));
}

// Builds a user by taking in email and username as parameters
fn build_user(email: &str, username: &str) -> User {
     User {
          active: true,
          email: String::from(email),
          username: String::from(username),
          sign_in_count: 1
     }
}

// This method will only if the parameter name and field names within User struct are IDENTICAL
fn build_user_with_shorthand(email: String, username: String) -> User {
     User {
          active: true,
          email,
          username,
          sign_in_count: 1
     }
}

fn react_area(width: u32, height: u32) -> u32 {
     width * height
}

fn react_area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Since this function only borrows rect, the function chapter5_main still has the ownership of rect2.
fn react_area_with_struct(rect: &Rectangle) -> u32 {
     rect.width * rect.height
 }