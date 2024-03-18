fn main() {
    println!("^_^ Rust Generic Type ^_^");

    // *** Define Generic type
    /*
     * Generic type là kiểu dữ liệu chung (placeholder) có thể thay thế cho các kiểu dữ liệu Rust, không biết chính xác kiểu dữ liệu gì
     * => Chạy compile or runtime thì mới biết
     * Sử dụng kí tự in hoa
     * Dynamic
     * Khi nào sử dụng Generic Type
     * - Viết code linh hoạt và làm việc với nhiều kiểu dữ liệu khác nhau
     * Hiệu suất code khi sử dụng Generic Type
     * - Không ảnh hưởng đến quá trình chạy runtime (thực thi code) so với sử dụng kiểu dữ liệu cụ thể
     * - Trong quá trình biên dịch code liên quan tới Generic thì toàn bộ biên dịch Rust sẽ thực hiện "monomorphization"
     * - Tốn thời gian, biên dịch lâu (Vì tạo ra nhiều phiên bản khác nhau mà chương trình có thể sử dụng)
     */

    // T trong trường hợp này có giá trị 10 và có kiểu dữ liệu là i32
    let _x = Some(10);

    // T trong trường hợp này có giá trị 10.0 và có kiểu dữ liệu là f64
    let _y = Some(10.0);

    let _x1 = OptionI32::Some(10); //  Sử dụng kiểu dữ liệu cụ thể
    let _y1 = OptionF64::Some(10.0); // Sử dụng kiểu dữ liệu cụ thể
    
    let _point1 = Point {
        x: 16f64,
        y: 16f64,
    };

    // Khi dùng Generic Type
    let _point2 = PointGen {
        x: 16f32,
        y: 16f32,
    };

    let _point2 = PointGens {
        x: 16f32,
        y: 16f64,
    };

    // Sử dụng Generic Type with Function
    let _res = bar(10u64);
    // Cách sử dụng khác
    let _res1 = bar::<i32>(10);
    let _res2 = bar::<f64>(10f64); // ::<>  --> gogij là turbo fish
}

pub enum OptionF64 {
    None,
    Some(f64),
}

pub enum OptionI32 {
    None,
    Some(i32),
}

// Sử dụng kiểu dữ liệu cụ thể f64
struct Point {
    x: f64,
    y: f64,
}

// Sử dụng Generic Type
pub struct  PointGen <T>{
    x:T,
    y:T
}

// Sử dụng nhiều Generic Type
pub struct  PointGens <T, U>{
    x:T,
    y:U
}

// Generic type có thể sử dụng như 1 paramater của 1 function
pub fn foo(a: u32) -> u32 {
    a
}

// Định nghĩa Generic Type in Function
pub fn bar<T>(a: T) -> T {
    a
}