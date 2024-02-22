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

    // *** While
    /*
     * Lặp cho đến khi gặp đk sai
     */
    let mut count = 5;
    while count > 0 {
        println!("While - number: {count}");
        count -= 1;
    }

    // *** For
    /*
     * Lặp cho đến khi gặp đk sai
     */

    let arr = [1, 2, 3, 4, 5];
    for itm in arr {
        println!("For - arr[i]: {itm}");
    }

    let mut count_arr = 0;
    while count_arr < arr.len() {
        println!("While - arr{count_arr}: {}", arr[count_arr]);
        count_arr += 1;
    }

    for itm in arr.iter() {
        println!("For with iter() - arr[i]: {itm}");
    }

    for itm in arr.into_iter() {
        println!("For with into_iter() - arr[i]: {itm}");
    }

    println!("While - count: {count_arr}");
    for itm in 1..11 {
        // 1..11: từ 1,2,...->10
        count_arr += itm;
    }
    println!("For - sum: {count_arr}");

    //Tăng thêm 1 đơn vị cho từng phần tử trong mảng
    let mut arr_new = [2, 3, 4, 5, 6];
    for itm in &mut arr_new {
        // 1..11: từ 1,2,...->10
        *itm += 1;
    }
    println!("For - New array: {:?}", arr_new);
}
