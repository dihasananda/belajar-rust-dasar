use std::result;

fn main() {
    println!("Hello, world!");
    
    println!("Hello, eko!");
}

#[test]
fn hello_test(){
    println!("hello test");
}

#[test]
fn test_variable(){
    let name = "adpes alamin";
    println!("hello {}", name);
}

#[test]
fn test_mutable(){
    let mut name = "adpes al";
    println!("hello {}", name);
    
    name = "al adpes";
    println!("hello {}", name);
}

#[test]
fn static_typing(){
    let name = "adpes al";
    println!("hello {}", name);
    
    println!("hello {}", name);
}

#[test]
fn shadowing(){
    let name = "adpes al";
    println!("hello {}", name);
    
    let name = 10;
    println!("hello {}", name);
}

/*
komen
komen
komen
 */

 //komen komen

 #[test]
 fn explicit(){
    let age: f64 = 20.5;
    println!("hello {}", age);
 }

 #[test]
 fn number_conversion(){
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i16 = 1000;
    let d: i8 = c as i8;
    println!("{}", d)
 }

 #[test]
 fn numeric_operator(){
    let a = 10;
    let b = 10;

    let c = a + b;
    let d = a - b;
    let e = a / b;
    let f = a * b;
    let g = a % b;
    
    println!("{} {} {} {} {}", c, d, e, f, g)
 }

 #[test]
 fn augmented_assignment(){
    let mut a = 10;
    println!("{}", a);
    
    a += 10;
    println!("{}", a);
    
    a -= 10;
    println!("{}", a);
    
    a *= 10;
    println!("{}", a);
    
    a /= 10;
    println!("{}", a);
    
    a %= 10;
    println!("{}", a);
 }

 #[test]
 fn boolean(){
    let a = true;
    let b = false;
    println!("{} {}", a, b);
 }

 #[test]
 fn comparison(){
    let a = 10;
    let b = 20;

    let result = a > b;
    println!("{}", result);
 }

 #[test]
 fn boolean_operator(){
    let a = 10;
    let b = 20;

    let ab = a > b;
    let ba = b > a;

    let and = ab && ba;
    let or = ab || ba;
    let neg = !ab;

    println!("{} {} {}", and, or, neg);
 }

 #[test]
 fn char_type(){
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
 }

 #[test]
 fn tuple(){
    let mut data = (10, 10.5, true);

    println!("{:?}", data);
    println!("{} {} {}", data.0, data.1, data.2);

    data.0 = 20;
    data.1 = 20.2;
    data.2 = false;
    
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
    
    data.0 = 30;
    data.1 = 30.2;
    data.2 = true;

    let (a, b, _) = data;
    println!("{} {} {}", a, b, c);
 }

