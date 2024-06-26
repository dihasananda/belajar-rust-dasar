mod first;
mod second;
mod third;
mod model;


use first::say_hello;
use second::say_hello as say_hello_second;
use model::User;

#[test]
fn test_use() {
   say_hello();
   say_hello_second();
   first::second::third::say_hello();
}

#[test]
fn test_module() {
   let user = User {
      first_name: String::from("dihas"),
      last_name: String::from("ananda"),
      username: String::from("dihasananda"),
      email: String::from("dihas@ananda.com"),
      age: 20,
   };
   
   user.say_hello("budi");
}
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
// fn say_hello() {
//    println!("hello");
// }

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

// Type Alias
type Age = u8;
type IdentityNumber = String;

struct Customer {
   id: IdentityNumber,
   name: String,
   age: Age,
}

#[test]
fn test_type_alias() {
    let customer = Customer {
      id: String::from("23456"),
      name: String::from("dihas"),
      age: 25,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}

trait CanSayHello {
   fn hello(&self) -> String {
      String::from("hello")
   } 
   fn say_hello(&self) -> String;
   fn say_hello_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
   fn say_hello(&self) -> String {
       format!("Hello, my name is {}", self.first_name)
   }

   fn say_hello_to(&self, name: &str) -> String {
       format!("Hello {}, my name is {}", name, self.first_name)
   }
}

fn say_hello_trait(value: &impl CanSayHello) {
   println!("{}", value.say_hello());
}

trait CanSayGoodBye {
   fn good_bye(&self) -> String;
   fn good_bye_to(&self, name: &str) -> String;
}

impl CanSayGoodBye for Person {
   fn good_bye(&self) -> String {
       format!("goodbye, my name is {}", self.first_name)
   }

   fn good_bye_to(&self, name: &str) -> String {
       format!("goodbye {} my name is {}", name, self.first_name)
   }
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodBye)) {
   println!("{} {}", value.say_hello(), value.good_bye());
}

#[test]
fn test_trait() {
    let person = Person {
      first_name: String::from("dihas"),
      last_name: String::from("ananda"),
      age: 25,
    };

    say_hello_trait(&person);
    hello_and_goodbye(&person);

    let result2 = person.say_hello_to("budi");
    println!("{}", result2);

    let result3 = person.hello();
    println!("{}", result3);

    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("budi"));

    CanSayHello::say_hello(&person);
    Person::say_hello(&person, "budi");
   }

struct SimplePerson {
   name: String
}

impl CanSayGoodBye for SimplePerson {
   fn good_bye(&self) -> String {
       format!("Goodbye, my name is {}", self.name)
   }
   fn good_bye_to(&self, name: &str) -> String {
       format!("goodbye {}, my name is {}", name, self.name)
   }
}

fn create_person(name: String) -> impl CanSayGoodBye {
   SimplePerson { name }
}

#[test]
fn test_return_trait() {
    let person = create_person(String::from("dihas"));
    println!("{}", person.good_bye());
    println!("{}", person.good_bye_to("budi"));
}

// trait CanSay: CanSayHello + CanSayGoodBye {
//    fn say(&self) {
//       println!("{}", self.say_hello());
//       println!("{}", self.good_bye());
//    }
// }

// struct SimpleMan {
//    name: String,
// }

// impl CanSay for SimpleMan {
    
// }

// Generic
struct Point<T = i32> {
   x: T,
   y: T,
}

impl<T> Point<T> {
   fn get_x(&self) -> &T {
      &self.x
   }

   fn get_y(&self) ->  &T {
      &self.y
   }
}

#[test]
fn test_generic_struct() {
    let integer = Point::<i32> { x: 5, y: 10 };
    let float = Point::<f64> { x: 1.2, y: 2.0 };

   println!("{} {}", integer.x, integer.y);
   println!("{} {}", float.x, float.y);
}

enum Value<T> {
   NONE,
   VALUE(T),
}

#[test]
fn test_generic_enum() {
    let _none = Value::<i32>::NONE;
    let value = Value::<i32>::VALUE(10);

    match value {
        Value::NONE => {
         println!("none")
      }
        Value::VALUE(value) => {
         println!("value {}", value)
      }
    }
}

struct Hi<T = SimplePerson> where T: CanSayGoodBye {
   value: T,
}

#[test]
fn test_generic_struct_with_trait() {
    let hi = Hi::<SimplePerson> {
      value: SimplePerson {
         name: String::from("dihas")
      }
    };
    println!("{}", hi.value.name);
}

