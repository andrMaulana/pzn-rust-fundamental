use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hallo, World!");
}

#[test]
fn variable() {
    let nama = "Andri Maulana";
    println!("Hallo {} ", nama);
}

#[test]
fn mutable() {
    let nama = "andri";
    println!("hallo {}", nama);

    let nama = 4122;
    println!("mutable => '{}'", nama);
}

#[test]
fn test_static() {
    // let mut name = "andri";
    // println!("{}", name);

    // name = 12;
    // println!("{}", name);
}

#[test]
fn komentar() {
    // ini adalah komentar satu baris
    println!("Hello");
    /*
     *   ini adalah komentar lebih dari satu baris
     *   ini adalah komentar lebih dari satu baris
     *   ini adalah komentar lebih dari satu baris
     */
}

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("{}", age);
}

#[test]
fn test_number() {
    let a: i32 = 10;
    println!("integer: {}", a);

    let b: f32 = 10.1;
    println!("float: {}", b);
}

#[test]
fn number_convertion() {
    let a: i8 = 10;
    println!("a: {}", a);

    let b: i16 = a as i16;
    println!("convertion a to b: {}", b);

    let c: i32 = a as i32;
    println!("convertion a to c: {}", c);
}

#[test]
fn numeric_operations() {
    // penjumlahan
    let penjumlahan = 10 + 10;
    println!("penjumlahan 10 + 10 = {}", penjumlahan);

    // pengurangan
    let pengurangan = 10 - 5;
    println!("pengurangan 10 + 5 = {}", pengurangan);

    // perkalian
    let perkalian = 10 * 2;
    println!("perkalian 10 * 2 = {}", perkalian);

    // pembagian
    let pembagian = 10 / 2;
    println!("pembagian 10 / 2 = {}", pembagian);

    // modulos atau sisa bagi
    let modulos = 10 % 2;
    println!("Modulos 10%2 = {}", modulos);
}

#[test]
fn augmented_assignments() {
    let mut a: i32 = 10;
    println!("Nilai awal : {}", a);

    a += 10;
    println!("operasi += : {}", a);
    a -= 10;
    println!("operasi -= : {}", a);
    a *= 10;
    println!("operasi *= : {}", a);
    a /= 10;
    println!("operasi /= : {}", a);
    a %= 10;
    println!("operasi %= : {}", a);
}

#[test]
fn multiple_variable() {
    let (var1, var2, var3) = (1, "hallo", true);
    println!("multiple var: {1} {0} {2}", var1, var2, var3);
}

#[test]
fn boolean() {
    let a = false;
    let b: bool = true;

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let a: i8 = 10;
    let b: i8 = 20;

    let c: bool = a <= b;
    println!("{}", c);
}

#[test]
fn integermax() {
    let a = i128::MAX;
    println!("{}", a);
}

#[test]
fn boolean_operation() {
    let absen: i8 = 75;
    let nilai: i8 = 80;

    let lulus_absen: bool = absen >= 75;
    let lulus_nilai: bool = nilai >= 75;

    let kelulusan: bool = lulus_absen && lulus_nilai;
    println!("{}", kelulusan);
}

#[test]
fn char() {
    let a: char = 'a';
    let b: char = 'b';

    println!("{} {}", a, b);
}

#[test]
fn test_tuple() {
    let data: (&str, i8, bool) = ("Andri", 25, true);
    println!("data => {:?}", data);
}

#[test]
fn access_data_tuple() {
    let datas: (i32, f64, bool) = (10, 10.5, false);
    println!("data: {:?}", datas);

    let a = datas.0;
    let b = datas.1;
    let c = datas.2;
    println!("data 1: {}, data 2: {}, data 3: {}", a, b, c);
}

#[test]
fn destructuring_tuple_test() {
    let datas: (&str, i32, f64) = ("Andri Maulana", 24, 170.2);
    println!("{:?}", datas);

    let (data1, data2, _) = datas;
    println!("data1: {} | data2: {} ", data1, data2);
}

#[test]
fn mutable_tuple() {
    let mut datas: (&str, i32, f64) = ("Andri Maulana", 25, 170.2);
    println!("before mut => {:?}", datas);

    datas.0 = "Andri";
    datas.1 = 24;
    datas.2 = 172.2;
    println!("after mut => {:?}", datas);
}

#[test]
fn void_tuple() {
    let res: () = ();
    println!("{:?}", res);
}

#[test]
fn test_array() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
}

#[test]
fn access_arr() {
    let arr: [i32; 2] = [1, 2];
    println!("{:?}", arr);
    println!("{} {}", arr[0], arr[1]);
}

#[test]
fn mutable_arr() {
    let mut arr: [&str; 3] = ["Andri", "Eko", "Asep"];
    println!("{:?}", arr);

    arr[1] = "Eko Kurniawan";
    arr[2] = "Asep Cobra";
    println!("{:?}", arr);
}

#[test]
fn length_arr() {
    let names: [&str; 4] = ["Budi", "Andri", "Eko", "Ayu"];
    println!("{:?}", names);
    println!("Panjang Arr: {}", names.len());
}

#[test]
fn array_two_dimensional() {
    let matrix: [[i32; 2]; 2] = [[1, 2], [3, 4]];

    println!("max matrix: {:?}", matrix);
    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}

#[test]
fn test_constant() {
    const MINIMUM: i32 = 100;
    println!("{}", MINIMUM);
}

#[test]
fn variable_scope() {
    let andri: &str = "Andri";

    {
        // inner scope
        println!("Hallo {}", andri);
        let maulana: &str = "Maulana";
        println!("Hallo {}", maulana);
    }
    println!("Hallo {}", andri);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Andri");
    println!("{}, {}", a, b);
}

