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
}