fn min<T>(value1: T, value2: T) -> T where T: PartialOrd {
   if value1 < value2 {
      value1
   } else {
      value2
   }
}

#[test]
fn generic_in_function() {
    let result = min::<i32>(10,20);
    println!("{}", result);

    let result = min(50,30);
    println!("{}", result);
}

#[test]
fn test_generic_method() {
    let point = Point{x: 10, y:20};
    println!("{} {} {}", point.get_x(), point.get_y(), point.get_value());
}

trait GetValue<T> where T: PartialOrd {
    fn get_value(&self) -> &T;
}

impl<T>  GetValue<T> for Point<T> where T: PartialOrd {
    fn get_value(&self) -> &T {
        &self.x
    }
}

// Overloadable operator
use core::ops::Add;

struct Apple {
   quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
      Apple {
         quantity: self.quantity + rhs.quantity
      }
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 20};

    let apple3 = apple1 + apple2;
    println!("{}", apple3.quantity);
}

// Optional values
fn double(value: Option<i32>) -> Option<i32> {
   match value {
      None => None,
      Some(i) => Some(i *2),
   }
}

#[test]
fn test_option() {
   let result = double(Some(10));
   println!("{:?}", result);

   let result = double(None);
   println!("{:?}", result);
}

// Comparing
impl PartialEq for Apple {
   fn eq(&self, other: &Self) -> bool {
      self.quantity == other.quantity
   }
}

impl PartialOrd for Apple {
   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
       self.quantity.partial_cmp(&other.quantity)
   }
}

#[test]
fn test_compare() {
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 20};

    println!("Apple1 == Apple2: {}", apple1 == apple2);
    println!("Apple1 > Apple2: {}", apple1 > apple2);
    println!("Apple1 <= Apple2: {}", apple1 <= apple2);
}

// String manipulation
#[test]
fn test_string_manipulation() {
    let s = String::from("dihas ananda");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());
    println!("{}", s.len());
    println!("{}", s.replace("dihas", "Dihas"));
    println!("{}", s.contains("dihas"));
    println!("{}", s.starts_with("dihas"));
    println!("{}", s.ends_with("ananda"));
    println!("{}", s.trim());
    println!("{}", &s[..5]);
    println!("{:?}", s.get(..5));
}

// Formatting
struct Category {
   id: String,
   name: String,
}

use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::VecDeque, fmt::Debug};

impl Debug for Category {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       f.debug_struct("Category")
         .field("id", &self.id)
         .field("name", &self.name)
         .finish()
   }
}

#[test]
fn test_format() {
   let category = Category {
      name: String::from("gadget"),
      id: String::from("GADGED"),
   };

   println!("{:?}", category);
}

// Closure
#[test]
fn test_closure() {
    let sum = |value1: i32, value2: i32| -> i32 {
      value1 + value2
    };

    let result = sum(1, 2);
    println!("Result: {}", result);
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
   let result = filter(value);
   println!("result: {}", result);
}

#[test]
fn test_closure_as_parameter() {
    print_with_filter(String::from("dihas"), |value: String| -> String {
      value.to_uppercase()
    }); //anonymous function
}

fn to_uppercase(value: String) -> String {
   value.to_uppercase()
}
#[test]
fn test_closure_as_closure() {
   print_with_filter(String::from("dihas"), to_uppercase);
}

#[test]
fn test_closure_scope() {
   let mut counter = 0;

   let mut increment = || {
      counter += 1;
      println!("increment");
   };

   increment();
   increment();
   increment();

   println!("Counter: {}", counter);
}

struct Counter {
   counter: i32,
}

impl Counter {
    fn increment(&mut self) {
      self.counter += 1;
      println!("Increment");
    }
}

#[test]
fn test_counter() {
    let mut counter = Counter{counter: 0};
    counter.increment();
    counter.increment();
    counter.increment();

    println!("Counter {}", counter.counter);
}

// Collection
/*
collection di rust ada 3
1. sequence: memiliki index
2. map: key-value
3. sets: data unik
 */

// Sequence

//vector
// sesuai dengan yang diingkan urutannya
// cocok sebagai stack (tumpukan), last in first out
#[test]
fn test_vector() {
    let array = ["dihas", "ananda"];

    for value in array {
      println!("{}", value);
    }

    let mut names = Vec::<String>::new();
    names.push(String::from("dihas"));
    names.push(String::from("ananda"));

    for name in &names {
      println!("{}", name);
    }

    println!("{:?}", names);
    println!("{}", names[0]);
}

