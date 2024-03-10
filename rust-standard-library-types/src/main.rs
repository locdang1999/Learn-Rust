use std::collections::HashMap;

fn main() {
    println!("^_^ Rust Standard Library Types ^_^");
    // Doc: https://doc.rust-lang.org/std/

    // *** Vector
    /*
     * Cấu trúc dữ liệu động, có thể mở rộng phần tử
     * Các phần tử phải trùng kiểu dữ liệu
     */

    // Tạo 1 vector rỗng
    // Cách 1:
    let vect: Vec<u8> = Vec::new();
    // Cách 2: vec! gọi là macro
    let vect1: Vec<u32> = vec![];
    println!("{:?}, macro: {:?}", vect, vect1);

    // Tạo 1 vector có giá trị
    let mut vect2: Vec<u32> = vec![1, 2, 3];
    println!("{:?}", vect2);

    vect2.push(4);
    println!("Thêm 'mut' nếu muốn thêm phần tử cho vec: {:?}", vect2);
    println!(
        "Giá trị phần tử tại vị trí số 3: {}, và vị trí đầu tiên: {:?}",
        vect2[3],
        vect2.first()
    );
    println!("Dùng 'method get()' để lấy phần tử: {:?}", vect2.get(0));

    for item in &vect2 {
        // println!("item in vect2: {:?}", item);
        println!("item in vect2: {}", item);
    }
    // bị lỗi vì ở vòng for thứ nhất vect2 đã truyền ownership cho từng phần tử item phải thêm "&" để cho vay, và hạn chế dùng clone() vì sẽ tạo ra 1 phiên bản khác
    println!("Thay đổi giá trị của Vector");
    for item in &mut vect2 {
        // Sử đổi giá trị trong vector
        // &mut u32 trỏ tới địa chỉ có giá trị là item
        // Vì thế muốn thay đổi: ta cần lấy giá trị với "*"
        *item += 1;
        println!("item in vect2: {}", item);
    }
    vect2.remove(1);
    println!("Remove phần tử theo vị trí index: {:?}", vect2);

    /*
     * Sử dụng Iterator
     *  - iter(): Mượn (borrowing) mỗi phần tử của collection sau mỗi lần duyệt
     *  - into_iter(): Duyệt đúng từng phần tử của collection -> collection sẽ bị "drop" sau khi duyệt xong
     *  - iter_mut(): Mượn mỗi phần tử sau mỗi lần duyệt và có thể thay đổi giá trị của phần tử (mutable reference)
     * *** Kết hợp với closure
     */

    // iter()
    let colors: Vec<&str> = vec!["Red", "Yellow", "Green"];
    for color in colors.iter() {
        println!("iter: {}", color);
    }

    // Collection vẫn còn "tồn tại" sau khi dùng inter();
    println!("colors sau iter() = {:?}", colors);

    // into_iter()
    for color in colors.into_iter() {
        println!("into_iter: {}", color);
    }
    // Collection bị "drop" sau khi dùng into_iter();
    // println!("colors = {:?}", colors);  // borrow of moved value

    // iter_mut()
    let mut colors_itmut: Vec<&str> = vec!["Red", "Yellow", "Green"];
    for color in colors_itmut.iter_mut() {
        println!("iter: {}", color);
        *color = "Black";
    }

    // Collection vẫn còn "tồn tại" sau khi dùng và có thể thay đổi giá trị trong khi dùng iter_mut();
    println!("colors sau iter() = {:?}", colors_itmut);

    // Kết hợp với closure
    let mut numbers: Vec<i32> = vec![1, 2, 3];
    numbers.iter_mut().for_each(|x| *x += 1);
    println!("numbers: {:?}", numbers);

    let numbers1: Vec<i32> = vec![1, 2, 3];
    let res: Vec<i32> = numbers1.iter().map(|x| x + 1).collect();
    println!("res: {:?}", res);

    let numbers2: Vec<i32> = vec![3, 4, 5];
    let mut res = Vec::new();
    for numb in numbers2.iter() {
        res.push(numb + 1);
    }
    println!("res of numbers2: {:?}", res);

    let mut numbers3: Vec<i32> = vec![4, 5, 6];
    for numb in numbers3.iter_mut() {
        *numb += 1;
    }
    println!("res of numbers3: {:?}", numbers3);

    // *** HashMap
    /*
     * - Cấu trúc dữ liệu lưu trữ khóa-giá trị (key-value)
     * - Khởi tạo:
     *  + Import HashMap
     *  + Định nghĩa Instance bằng hàm new()
     */

    let mut user = HashMap::new();
    user.insert("username", "Alice");
    user.insert("nickname", "AIC");
    println!("User: {:?}", user);
    println!("User: get username {:?}", user.get("username"));

    for item in user.iter() {
        println!("Key: {}, Value: {}", item.0, item.1);
    }

    for (key, value) in user.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
    user.insert("username", "Jack");
    println!("User: {:?}", user);
}
