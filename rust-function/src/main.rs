use phpify::array::array_unshift;
fn main() {
    println!(" ^_^ Rust Function ^_^ ");

    // *** Function: Hàm
    /*
     * Tập hợp các đoạn code để thức hiện 1 logic, nhiệm vụ nào đó
     */
    print_hello();
    let a = 2;
    let b = 3;
    print_num(a, b);
    let a = 5;
    let b = 4;
    let arr = [1, 2, 5, 7, 8, 9, 6];
    println!("Total: {}, {}", add(a, b), sub(a, b));
    println!("Find max: {}", find_max(&arr));
    // println!("Find even number: {}", );
    find_evennum(&arr);
}

fn print_hello() {
    println!("-----*** Hello Rust with Function ***-----")
}

fn print_num(a: i32, b: i32) {
    println!("{a}, {b}")
}

fn add(a: i32, b: i32) -> i32 {
    // Cách 1: trả về duntgf "return"
    // return a + b;

    // Cách 2 trả về sẽ ko có dấu ";" vì khi đó rust sẽ hiểu thành 1 biểu thức
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    // Cách 1: trả về duntgf "return"
    // return a + b;

    // Cách 2 trả về sẽ ko có dấu ";" vì khi đó rust sẽ hiểu thành 1 biểu thức
    a - b
}

// slice array: &[1,2,3]
fn find_max(arr: &[i32]) -> i32 {
    let mut max = arr[0];
    for &itm in arr {
        if itm >= max {
            max = itm
        }
    }
    max
}

fn find_evennum(arr: &[i32]) {
    //-> &[i32]
    let mut even: &[i32];
    for &itm in arr {
        if itm % 2 == 0 {
            even = &[itm];
            // println!("{}", itm);
            // even
            array_unshift(&mut even, itm);
        }
    }

    println!("{:?}", even);
}
