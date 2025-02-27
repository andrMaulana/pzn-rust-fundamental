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
