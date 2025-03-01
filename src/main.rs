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