fn unit() {
    println!("hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);
    
    let test = ();
    println!("{:?}", test);
}

#[test]
fn array() {
   let mut array = [1, 2, 3, 4, 5];
   println!("{:?}", array);
   
   let a = array[0];
   let b = array[1];
   println!("{} {}", a, b);
   
   array[0] = 10;
   array[1] = 20;
   println!("{:?}", array);

   let length = array.len();
   println!("{}", length);
}

#[test]
fn two_dimensional_array() {
   let matrix = [
      [1, 2],
      [3, 4]
   ];

   println!("{:?}", matrix);
   println!("{} {} ", matrix[0][0], matrix[0][1]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant() {
   const MINIMUM: i32 = 50;
   println!("{} {}", MAXIMUM, MINIMUM);
}

#[test]
fn variable_scope() {
   let dihas = 1;

   {
      println!("inner dihas: {}", dihas);
      let ananda = 2;
      println!("inner ananda: {}", ananda);
   }

   // println!("inner ananda: {}", ananda); //error
}

#[test]
fn stack_heap() {
   function_a();
   function_b();
}

fn function_b() {
   let a = 10;
   let b = String::from("dihas");

   println!("{} {}", a, b);
}

fn function_a() {
   let a = 10;
   let b = String::from("ananda");

   println!("{} {}", a, b);
}

#[test]
fn string() {
   let name = "   dihas ananda    ";
   let trim = name.trim();

   println!("{}", trim);
}

#[test]
fn string_type() {
   let mut name = String::from("dihas");
   name.push_str(" ananda");
   println!("{}", name);

   let name_new = name.replace("dihas", "ananda");
   println!("{}", name_new);
}

#[test]
fn ownership_rules() {
   // a tidak bisa diakses disini, belum dideklarasikan
   let a = 10; // a bisa diakses mulai disini

   { // b tidak bisa diakses disini, belum dideklarasikan
      let b = 20; // b bisa diakses mulai disini
      println!("{}", b);
   } // scope b selesai, b dihapus, b tidak bisa diakses lagi

   println!("{}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses lagi

#[test]
fn data_copy() {
   let a = 10;
   let b = a;
   println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
   let name1 = String::from("dihas");
   println!("{}", name1);
   
   let name2 = name1; // owner name1 pindah ke name2
   
   // println!("{}", name1); // error
   println!("{}", name2);
}

#[test]
fn clone() {
   let name1 = String::from("dihas");
   let name2 = name1.clone();

   println!("{} {}", name1, name2);
}

#[test]
fn if_expression() {
   let value = 9;
   let result = if value >= 8 {
      "good"
   } else if value >= 6 {
      "not good"
   } else if value >= 3 {
      "bad"
   } else {
      "very bad"
   };

   println!("{}", result);
}

#[test]
fn loop_expression() {
   let mut counter = 0;
   loop {
      counter += 1;
      if counter > 10 {
         break;
      } else if counter % 2 == 0 {
         continue;
      }

      println!("counter: {}", counter);
   }
}

#[test]
fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
      counter += 1;
      if counter > 10 {
         break counter * 2;
      }
   };

   println!("{}", result);
}

#[test]
fn loop_label() {
   let mut number = 1;
    'outer: loop {
      let mut i = 1;
      loop {
         if number > 10 {
            break 'outer;
         }

         println!("{} x {} = {}", number, i, number * i);

         i += 1;
         if i > 10 {
            break;
         }
      }

      number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
      if counter % 2 == 0 {
         println!("counter: {}", counter);
      }

      counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array = ["a", "b", "c", "d", "e"];
    let mut index = 0;
    
    while index < array.len() {
       println!("value: {}", array[index]);
       index += 1;
      }
      
      for value in array {
         println!("value {}", value);
      }
   }
   
#[test]
fn range() {
   let array = ["a", "b", "c", "d", "e"];

   let range = 0..5;
   println!("start: {}, end: {}", range.start, range.end);

   for i in 0..5 {
       println!("{}", array[i]);
   }
}
   
#[test]
fn range_inclusive() {
   let array = ["a", "b", "c", "d", "e"];

   let range = 0..=4;
   println!("start: {}, end: {}", range.start(), range.end());

   for i in 0..=4 {
       println!("{}", array[i]);
   }
}

// function
fn say_hello() {
   println!("hello");
}

#[test]
fn test_say_hello() {
    say_hello();
    say_hello();
    say_hello();
    say_hello();
}

// parameter funtion
fn say_goodbye(first_name: &str, last_name: &str) {
    println!("say goodbye {} {}", first_name, last_name)
}

#[test]
fn test_say_goodbye() {
    say_goodbye("dihas", "ananda");
    say_goodbye("eko", "susilo");
}

// return function
fn factorial_loop(n: i32) -> i32 {
   if n < 1 {
       return 0;
   }

   let mut result = 1;
   for i in 1..=n {
       result *= i;
   }

   return result;
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("{}", result);
    
    let result = factorial_loop(-2);
    println!("{}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("dihas"), 10)
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("{}", result);
}

fn print_number(number: i32) {
    println!("number: {}", number);
}

fn hi(name: String) {
    println!("Hi, {}", name);
}

#[test]
fn test_hi() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Dihas");
    hi(name);
    // println!("{}", name);
}

fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("dihas");
    let last_name = String::from("ananda");

    let name = full_name(&first_name, &last_name);

    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(value: &mut String) {
   value.push_str("test");
}

#[test]
fn test_change_value() {
    let mut value= String::from("dihas ");
    let value_borrow = &mut value;

    change_value(value_borrow);
    change_value(value_borrow);
    change_value(value_borrow);

    println!("{}", value);
}
    
fn get_full_name(first_name: &String, last_name: &String) -> String {
   let name = format!("{} {}", first_name, last_name);
   return name;
}

#[test]
fn test_get_full_name() {
   let first_name = String::from("dihas");
   let last_name = String::from("ananda");

   let full_name = get_full_name(&first_name, &last_name);

   println!("{}", full_name);
   println!("{}", first_name);
   println!("{}", last_name);
}

#[test]
fn slice_reference() {
   let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

   let slice1 = &array[..];
   println!("{:?}", slice1);

   let slice2 = &array[1..5];
   println!("{:?}", slice2);

   let slice3 = &array[..5];
   println!("{:?}", slice3);

   let slice4 = &array[5..];
   println!("{:?}", slice4);
}

#[test]
fn string_slice() {
    let name = String::from("dihas ananda");
    let first_name = &name[..5];
    println!("{}", first_name);
    
    let last_name = &name[6..];
    println!("{}", last_name);
}

// STRUCT
struct Person {
   first_name: String,
   last_name: String,
   age: u8,
}

#[test]
fn struct_person() {
   let first_name = String::from("dihas");
   let last_name = String::from("ananda");


   let mut person = Person {
      first_name,
      last_name,
      age: 25,
   };

   person.first_name = String::from("Dihas");

   // println!("{}", first_name); //error

   
   let person2 = Person { 
      first_name: person.first_name.clone(),
      last_name: person.last_name.clone(),
      ..person 
   };
   
   print_person(&person);
   print_person(&person2);
}

fn print_person(person: &Person) {
   println!("{} {} {}", person.first_name, person.last_name, person.age);
}

struct GeoPoint(f64, f64);

#[test]
fn tuple_struct() {
    let geo_point = GeoPoint(125.5, 256.4);
    println!("{} {}", geo_point.0, geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
   //diberi underscore di depan variable jika tidak berinat digunakan
    let _nothing = Nothing; 
    let _nothing2 = Nothing{};
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("hello  {}, my name is {}", name, self.first_name);
    }
}

#[test]
fn test_method() {
    let person = Person {
      first_name: String::from("dihas"),
      last_name: String::from("ananda"),
      age: 20,
    };

    person.say_hello("budi");
}

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
      GeoPoint(long, lat)
    }
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(2.5, 5.5);
    println!("{} {}", geo_point.0, geo_point.1);
}

enum Level {
   Regular,
   Premium,
   Platinum,
}

#[test]
fn test_enum() {
    let level = Level::Regular;
    let _level2 = Level::Platinum;
    let _level3 = Level::Premium;

    match level {
        Level::Regular => {
         println!("Regular");
        }
        Level::Premium => {
         println!("Premium");
        }
        Level::Platinum => {
         println!("Platinum");
        }
    }
}

enum Payment {
   CreditCart(String),
   BankTransfer(String, String),
   EWallet(String, String),
}

impl Payment {
   fn pay (&self, amount: u32) {
      match self {
         Payment::CreditCart(number) => {
            println!("paying with credit card {} amount {}", number, amount)
         }
         Payment::BankTransfer(bank, number) => {
            println!("paying with Bank Transfer {} {} amount {}", bank, number, amount)
         }
         Payment::EWallet(wallet, number) => {
            println!("paying with e wallet {} {} amount {}", wallet, number, amount)
         }
      }
   }
}

#[test]
fn test_payment() {
    let _payment1 = Payment::CreditCart(String::from("123455"));
    _payment1.pay(50000);
    
    let _payment2 = Payment::BankTransfer(String::from("BCA"), String::from("123123"));
    _payment2.pay(50000);
    
    let _payment3 = Payment::EWallet(String::from("Gopay"), String::from("123123"));
    _payment3.pay(50000);
}

#[test]
fn test_match_value() {
   let name = "dihas";
   match name {
      "dihas" => {
      println!("hello dihas!");
       }
      "ananda" => {
      println!("hello ananda!!!");
       }
      other => {
      println!("hello {}", other);
       }
   }

   match name {
      "dihas" | "ananda" => {
      println!("hello boss!");
       }

      other => {
      println!("hello non boss {}", other);
       }
   }
}

#[test]
fn test_range_pattern() {
    let value = 100;
    match value {
      75..=100 => {
         println!("great");
      }
      50..=74 => {
         println!("good");
      }
      25..=49 => {
         println!("not bad");
      }
      0..=24 => {
         println!("bad");
      }
      other => {
         println!("invalid value {}", other);
      }
    }
}

#[test]
fn test_struct_pattern() {
    let point = GeoPoint(0.0, 5.6);
    match point {
        GeoPoint(long, 0.0) => {
         println!("long: {}", long);
        }
        GeoPoint(0.0, lat) => {
         println!("lat: {}", lat);
        }
        GeoPoint(long, lat) => {
         println!("long: {}, lat: {}", long, lat);
        }
    }
    let person = Person{
      first_name: String::from("dihas"),
      last_name: String::from("ananda"),
      age: 25,
    };
    match person {
        Person { first_name, last_name, .. } => {
         println!("{} {}", first_name, last_name);
        }
    }
}

#[test]
fn test_ignoring() {
   let point = GeoPoint(0.0, 5.6);
   match point {
       GeoPoint(long, _) => {
        println!("long: {}", long);
       }
   }
}

#[test]
fn test_ignoring_range() {
   let value = 999;
   match value {
     75..=100 => {
        println!("great");
     }
     50..=74 => {
        println!("good");
     }
     25..=49 => {
        println!("not bad");
     }
     0..=24 => {
        println!("bad");
     }
     _ => {
        println!("invalid value");
     }
   }
}

#[test]
fn test_match_expression() {
    let value = 33;
    let result = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid"
    };

    println!("{}", result);
}