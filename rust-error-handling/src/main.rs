use std::{
    fs::File,
    io::{Error, Read},
};
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
    let name_st_expect = get_user("JKay").expect("Mong chờ input"); //JKay
    println!("Name student: {:?}", name_st_expect);

    // *** Custom Error
    /*
     * Question Mark(?)
     * - Toán tử "?" dùng để unwrap cho Result
     * - Nếu có giá trị -> unwrap và trả về giá trị
     * - Nếu có lỗi -> trả về lỗi ngay lập tức
     * - Giống với unwrap() và expect() method
     */

    let file_path = "data.txt"; // data.txt
    match read_file_contents(file_path) {
        Ok(contents) => println!(" File contents: {}", contents),
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    let content = read_file_contents_error_custom(file_path);
    println!("Content after error custom: {:?}", content);
}

fn get_user(username: &str) -> Option<&str> {
    if username.is_empty() {
        return None;
    }
    return Some(username);
}

fn read_file_contents(path: &str) -> Result<String, Error> {
    // Sử dụng toán tử ? để unwrap lấy giá trị
    // Vì hàm open trả về Result
    let mut file = File::open(path)?;
    let mut contents = String::new();

    //Sử dụng toán tử ? để unwrap lấy giá trị
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn read_file_contents_error_custom(path: &str) -> Result<String, CustomError> {
    // Sử dụng toán tử ? để unwrap lấy giá trị
    // Vì hàm open trả về Result
    let mut file = File::open(path).map_err(|_| CustomError::FileOpenError)?;
    let mut contents = String::new();

    //Sử dụng toán tử ? để unwrap lấy giá trị
    file.read_to_string(&mut contents)
        .map_err(|_| CustomError::FileReadError)?;

    Ok(contents)
}

#[derive(Debug)]
enum CustomError {
    FileOpenError,
    FileReadError,
}
