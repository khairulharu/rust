fn main() {
    println!("Hello, world!");

    println!("halo");
    
    println!("Hello nama saya khairul aswad");

}

#[test]
fn hello_test() {
    println!("Hello test");
}

#[test]
fn test_variable() {

    let name= "khairul aswad";

    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Alok Gaming salto";

    println!("Hello kamu siapa {}", name);

    name = "Ubah nama si alok itu";

    println!("Hello kamu siaapa lagi {}", name)
}

#[test]
fn static_typing() {
    let name = "halo dek";

    // name = 10;

    println!("hello {}",name)
}

#[test]
fn shadowing() {
    let name = "khairul Aswad";

    println!("Hello {}", name);

    let name = 10;

    println!("Hello {}", name);
}

#[test]
fn comment() {
    println!("Hello");
    //single comment 

    /*

    comment on besides one line
    
    */
}

#[test]
fn explicit() {
    let umur_pengguna:u8 = 20;

    println!("Umur Saya : {}", umur_pengguna)
}

#[test]
fn number() {
    let a = 10;
    let b = 10.5;

    println!("{}", a);

    println!("{}", b);
}

#[test]
fn conversion_number() {
    let enambelas_bit: i16 = 256;
    println!("{}", enambelas_bit);

    let tigadua_bit: i32 = enambelas_bit as i32;
    println!("{}", tigadua_bit);

    let delapan_bit: i8 = tigadua_bit as i8;
    println!("{}", delapan_bit);
}

#[test]
fn numerical_operator() {
    let a = 10;
    let b = 20;

    let c = a * b;
    let e = a % b;
    let d  = c / b;

    let remainder = 2 % 4;
    println!("Hasil dari modulus tersebut ialah: {remainder}"); 
    println!("{}", c);
    println!("{}", e);
    println!("{}", d);
}
#[test]
fn boolean_type() {

    let t = true;

    let f: bool = false;

    println!("{t}");
    println!("{f}");
}

#[test]
fn char_type() {

    let t = 's';

    let f: char = 'B';

    println!("this is char of : {t}");
    println!("this is char of : {f}");
}

#[test]
fn tuple_type() {
    let tup: (&str, bool, i32) = ("aswad", false, 20);

    //destructuring type of tuple
    let (name, status, age) = tup;

    println!("name: {}", tup.0);
    println!("status: {}", tup.1);
    println!("age: {}", tup.2);
    println!("{name}");
    println!("{}", status);
    println!("{}", age)
}