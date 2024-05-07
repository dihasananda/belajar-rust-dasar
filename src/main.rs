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