//VecDeque
// menambah data di depan (head) dan belakang (end)
// cocok sebagai Queue (Antrian), first in first out


#[test]
fn test_vec_deque() {
    let mut names = VecDeque::new();
    names.push_back(String::from("dihas"));
    names.push_back(String::from("ananda"));
    names.push_front(String::from("ananda"));

    for name in &names {
      println!("{}", name)
   }
   
   println!("{:?}", names);
   println!("{}", names[0]);
}

//linkedList
// lebih efisien
use std::collections::{BTreeMap, BTreeSet, HashSet, LinkedList};

#[test]
fn test_linked_list() {
    let mut names = LinkedList::new();
    names.push_back(String::from("dihas"));
    names.push_back(String::from("ananda"));
    names.push_front(String::from("ananda"));

    for name in &names {
      println!("{}", name)
   }
   
   println!("{:?}", names);
   // println!("{}", names[0]);
}

// Maps
/*
1. HashMap : tidak diurutkan, lebih cepat
2. BTreeMap : diurutkan , balance tree map
*/

// HashMap
use std::collections::HashMap;

#[test]
fn test_hash_map() {
    let mut map = HashMap::new();
    map.insert(String::from("name"), String::from("dihas"));
    map.insert(String::from("age"), String::from("26"));
    map.insert(String::from("country"), String::from("Indonesia"));

    let name = map.get("name");
    let age = map.get("age");

    println!("Name {}, age {}", name.unwrap(), age.unwrap());
    
    for entry in map {
      println!("{}: {}", entry.0, entry.1)
    }
}

//BTreeMap 
#[test]
fn test_btree_map() {
    let mut map = BTreeMap::new();
    map.insert(String::from("name"), String::from("dihas"));
    map.insert(String::from("age"), String::from("26"));
    map.insert(String::from("country"), String::from("Indonesia"));

    for entry in map {
      println!("{}: {}", entry.0, entry.1)
    }
}

// Sets
/*
1. HashSet: tidak diurutkan, lebih cepat
2. BTreeSet: diurutkan
*/

// HashSet
#[test]
fn test_hash_set() {
    let mut set = HashSet::new();
    set.insert(String::from("dihas"));
    set.insert(String::from("dihas"));
    set.insert(String::from("ananda"));
    set.insert(String::from("ananda"));
    set.insert(String::from("dihas"));

    for value in set {
      println!("{}", value);
    }
}

// BTreeSet
#[test]
fn test_btree_set() {
    let mut set = BTreeSet::new();
    set.insert(String::from("dihas"));
    set.insert(String::from("dihas"));
    set.insert(String::from("ananda"));
    set.insert(String::from("ananda"));
    set.insert(String::from("dihas"));

    for value in set {
      println!("{}", value);
    }
}

// Iterator
#[test]
fn test_iterator() {
    let array = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next() {
      println!("{}", value);
    }

    for value in iterator {
      println!("{}", value);
    }
}

#[test]
fn test_iterator_method() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Vector: {:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("Sum: {}", sum);

    let count = vector.iter().count();
    println!("Count: {}", count);

    let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("Odd: {:?}", odd);
}

// Error Handling

// Unrecoverable Error
// panic!
fn connect_database(host: Option<String>) {
   match host {
      None => {
         panic!("No database host provided");
      }
      Some(host) => {
         println!("Connecting to database {}", host);
      }
   }
}

#[test]
fn test_panic() {
    connect_database(Some(String::from("localhost")));
   //  connect_database(None); // error
}

// Recoverable Error
// Enum Result
// OK(T) Err(E)

fn connect_cache(host: Option<String>) -> Result<String, String> {
   match host {
      None => Err("No cache host provided".to_string()),
      Some(host) => Ok(host)
   }
}

#[test]
fn test_recoverable_error() {
   //  let cache = connect_cache(Some("localhost".to_string()));
   let cache = connect_cache(None);
   
   match cache {
      Ok(host) => {
         println!("Success connect to host : {}", host)
      }
      Err(error) => {
         println!("Error with message : {}", error)
      }
   }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
   match host {
      None => Err("No email host provided".to_string()),
      Some(host) => Ok(host)
   }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
   //  let cache_result = connect_cache(host.clone());
   //  match cache_result {
   //      Ok(_) => {}
   //      Err(err) => {
   //       return Err(err);
   //      }
   //  }

   //  let email_result = connect_email(host.clone);
   //  match email_result {
   //      Ok(_) => {}
   //      Err(err) => {
   //       return Err(err);
   //      }
   //  }
   connect_cache(host.clone())?;
   connect_email(host.clone())?;
    Ok("Connected to application".to_string())
}

