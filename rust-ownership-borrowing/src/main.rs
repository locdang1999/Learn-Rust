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
    print_ownership(fruit3);// Ownership(Tính sở hữu) của "fruit3" được chuyển qua "str" của function print_ownership
    // println!("fruit = {}", fruit3); // "fruit3" bị dropped

    // *** Reference and Borrowing: (Tham chiếu)
    /*
     * Trỏ tới giá trị mà không sở hữu giá trị đó (mượn)
     * Tránh lỗi do nguyên tắc Ownership
     */
}

fn print_ownership(str: String) {
    println!("print_ownership = {}", str);
}
