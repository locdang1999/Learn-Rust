fn main() {
    println!("^_^ Rust Traits ^_^");

    // *** Define Trait
    /*
     * Trait là một cách để định nghĩa một tập hợp các đặc tính (hành vi) được chia sẻ giữa các kiểu dữ liệu khác nhau
     *    + Chức năng như 1 Interface ở 1 số ngôn ngữ khác
     * Implement trait: bằng cách sử dụng từ khóa trait và liệt kê các phương thức và tính chất của 1 trait đó
     */

    let vios = Car {
        category: "Sedan".to_string(),
    };
    let vios_speed = vios.speed();
}

pub struct Car {
    category: String,
}

pub struct Motorbike {
    category: String,
}

// impl Car {
//     fn get_category(&self) {
//         println!("Category:{}", self.category);
//     }
// }

// Sử dụng trait để mô tả đặc tính chung cho Car và Motorbike

pub trait Vehicle {
    // Mô tả đặc tính
    // bằng hàm
    fn get_category(&self) -> String;
    fn speed(&self) -> u32;
}

// Implement trait
// Biểu diễn đặc tính cụ thể cho Car
//
impl Vehicle for Car {
    fn get_category(&self) -> String {
        self.category.clone()
    }

    fn speed(&self) -> u32 {
        100
    }
}
