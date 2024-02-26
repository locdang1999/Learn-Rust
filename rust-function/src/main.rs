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
    let vec_x = vec![1, 2, 5, 7, 8, 9, 6];
    println!("Total: {}, {}", add(a, b), sub(a, b));
    println!("Find max: {}", find_max(&arr));
    // println!("Find even number: {}", );
    // println!("Find even number Vec: {}", find_even_vec(&vec_x));
    find_evennum(&arr);
    find_even_vec(&vec_x);
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
// Adding a _ to the beginning of the variable name (which is the standard way of telling the compiler 'Yes, this is unused on purpose')
// https://doc.rust-lang.org/beta/nightly-rustc/rustc_lint_defs/builtin/static.UNUSED_VARIABLES.html#:~:text=Explanation,an%20underscore%20such%20as%20_x%20.
fn _print_text(){
    let test = "test";
    println!("{test}")
}

#[allow(dead_code)] // là một thuộc tính vô hiệu hóa lint `dead_code`
// https://doc.rust-lang.org/rust-by-example/attribute/unused.html
fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn rem_last(value: &str) -> &str {
    let mut chars = value.chars();
    // chars.next();
    chars.next_back();
    chars.as_str()
}

fn find_evennum(arr: &[i32]) {
    //-> &[i32]
    let mut even = [0;3];
    let mut i = 0;
    let mut text_even = String::from("");
    for &itm in arr {
        if itm % 2 == 0 {
            even[i] = itm;
            i += 1;
            text_even = text_even +  &itm.to_string() + ", ";
        }
    }
    println!("Arrays: Cách 1: {:?}", even);
    println!("Arrays: Cách 2: [{}]", rem_last(text_even.trim()));
}

fn find_even_vec(vec_arr: &[i32]){
    let mut new_vec =  vec![];
    let mut i = 0;
    for &itm in vec_arr {
        if itm % 2 == 0 {
            new_vec.insert(i, itm);
            i += 1;
        }
    }
    println!("Dùng Vec: {:?}", new_vec);
}
