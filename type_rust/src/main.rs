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
    let test_integer = 2024; // i32: integer 32bit --> số dương & âm
    let test_char = 'b';
    println!("{test_string}, {test_boolean}, {test_integer}, {test_char}");

    // *** Hằng số: const
    const PI: f32 = 3.14;
    println!("{PI}");

    // *** Compound
    // * Array: Tất cả các phần từ phải cùng kiểu dữ liệu
    let test_array = [1, 2, 3, 4];
    println!("{}", test_array[2]);

    // *** Type-Casting: Ép kiểu dữ liệu -> Dùng từ khóa "as" và sẽ có các trường hợp ko thể ép kiểu -> chuyển từ type này thành type khác (convert type)
    // định nghĩa kiểu dữ liệu trước tiếp cho biến
    let qt = 100u16; // u16: integer 16bit --> nhưng chỉ tính số dương
    let qw: u16 = 300;
    println!("{qt} === {qw}");
    // convert
    let change_qt = qw as u8; // u8: maxlength: 256
    println!("{qw} !== {change_qt}"); // Nếu khi convert type vượt quá giới hạn của kiểu dữ liệu thì Rust sẽ tự động lấy --> giá trị của biến ép - cho max length của kiểu dữ liệu được ép

    let chartr = 'A';
    let change_int = chartr as u8;
    println!("{chartr} vs {change_int}");

    let flg = false;
    let change_int_flg = flg as u8;
    println!("{flg} vs {change_int_flg}");

    // case ko thể convert
    let intg: u32 = 65;
    // let change_char = intg as char; // only `u8` can be cast as `char`, not `u32` invalid cast
    println!("convert: {intg}");

    // *** Operators: Toán tử
    /*
     * Hỗ trợ toán tử có bản: +, -, *, /, ... cho các phép toán số học
     * Các toán tử so sánh: ==, !=, >, <, ...
     * Các toán tử logic: &&, ||, !
     * Ngoài ra còn nhiều toán tử khác
     */
    let book_blue = 10;
    let mut book_red = 30;

    let add = book_blue + book_red;
    let sub = book_blue - book_red;
    book_red += 1;
    println!("Book Red value: {book_red}");
    println!("Add: {add}");
    println!("Sub: {sub}");
    println!("So sánh Red vs Blue: {}", book_red > book_blue);
}