#[test]
fn test_connect_app() {
   //  let result = connect_application(Some("localhost".to_string()));
    let result = connect_application(None);
    match result {
        Ok(msg) => {
         println!("Success connect with message : {}", msg)
        }
        Err(err) => {
         println!("Error connecting to application: {}", err)
        }
    }
}

//lifetime

#[test]
fn test_dangling_reference() {
    let r: &i32;
    {
      let _x = 5;
      // r = &x; error
    }

    r = &40;
    println!("r: {}", r);
}

// lifetime annotation syntax
fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
   if value1.len() > value2.len() {
      value1
   } else {
      value2
   }
}

#[test]
fn test_lifetime_annotation() {
    let value1 = "dihas";
    let value2 = "ananda";
    let result = longest(value1, value2);
    println!("result: {}", result);
}

#[test]
fn test_lifetime_annotation_dangling_reference() {
    let string1 = String::from("ananda");
    let string2 = String::from("dihas");
    let result;
    {
      result = longest(string1.as_str(), string2.as_str())
    }
    println!("result: {}", result);
}

// lifetime annotation di struct

struct Student<'a, 'b> {
   name: &'a str,
   last_name: &'b str,
}

impl<'a, 'b> Student<'a, 'b> {
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
      if self.name.len() > student.name.len() {
         self.name
      } else {
          student.name
      }
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
   if student1.name.len() > student2.name.len() {
       student1.name
   } else {
      student2.name
   }
}

#[test]
fn test_student() {
    let student = Student{
      name: "dihas",
      last_name: "ananda",
    };
    println!("{} {}", student.name, student.last_name);

    let student2 = Student {
      name: "ananda",
      last_name: "dihas"
    };
    let result = longest_student_name(&student, &student2);
    println!("{}", result);

    let result = student.longest_name(&student2);
    println!("{}", result);
    
}

struct Teacher<'a, ID> where ID: Ord {
   id: ID,
   name:  &'a str,
}

#[test]
fn test_lifetime_annotation_generic() {
    let teacher = Teacher {
      id: 10,
      name: "dihas",
    };
    println!("{} {}", teacher.id, teacher.name)
}

// Attribute
// #[namaAtribute]
// seperti decorator dalam ts

// derive attribute

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
   name: String,
   location: String,
   website: String,
}

#[test]
fn test_attribute_debug() {
    let company = Company {
      name: "dihas ananda".to_string(),
      location: "indonesia".to_string(),
      website: "dihasananda.com".to_string(),
    };
    let company2 = Company {
      name: "dihas ananda".to_string(),
      location: "indonesia".to_string(),
      website: "dihasananda.com".to_string(),
    };

    println!("{:?}", company);

    let result = company == company2;
    println!("{}", result);

    let result = company > company2;
    println!("{}", result);
}

// Smart Pointer

// Box<T> 

#[test]
fn test_box() {
    let value = Box::new(10);
    println!("value: {}", value);
    display_number(*value);
    display_number_reference(&value);
}

fn display_number(value: i32) {
    println!("display number: {}", value);
}

fn display_number_reference(value: &i32) {
    println!("display number reference: {}", value);
}

//recursive data type

#[derive(Debug)]
enum ProductCategory {
   Of(String, Box<ProductCategory>),
   End
}

#[test]
fn test_box_enum() {
    let category = ProductCategory::Of(
      "laptop".to_string(),
      Box::new(ProductCategory::Of(
         "Dell".to_string(), 
         Box::new(ProductCategory::End)
      )),
   );
   println!("{:?}", category);
   print_category(&category)
}

fn print_category(category: &ProductCategory) {
   println!("{:?}", category);
}

// Dereference

#[test]
fn test_dereferene() {
    let value1 = Box::new(10);
    let value2 = Box::new(20);

    let result = *value1 * *value2;
    println!("{}", result);
}

// deref trait

struct MyValue<T> {
   value: T,
}

use std::ops::Deref;

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
      &self.value
    }
}

#[test]
fn test_dereference_struct() {
    let value = MyValue { value: 10 };
    let real_value = *value;
    println!("value: {}", real_value)
}

