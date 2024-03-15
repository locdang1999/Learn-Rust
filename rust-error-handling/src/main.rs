use std::fs::File;

fn main() {
    println!("^_^ Rust Error Handling ^_^");

    /*
     * - Các loại lỗi:
     *  + Lỗi không thể phục hồi (Unrecoverable Errors)
     *  + Lỗi có thể phục hồi (Recoverable Errors)
     * - Lỗi không thể phục hồi
     *  + Lỗi này dẫn đến chương trình dừng ngay lập tức
     */

    // *** Unrecoverable Errors
    // panic!("Exit!");
    // println!("Hello World again"); // Không thể thực hiện dòng này thì dòng trên đã stop chương trình
    // let numbers = [1,2,3];
    // println!("Number at index 3: {}", numbers[3]); chương trình compiler sẽ không thông báo lỗi ngay lập tức khi chạy chương trình sẽ báo lỗi và ngừng chương trình

    /*
     * - Cách xử lý dùng
     *  + Reslt Enum: Kiểu "Result" sẽ trả về giá trị hoặc lỗi
     *   1. OK -> Thao tác thành công sẽ trả về giá trị
     *   2. Err -> Thao tác thất bại sẽ trả về lỗi
     *  + Option Enum: Kiểu "Option" trả vể None hoặc Some
     *   1. None -> Không có giá trị -> tương đương với null
     *   2. Some -> Có giá trị
     */

    // *** Recoverable Errors
    /*
     * - Lỗi này sẽ không dừng chương trình
     * - Dừng chương trình khi file ko tồn tại
     */

    let data_result = File::open("data.txt");
    // Unwrap -> lấy giá trị
    // OK: lấy nội dung file
    // Err: panic
    // match để unwrap
    let data_file = match data_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the data file: {:?}", error),
        // phục hồi khi bị lỗi không có file tồn tại
        Err(_) => match File::create("data.txt") {
            Ok(new_file) => new_file,
            Err(error) => panic!("Problem creating a new data file: {:?}", error),
        },
    };
    println!("Data file: {:?}", data_file);

    // *** Unwrap and Expect
    /*
     * wnwrap() và expect() có thể hoạt động trên kiểu Option và Result
     * Lấy giá trị bên trong Option or Result
     */

    let name_st = get_user("alice"); //alice
    println!("Name student: {:?}", name_st);
    match name_st {
        Some(value) => println!("Handle error one more: {:?}", value),
        None => panic!("Không có giá trị"),
    }

    // Unwrap
    let name_st_unwrap = get_user("Jix").unwrap(); //Jix
    println!("Name student: {:?}", name_st_unwrap);

    // Expect
    let name_st_expect = get_user("").expect("Mong chờ input"); //JKay
    println!("Name student: {:?}", name_st_expect);
}

fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }
    return Some(username);
}