fn function_b() {
    let a = 10;
    let b = String::from("Maulana");
    println!("{}, {}", a, b);
}

#[test]
fn str_type() {
    let name: &str = "  Andri  ";
    println!("{}", name);
    let trims: &str = name.trim();
    println!("{}", trims);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Andri");
    println!("{}", name);
    name.push_str(" Maulana");
    println!("{}", name);

    let replaces: String = name.replace("Andri", "Budi");
    println!("{}", replaces);
}

#[test]
fn ownership_rules() {
    // variable a tidak bisa diakses disini karena belum di deklarasi
    let a: i32 = 10; // variable a bisa diakses mulai dari sini

    {
        // variable b tidak bisa diakses disini karena belum di deklarasi
        let b: i32 = 20; // variable b bisa diakses mulai dari sini
        println!("{}", b);
    } // variable b sudah selesai dieksekusi dan keluar dari scope nya maka akan dihapus di memory
      // dan sudah tidak bisa diakses lagi
    println!("{}", a);
} // variable a sudah selesai dieksekusi dan keluar dari scope nya maka akan dihapus di memory dan
  // sudah tidak bisa diakses lagi

#[test]
fn data_copy() {
    let a: i32 = 10;
    let b: i32 = a; // copy data dari variable a ke varible b
    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1: String = String::from("Andri Maulana");

    // Ownership dari name1 di pindahkan ke ownership baru yaitu name2
    let name2: String = name1;
    // name1 jadi tidak akan valid lagi disini karena ownership nya sudah di pindahkan
    println!("{}", name2);
}

#[test]
fn clone_test() {
    let name1: String = String::from("Andri Maulana");
    let name2 = name1.clone();
    println!("name1 = {}\nname2 = {}", name1, name2);
}

// If Expression
#[test]
fn if_expression() {
    let value = 9;
    if value >= 8 {
        println!("Good");
    }
}

// Else Expression
#[test]
fn else_expression() {
    let value: i32 = 9;
    if value >= 10 {
        println!("Good Boy");
    } else {
        println!("Not Good Boy");
    }
}

// Else If Expression
#[test]
fn else_if_expression() {
    let value: i32 = 2;
    if value >= 8 {
        println!("Good");
    } else if value >= 6 {
        println!("Not Bad");
    } else if value >= 3 {
        println!("Bad");
    } else {
        println!("Very Bad");
    }
}

// Let Statement
#[test]
fn let_statement() {
    let value: i32 = 9;
    let result = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Bad"
    } else if value >= 3 {
        "bad"
    } else {
        "Very Bad"
    };

    println!("{}", result);
}

// While Expression
#[test]
fn test_while() {
    let mut i = 0;
    let max = 5;

    while i < max {
        println!("Nilai i : {i}");
        i += 1;
    }
}

// Nested while
#[test]
fn test_nested_while() {
    let mut i = 0;
    let max = 5;

    while i < max {
        let mut j = 0;
        let inner_j = i;
        while j <= inner_j {
            print!("* ");
            j += 1;
        }

        println!();
        i += 1;
    }
}

// while Duration
#[test]
fn test_while_duration() {
    let mut i = 0;
    let max = 5;
    while i < max {
        println!("Nilai i: {i}");
        i += 1;
        sleep(Duration::from_secs(1));
    }
}

// Loop Expression
#[test]
fn loop_expression() {
    let mut number = 0;
    loop {
        number += 1;
        if number >= 10 {
            break;
        }

        if number % 2 == 1 {
            continue;
        }

        println!("{}", number);
    }
}

// Loop Return Value
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

// Loop label
#[test]
fn loop_label() {
    let mut number = 1;
    'andri: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'andri;
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

// Nested Loop *
#[test]
fn asterisk_nested_loop() {
    let mut i = 0;
    let max = 5;
    loop {
        let mut j = max;
        let inner_j = i;
        loop {
            print!("* ");
            j -= 1;
            if j < inner_j {
                break;
            }
        }
        println!();
        i += 1;
        if i > max {
            break;
        }
    }
}

// persiapan materi baru
#[test]
fn array_itteration() {
    let arr: [&str; 5] = ["andri", "budi", "malik", "udin", "reza"];
    let mut index = 0;

    while index < arr.len() {
        println!("Value {}", arr[index]);
        index += 1;
    }
}

#[test]
fn for_each() {
    let arr: [i32; 4] = [4, 2, 3, 1];
    for value in arr {
        println!("Datas: {}", value);
    }
}

// Range exclude
#[test]
fn range_exclude() {
    let numbers: [i32; 5] = [3, 4, 8, 1, 2];
    for idx in 1..4 {
        println!("Number: {}", numbers[idx]);
    }
}

#[test]
fn for_inclusive() {
    let names: [&str; 5] = ["andri", "ano", "jaka", "fahrudin", "rehan"];

    for val in 2..=4 {
        println!("names: {}", names[val]);
    }
}

// persipan materi function
fn say_hello() {
    println!("Hello");
}

#[test]
fn test_function() {
    say_hello();
    say_hello();
    say_hello();
}

fn say_goodbye(first_name: &str, last_name: &str) {
    println!("Goodbye👋 {first_name} {last_name}");
}

#[test]
fn test_parameter() {
    say_goodbye("Andri", "Maulana");
    say_goodbye("Joko", "Anwar");
}

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
fn return_value_test() {
    let result = factorial_loop(5);
    println!("Result: {result}");

    let result = factorial_loop(10);
    println!("Result: {result}");
}

// recursive function
fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_case_satu() {
    print_text(String::from("Andri"), 5);
}

fn factorial_recursive(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    n * factorial_recursive(n - 1)
}

#[test]
fn test_case_dua() {
    let result = factorial_recursive(5);
    println!("Result: {result}");
}
