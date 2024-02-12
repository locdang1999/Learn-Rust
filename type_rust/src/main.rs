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

    /* DATA TYPES: KIỂU DỮ LIỆU */
}
