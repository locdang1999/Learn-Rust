fn main() {
    println!("^_^ Rust Ownership & Borrowing ^_^");

    // *** Ownership: (Quyền sở hữu)
    /*
     * + Đây là cơ chế quản lý bộ nhớ chương trình của Rust
     * + Gổm 3 quy tắc nhằm đảm bảo an toàn bộ nhớ chương trình:
     *  1> Mỗi giá trị chỉ có owner
     *  2> Chỉ có 1 owner tại 1 thời điểm
     *  3> Khi owner không còn nằm trong phạm vi thì giá trị sẽ bị dropped (xóa)
     * - Phạm vi biến trong Rust
     * + Phạm vi ở đây có nghĩa là 1 block code được định nghĩa trong {}
     * + Biến định nghĩa trong 1 phạm vi là hợp lệ, ngoài phạm vi {} thì biến không hợp lệ (vùng nhớ lưu giá trị của biển đó đã tự động xóa)
     */

    {
        let name = String::from_utf8("Xin chào Việt Nam".into());
        println!("In scope: {:?}", name);
    }
    // Giải phóng bộ nhớ cho biến "name" -> Giá trị của "name" không còn tồn tại
    // println!("Out scope: {:?}", name); // cannot find value "name" in this scope

    // Quy tắc 1: `fruit1` đang sở hữu 1 giá trị có kiểu String
    let fruit1 = String::from("Banana"); // Kiểu String, vector,.. lưu ở Heap

    // Quy tắc 2: Chỉ có 1 Owner tại 1 thời điểm -> Tính sở hữu (ownership chuyển qua từ `fruit1` -> `fruit2`)
    let fruit2 = fruit1;

    // Quy tắc 3: dropped => Khi đó không thể in ra `fruit1` vì `fruit2` đã mất quyền sở hữu => Giá trị của `fruit1` bị xóa
    // println!("fruit1 = {}", fruit1); // borrow of moved value: `fruit1`
    // Nếu `fruit1` được in ra sẽ gây ra "garbage collector" và liên quan đến quản lý bộ nhớ
    println!("fruit2 = {}", fruit2);

    let x = 11; // Kiểu int, float,... lưu ở stack
    let y = x;
    println!("x: {}, y: {}", x, y);

    // Ownership with function
    let fruit3 = String::from("Apple");
    print_ownership(fruit3); // Ownership(Tính sở hữu) của "fruit3" được chuyển qua "str" của function print_ownership
    // println!("fruit = {}", fruit3); // "fruit3" bị dropped

    // *** Reference and Borrowing: (Tham chiếu)
    /*
     * Trỏ tới giá trị mà không sở hữu giá trị đó (mượn)
     * Tránh lỗi do nguyên tắc Ownership
     * Tính mượn -> sử dụng tính chất Reference (con trỏ) => tránh vi phạm Ownership
     * Tính đọc -> "&" (ampersand) -> share Reference
     * Tính thay đổi (write) -> mượn nhưng có sự cho phép của Owner để thay đổi thông tin -> Mutable Reference
     */
    let fruit4 = String::from("Orange");
    // Cách 1: clone
    let fruit5 = fruit4.clone();
    print_ownership(fruit4.clone()); // "clone()" tạo ra 1 địa chỉ mới so với thằng `fruit4` (fruit4.clone() != fruit4)
    println!("Owner fruit5: {}", fruit5);
    // Cách 2: Sử dụng Reference (con trỏ): "&" -> mượn => Tránh vi phạm ownership
    print_reference(&fruit4);
    let mut fruit6 = String::from("Cherry");
    print_mut_reference(&mut fruit6);

    let mut s1 = String::from("S1");
    let s2 = &s1;
    let s3 = &s1;
    println!("s1: {} - s2: {} - s3: {}", s1, s2, s3);
    let s4 = &mut s1;
    println!("s4: {:?}", s4); // s1 phải sửa thành mut. Nhưng vì s2, s3 mượn (&) từ s1 và được sử dụng sau khi thay s4 mượn và thay đổi (&mut) s1 nên bị lỗi
    // println!("s1: {} - s2: {} - s3: {}", s1, s2, s3); // => Không sử dụng các biến mượn từ s1 sau khi thay đổi s1, hoặc loại bỏ các biến mượn từ s1 trước khi s1 bị thay đổi
    // let s5 = &mut s1; // Chỉ cho duy nhất 1 biến vừa mượn vừa thay đổi và chủ sở hữu ban đầu đã bị mượn và thay đổi rồi thì cũng không thể sử dụng "và" cho mượn "và" thay đổi được nữa
    // println!("s4: {:?}", s4);// lỗi khi gán với s5

    // Dangling Pointer
    // let refer_to_nothing = dangle();
}

fn print_ownership(str: String) {
    println!("print_ownership = {}", str);
}

fn print_reference(str: &String) {
    println!("print_reference = {}", str);
}

fn print_mut_reference(str: &mut String) {
    println!("Before mutable reference = {}", str);
    str.push_str(" Lemon");
    println!("After Mutable referenc = {}", str);
}

// fn dangle() -> &String {
//     let s = String::from("value");
//     // trả về 1 tham chiếu của chuỗi
//     &s // lỗi null pointer vì biến "s" bị drop khi kết thúc scope lúc này sẽ ko lấy được giá trị
// }
