use std::io;

fn main() {
    println!("Hello, world!");

    println!("halo");

    println!("Hello nama saya khairul aswad");

    say_hello_to_number(30);

    print_labeled_measurement(40, 'N');

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}

#[test]
fn hello_test() {
    println!("Hello test");
}

#[test]
fn test_variable() {
    let name = "khairul aswad";

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

    println!("hello {}", name)
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
    let umur_pengguna: u8 = 20;

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
    let d = c / b;

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
#[test]
fn arrays_type() {
    let names = ["khairul", "aswad", "budi", "eko"];

    println!("{}", names[0]);
}

#[test]
fn augmented_assignment() {
    let mut a: i32 = 10;

    println!("{}", a);

    a += 10;
    println!("{}", a);

    a *= 5;
    println!("{}", a);
}

#[test]
fn comparison_operators() {
    let result: bool = 10 > 20;

    assert_eq!(result, false, "result {}", result)
}

#[test]
fn boolean_operator() {
    let absen: i8 = 75;
    let nilai_akhir: i8 = 90;

    let lulus: bool = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 80;

    let lulus_final: bool = lulus && lulus_nilai_akhir;

    println!(
        "lulus nilai akhir true=lulus, false=tidak : {}",
        lulus_final
    )
}

#[test]
fn loop_iterator() {
    let mut status_nikah: bool = true;

    if status_nikah != false {
        status_nikah = false;
    }

    println!("status nikah {}", status_nikah)
}

#[test]
fn arrays_but_adding_value() {
    let mut a = [1, 2, 3, 4];

    a[0] = 10;

    let b: [i32; 2] = [1, 2];

    println!("this is b first example {}", b[1]);

    println!("{}", a[0]);
}

#[test]
fn function_represenst() {
    println!("Hello, world!");

    say_hello_to_number(30);
    print_labeled_measurement(40, 'H');
}

fn say_hello_to_number(x: i32) {
    println!("Hello number: {}", x)
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the number of: {} labeled by: {}", value, unit_label)
}

#[test]
fn function_statement_and_expressions() {
    //this is statemnt
    let t = 7;

    //Calling a macro is expression
    println!("{t}");

    //and this is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("this is expression of new line: {}", y);
}
#[test]
fn function_return_value() {
    let value = five();

    println!("data of variable value is: {value}");

    let x = plus_one(5);

    println!("the value of x variable is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

#[test]
fn control_flow() {
    let nilai_ujian = 90;
    let batas_nilai_ujian = 75;

    if nilai_ujian >= batas_nilai_ujian {
        println!("Kamu lulus!");
    } else {
        println!("Kamu gagal");
    }

    if nilai_ujian >= 90 {
        println!("Kamu masuk grade A");
    } else if nilai_ujian <= 90 && nilai_ujian >= 80 {
        println!("Kamu masuk grade B");
    } else {
        println!("Kamu grade C")
    }
}

#[test]
fn ganjil_genap_cek() {
    let value = 40;

    if value % 2 == 0 {
        println!("Angka {value} adalah genap");
    } else {
        println!("Angka {value} adalah ganjil");
    }
}

#[test]
fn using_if_on_let() {
    let condition = false;

    let number = if condition { 6 } else { 7 };

    // incompatible, value must be one if type
    // let number = if condition {6} else {"seven"};

    println!("the value of number is:{number}");
}

#[test]
fn repetitio_using_loops() {
    //this code will print Again until we stop the program
    //this not will print on test mode but in main function its actually running

    // loop {
    //     println!("Again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter;
        }
    };

    println!("value of counter is {result}")
}

#[test]
fn loop_labels_to_disambigu() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining {remaining}");

            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

#[test]
fn while_loop_in_rust() {
    let mut number = 5;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

#[test]
fn loop_arrays_using_while() {
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < a.len() {
        println!("The value of a is:{}", a[index]);

        index += 1;
    }

    println!("Thats All")
}

#[test]
fn loop_arrays_using_for() {
    let arrays: [i8; 5] = [10, 20, 30, 40, 50];

    for element in arrays {
        println!("this is value of arrays: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LiftOff");
}
#[test]
fn another_looping_expreience() {
    let mut nano_prison = String::new();

    io::stdin()
        .read_line(&mut nano_prison)
        .expect("cannot read line");

    //using shadowing variable this method only use when you need convert type of variabel
    let nano_prison: String = nano_prison.trim().parse().expect("Only string");

    println!(" this is taking gesture: {nano_prison}")
}

#[test]
fn looping_using_for() {
    for number in 1..100 {
        println!("{number}");
    }
}

#[test]
fn debugging_mode() {
    let pablo_escobar_mode: (i32, bool, String) = (10, true, String::from("halo dek"));

    let a = pablo_escobar_mode;

    println!("{:?}", a);

    println!("")
}
