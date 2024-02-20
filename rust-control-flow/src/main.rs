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

    if !y {
        println!("Invalid")
    } else {
        println!("Valid")
    }

    // *** LOOP
    /*
        * Vòng lập vô hạn, chỉ dừng khi gặp từ khóa "break"
        * Dùng khi không biết trước số lần lập cần thiết
     */
    let mut num = 10;
    loop {
        println!("Loop in rust");
        break;
    }
    loop {
        if num <= 0 {
            break;
        }       
        println!("Number is: {num}");
        num -= 1;
        // println!("{}", --num); // Rust ko sử dụng cú pháp ++, --
    }
}
