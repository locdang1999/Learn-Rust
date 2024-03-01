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
        let name = String:: from_utf8("Xin chào Việt Nam".into());
        println!("In scope: {:?}", name);
    }
    // Giải phóng bộ nhớ cho biến "name" -> Giá trị của "name" không còn tồn tại
    // println!("Out scope: {:?}", name); // cannot find value "name" in this scope
}
