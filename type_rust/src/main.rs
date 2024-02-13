fn main() {
    println!("Learn Type in Rust");

    /* VARIABLES: BIẾN */
    // Định nghĩa biến x = 5
    // Nhưng lúc này biến x đang là bất biến (immutable) và sẽ không thay đổi được giá trị của x
    let x = 5;
    // x = x + 1; => error[E0384]: cannot assign twice to immutable variable `x`
    println!("{}", x);

    // Khắc phục: Muốn thay đổi giá trị của 1 biến "immutable" --> ta phải thêm "mut" vào trước tên biến
    let mut y = 5;
    y = y + 1;
    println!("x: {} so với y: {}", x, y);
    println!("{x} < {y}");

    /* DATA TYPES: KIỂU DỮ LIỆU
       Có 2 kiểu dữ liệu
        * Scalar: lưu trữ đơn giá trị như Integer, Float, Char, Boolean, String
        * Compound: lưu trừ đa giá trị như Array, Tuple
    */
    // *** Scalar
    let test_string = "Rust learn";
    let test_boolean = true;
    let test_integer = 2024;
    let test_char = 'b';
    println!("{test_string}, {test_boolean}, {test_integer}, {test_char}");
    // *** Hằng số: const
    const PI: f32 = 3.14;
    println!("{PI}");
    // *** Compound
    // * Array: Tất cả các phần từ phải cùng kiểu dữ liệu
    let test_array = [1, 2, 3, 4];
    println!("{}", test_array[2]);
}
