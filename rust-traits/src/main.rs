use std::{clone, f64::consts::PI};

fn main() {
    println!("^_^ Rust Traits ^_^");

    // *** Define Trait
    /*
     * Trait là một cách để định nghĩa một tập hợp các đặc tính (hành vi) được chia sẻ giữa các kiểu dữ liệu khác nhau
     *    + Chức năng như 1 Interface ở 1 số ngôn ngữ khác
     * Implement trait: bằng cách sử dụng từ khóa trait và liệt kê các phương thức và tính chất của 1 trait đó
     *
     * Trait có thể là đầu vào của hàm
     * - Trait có thể là đầu vào của hàm -> Đầu vào này là 1 thực thể của 1 object mà có implement trait Vehicle
     * - Cần có từ khóa "impl" khi pass trait là đầu vào của hàm
     */

    let vios = Car {
        category: "Sedan".to_string(),
    };
    let vios_speed = vios.speed();

    print_vehicle_info(&vios);

    let bike = Bicycle {
        category: "moto".to_string(),
    };
    // print_vehicle_info(&bike); // Bị lỗi vì Vehicle chỉ biểu diễn cho thằng nào implement nó

    // *** Trait Bound: thường được sử dụng cho generic function

    check_speed(&vios);

    // *** Returning Traits: Bởi vì Rust không biết chính xác khi trả về trait -> sử dụng Box<dyn Trait> => Lưu vào Heap
    let vios2 = get_vehicle(&"Car".to_string());
    // *** Traits Combos: Thêm cú pháp dấu "+" => 1 object có thể "constraint" nhiều trait khác nhau (nghĩa là implement nhiều đặc tính khác nhau)
    print_insurable_info_2(&vios);
    print_insurable_info(&vios);

    // *** Super traits
    /*
     * Là 1 trait phụ thuộc vào những trait khác
     * Định nghĩa 1 trait Displayable có supertrait là Vehicle
     */
    display_dis(&vios);

    // *** Trait Object
    /*
     * Trait Object là 1 cách để làm việc với các đối tượng có kiểu trait chung, cho phép tạo ra 1 danh sách đa dạng các đối tượng có cùng kiểu trait
     * Có 2 dạng sử dụng của Trait Object: Static Dispatch và Dynamic Dispatch
     */
    // circle là 1 instance của object Circle
    let circle = Circle { radius: 10.0 };

    let rec = Rectangle {
        width: 2.0,
        height: 5.0,
    };

    // sử dụng trait object -> dynamic trait object
    let vec: Vec<Box<dyn Drawable>> = vec![Box::new(circle.clone()), Box::new(rec.clone())];

    // *** Static-Dispatch
    /*
     * - Dispatch nghĩa là hàm phương thức(method) có sử dụng liên quan tới trait
     * - Static Dispatch
     *  + Xảy ra khi dữ liệu cụ thể của đối tượng được biết trong quá trình biên dịch (nghĩa là kích thước của object sẽ biết trong quá trình biên dịch -> cấp phát bộ nhớ cho object)
     *  + Việc triển khai phương thức (method call) được xử lý trong quá trình biên dịch dựa trên kiểu dữ liệu cụ thể
     * - Cách định nghĩa Static Dispatch
     *  + parameter: sử dụng generic type -> constraint bởi trait
     *  + parameter là 1 kiểu dữ liệu cụ thể (object) có liên quan tới trait
     *  + VD: Định nghĩa static dispatch sử dụng trait như 1 parameter
     */
    draw_static(&circle);
    draw_static(&rec);

    // object not implement trait Drawable
    let tri = Triangle{};
    // draw_static(&tri); // lỗi vì ko có implement Drawable
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

impl Vehicle for Motorbike {
    fn get_category(&self) -> String {
        self.category.clone()
    }

    fn speed(&self) -> u32 {
        70
    }
}

impl Insurable for Car {
    fn insurable_name(&self) -> String {
        "Toyota".to_string()
    }
}

// impl Vehicle
// có 2 object là car và motorbike
// tương đương khi sử dụng thẳng object Car
fn print_vehicle_info(vehicle: &impl Vehicle) {
    println!(
        "Category: {}, Speed: {}",
        vehicle.get_category(),
        vehicle.speed()
    );
}
// <=> // tương đương khi sử dụng thẳng object Car
// fn print_vehicle_info_car(vehicle: &Car) {
//     println!(
//         "Category: {}, Speed: {}",
//         vehicle.get_category(),
//         vehicle.speed()
//     );
// }

pub struct Bicycle {
    category: String,
}

// Trait Bound
fn check_speed<T: Vehicle>(vehicle: &T) {
    if vehicle.speed() > 80 {
        println!("{} is fast!", vehicle.speed());
    } else {
        println!("{} is slow!", vehicle.speed());
    }
}
// check_speed <=> check_speed2 nhưng check_speed sẽ được sử dụng nhiều hơn
fn check_speed2(vehicle: &impl Vehicle) {
    if vehicle.speed() > 80 {
        println!("{} is fast!", vehicle.speed());
    } else {
        println!("{} is slow!", vehicle.speed());
    }
}

// Return Trait
fn get_vehicle(vehicle_type: &str) -> Box<dyn Vehicle> {
    match vehicle_type {
        "Car" => Box::new(Car {
            category: String::from("Car"),
        }),
        _ => Box::new(Motorbike {
            category: String::from("Motobike"),
        }),
    }
}

// Trait combos
trait Insurable {
    fn insurable_name(&self) -> String;
}

// trait là parameter
fn print_insurable_info(item: &(impl Vehicle + Insurable)) {
    println!(
        "{} is insured by {}",
        item.get_category(),
        item.insurable_name()
    );
}

fn print_insurable_info_2<T: Vehicle + Insurable>(item: &T) {
    println!(
        "{} is insured by {}",
        item.get_category(),
        item.insurable_name()
    );
}

// *** Super traits
trait Displayable: Vehicle {
    fn display_info(&self) {
        println!(
            "vehicle Category: {}, Speed: {} km/h",
            self.get_category(),
            self.speed()
        );
    }
}

impl Displayable for Car {}

fn display_dis<T: Displayable>(item: &T) {
    item.display_info();
}

// *** Trait Object
#[derive(Clone)]
struct Circle {
    radius: f64,
}
#[derive(Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}

trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

impl Drawable for Circle {
    fn draw(&self) {
        println!(" Drawing a circle");
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!(" Drawing a rectangle");
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// *** Statis Dispatch
// T như là 1 trait object được constraint bởi trait Drawble
// Sử dụng trait bound
// Sử dụng trait như 1 parameter
pub struct Triangle {}

fn draw_static<T: Drawable>(shape: &T) {
    shape.draw();
}
//    <=>
fn draw_static_cir(shape: &Circle) {
    shape.draw();
}
