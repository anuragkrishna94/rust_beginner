enum IpAddrKind {
     V4,
     V6
}

struct IpAddr {
     kind: IpAddrKind,
     address: String
}

// define enum with string for each variant
enum IpAddr1 {
     V4(String),
     V6(String)
}

enum IpAddr2 {
     V4(u8,u8,u8,u8),
     V6(String)
}

#[derive(Debug)]
enum Coin {
     Penny,
     Nickel,
     Dime,
     Quarter
}

pub fn chapter6_main() {
     // Simple usage of enum
     let ip_v4: IpAddrKind = IpAddrKind::V4;
     let ip_v6: IpAddrKind = IpAddrKind::V6;
     route(&ip_v4);
     route(&ip_v6);

     // Simple example to demonstrate usage of enum within struct.
     let home: IpAddr = IpAddr {
          kind: ip_v4,
          address: String::from("127.0.0.1")
     };
     let loopback: IpAddr = IpAddr {
          kind: ip_v6,
          address: String::from("::1")
     };

     // Using enums that also have an associated type for each variant
     let home1: IpAddr1 = IpAddr1::V4(String::from("127.0.0.1"));
     let loopback1: IpAddr1 = IpAddr1::V6(String::from("::1"));

     let home2: IpAddr2 = IpAddr2::V4(127,0,0,1);
     let loopback2: IpAddr2 = IpAddr2::V6(String::from("::1"));

     // Usage of Option<T> enum which is an awesome feature to store null and not-null values.
     let not_null_number: Option<i32> = Some(22);
     println!("'not_null_number' is: {}", not_null_number.unwrap());
     let null_number: Option<i32> = None;
     println!("'null_number' is: {}", null_number.unwrap_or(0));

     // Usage of match
     let coinChoice: Coin = Coin::Penny;
     let anotherConChoice: &Coin = &coinChoice;
     println!("Value of 'coinChoice': {:#?}, is: {1}", coinChoice, get_value_in_cents(anotherConChoice));

     // Demonstrate how match-catch all pattern works. 
     // other is used when the value is needed
     // _ is used when the value is NOT required.
     let position: u32 = 0;
     let dice_roll: u32 = 10;
     match dice_roll {
          3 => case_3(),
          7 => case_7(),
          other => move_positions(other, position)
     }

     match dice_roll {
          3 => case_3(),
          7 => case_7(),
          _ => re_roll()
     }
}

fn route(ipKind: &IpAddrKind) {}

fn get_value_in_cents(coin: &Coin) -> u8 {
     match coin {
          Coin::Penny => 1,
          Coin::Nickel => 5,
          Coin::Dime => 10,
          Coin::Quarter => 25
     }
}

fn case_3(){
     println!("Case 3");
}
fn case_7(){
     println!("Case 7");
}
fn move_positions(num_of_positions: u32, mut pos: u32){
     pos += num_of_positions;
     println!("New Position is: {}", pos);
}
fn re_roll(){
     println!("Rerolling");
}