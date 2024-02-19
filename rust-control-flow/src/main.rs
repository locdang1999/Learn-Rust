fn main() {
    println!("***----Rust Control Flow----***");
    // *** IF ELSE
    let x = 10;
    let y = false;
    if x >= 8 {
        println!("Excellent student (Học sinh giỏi)")
    } else if x >= 6.5 as i32 {
        // Không thể so sánh int 32bit với float 64bit nên pahir ép kiểu về i32(int 32bit)
        println!("Good student (Học sinh khá)")
    } else {
        println!("Average student (Học sinh trung bình)")
    }

    if !y{
        println!("Invalid")
    }else {
        println!("Valid")
    }
}
