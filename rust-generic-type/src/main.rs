fn main() {
    println!("^_^ Rust Generic Type ^_^");

    // *** Define Generic type
    /*
     * Generic type là kiểu dữ liệu chung (placeholder) có thể thay thế cho các kiểu dữ liệu Rust
     * Sử dụng kí tự in hoa
     * Dynamic
     */

    // T trong trường hợp này có giá trị 10 và có kiểu dữ liệu là i32
    let _x = Some(10);

    // T trong trường hợp này có giá trị 10.0 và có kiểu dữ liệu là f64
    let _y = Some(10.0);

    let _x1 = OptionI32::Some(10); //  Sử dụng kiểu dữ liệu cụ thể
    let _y1 = OptionF64::Some(10.0); // Sử dụng kiểu dữ liệu cụ thể
}

pub enum OptionF64 {
    None,
    Some(f64),
}

pub enum OptionI32 {
    None,
    Some(i32),
}