// deref untuk parameter

fn say_hello_reference(name: &String) {
   println!("Hello {}", name);
}

#[test]
fn test_deref_reference() {
    let name = MyValue{
      value: "dihas".to_string()
    };
    say_hello_reference(&name)
}

// Cleanup

//drop trait, membuat kode sebelum value di drop/hapus dari memori

struct Book {
   title: String,
}

impl Drop for Book {
   fn drop(&mut self) {
      println!("Dropping Book {}", self.title)
   }
}

#[test]
fn test_drop() {
    let book = Book { title: "rust".to_string() };
    println!("{}", book.title);
}

// Multiple Ownership

//Rc<T> reference counted
// merupakan smart pointer, namun bisa beberapa owner

enum Brand {
    Of(String, Rc<Brand>),
    End
}

#[test]
fn test_multiple_ownership_box() {
   let apple = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
   println!("Apple reference count: {}", Rc::strong_count(&apple));
   
   let laptop = Rc::new(Brand::Of("laptop".to_string(), Rc::clone(&apple)));
   println!("Apple reference count: {}", Rc::strong_count(&apple));

   {
      let smartphone = Rc::new(Brand::Of("smartphone".to_string(), Rc::clone(&apple)));
      println!("Apple reference count: {}", Rc::strong_count(&apple));
   }
   
   println!("Apple reference count: {}", Rc::strong_count(&apple));

   //  let apple = ProductCategory::Of("apple".to_string(), Box::new(ProductCategory::End));
   //  let laptop = ProductCategory::Of("laptop".to_string(), Box::new(apple));
   //  let smartphone = ProductCategory::Of("smartphone".to_string(), Box::new(apple));
}

// Interior Mutability

// memperbolehkan mengubah data walau reference imutable
// RefCell<T>, single ownership

#[derive(Debug)]
struct Seller {
   name: RefCell<String>,
   active: RefCell<bool>,
}

#[test]
fn test_ref_cell() {
    let seller = Seller {
      name: RefCell::new("dihas".to_string()),
      active: RefCell::new(true),
    };

    {
    let mut result = seller.name.borrow_mut();
    *result = "ananda".to_string();
    }
    println!("{:?}", seller);
}

// Static

static APPLICATION: &str = "My application";

#[test]
fn test_static() {
    println!("{}", APPLICATION);
}

//mutable static
// wajib menggunakan unsafe block

static mut COUNTER: u32 =0;

unsafe fn increment() {
   COUNTER += 1;
}

#[test]
fn test_static_mut() {
    unsafe {
      increment();
      COUNTER += 1;
      println!("COUNTER: {}", COUNTER);
    }
}

// Macro

// kode untuk membuat kode lainnya
// metaprogramming
// implementasi macro lebih kompleks daripada function biasa

//declarative macro

macro_rules! hi {
   () => {
      println!("Hi macro!");
   };
   ($name: expr) => {
      println!("Hi, {}!", $name);
   };
}

#[test]
fn test_macro() {
    hi!();
    hi!("dihas");
    hi! {
      "ananda"
    };
    let name = "dihasanada";
    hi!(name);
}

macro_rules! iterate {
   ($array: expr) => {
      for i in $array {
         println!("{}", i);
      }
   };
   ($($item: expr), *) => {
      $(
         println!("{}", $item);
      )*
   }
}

#[test]
fn test_macro_iterate() {
    iterate!([1,2,3,4,5,6,7,8,9]);
    iterate!(1,2,3,4,5,6,7,8,9);
}

// Macro

// kode untuk membuat kode lainnya
// metaprogramming
// implementasi macro lebih kompleks daripada function biasa

//declarative macro

macro_rules! hi {
   () => {
      println!("Hi macro!");
   };
   ($name: expr) => {
      println!("Hi, {}!", $name);
   };
}

#[test]
fn test_macro() {
    hi!();
    hi!("dihas");
    hi! {
      "ananda"
    };
    let name = "dihasanada";
    hi!(name);
}

macro_rules! iterate {
   ($array: expr) => {
      for i in $array {
         println!("{}", i);
      }
   };
   ($($item: expr), *) => {
      $(
         println!("{}", $item);
      )*
   }
}

#[test]
fn test_macro_iterate() {
    iterate!([1,2,3,4,5,6,7,8,9]);
    iterate!(1,2,3,4,5,6,7,8,9);
}