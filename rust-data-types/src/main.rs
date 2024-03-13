fn main() {
    println!("^_^ Rust Data Types ^_^");
    // *** Enum (Enumeration)
    /*
     * Định nghĩa 1 kiểu dữ liệu có các biến thể cố định
     */

    let up = Direction::Up;
    println!("Direction value: {:?}", up);
    let up = Direction::Down;
    println!("Direction value: {:?}", up);
    let up = Direction::Left;
    println!("Direction value: {:?}", up);
    let up = Direction::Right;
    println!("Direction value: {:?}", up);

    //match: thường sử dụng với enum và tương đương với if else, case
    match up {
        Direction::Up => println!("Robot go up"),
        Direction::Down => println!("Robot go down"),
        Direction::Left => println!("Robot go left"),
        Direction::Right => println!("Robot go right"),
    }

    // Convert Enum sang 1 kiểu dữ liệu khác or ngược lại
    let mood = Mood::Angry;
    println!("Mood value: {:?}", mood);
    println!("Mood value: {:?}", match_mood(&mood));
    let mood_h = Mood::Happy;
    println!("Mood value: {:?}", mood_h);
    println!("Mood value: {:?}", match_mood(&mood_h));
    let mood_nb = Mood::NotBad;
    println!("Mood value: {:?}", mood_nb);
    println!("Mood value: {:?}", match_mood(&mood_nb));
    let mood_s = Mood::Sleepy;
    println!("Mood value: {:?}", mood_s);
    println!("Mood value: {:?}", match_mood(&mood_s));

    // Cách 1: khi sử dụng enum
    // let starvec = vec![
    //     Start::BrownDwarf,
    //     Start::RedDwarf,
    //     Start::YellowStart,
    //     Start::RedGiant,
    //     Start::DeadStar,
    // ];

    // Cách 2:
    use Start::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStart, RedGiant, DeadStar];

    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."),
            size if size >= 80 => println!("This is a good-sized star."),
            _ => println!("That star is pretty big!"),
        }
    }
    println!("What about DeadStar? It's the number {}.", DeadStar as u32);

    //Enum sử dụng nhiểu kiểu dữ liệu khác nhau (wrap)
    // Convert rust primitive sang kiểu dữ liệu enum mà mình custom
    let my_num = vec![get_number(-800), get_number(8)];
    for itm in my_num {
        match itm {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's a i32 with the value {}", number),
        }
    }

    // *** Struct
    /* Định nghĩa:
     *  - Kiểu dữ liệu tổng hợp
     *  - Cấu trúc dữ liệu với nhiều thành phần có các kiểu dữ liệu khác nhau
     * Đặc điểm
     *  - Tổ chức dữ liệu
     *  - Kiểu dữ liệu tùy chỉnh
     *  - Biểu đạt hành vi (Method Implementation):
     *   + Sử dụng từ khóa "impl"
     *   + Mô tả hành vi cho 1 vatah thể (object) nào đó
     *  - Phân biệt self và Self trong Struct
     *   + self: Thể hiện cho 1 instance (thực thể) hiện tại của 1 object (như là "this") đối với các ngôn ngữ khác
     *   + Self: Đại diện cho đối tượng chung (class)
     *  - Dùng self, &self, &mut self
     *   + self: Mang tính sở hữu (Ownership)
     *   + &self: Mang tính mượn (Reference)
     *   + &mut self: Mang tính thay đổi (Mutable Reference)
     */

    // Khác nhau giữa kiểu struct và tuple
    let _student = ("Alice", 10, "C"); // Tuple

    // Sử dụng Struct
    // Định nghĩa instance
    let alice = Student {
        name: String::from("Alice"),
        age: 24,
        class: String::from("C"),
    };

    println!("Alice Age: {}", alice.age);

    // Cũng sử dụng "mut" để thay đổi giá trị của "Struct"
    let mut jack = Student {
        name: String::from("Jack"),
        age: 24,
        class: String::from("C"),
    };

    jack.class = String::from("A");
    println!("Jack Class: {}", jack.class);

    println!("Name of Student: {}", jack.get_name());
    println!("Name of Student: {}", alice.get_name());

    let jimmy = Student::new();
    println!("Jack Class: {:?}", jimmy);

    println!("Class of Student 1: {}", alice.get_class());
    // println!("Alice Age: {:?}", alice); // Lúc này 'alice' sẽ không còn tồn tại vì đã reference (cho mượn) 'alice.get_class()'
    println!("Name of Student 2: {}", jimmy.get_name2());
    println!("Alice: {:?}", jimmy); // Vấn sử dụng 'jimmy' được vì chỉ shared reference

    jack.set_class(String::from("B"));
    println!("Jack Class: {}", jack.class);
    println!("Jack: {:?}", jack);
}

// *** Enum
fn match_mood(mood: &Mood) -> i32 {
    let happiness_level = match mood {
        Mood::Happy => 10,
        Mood::Sleepy => 6,
        Mood::NotBad => 7,
        Mood::Angry => 2,
    };
    happiness_level
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

#[derive(Debug)]
enum Start {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStart = 100,
    RedGiant = 1000,
    DeadStar,
}

#[derive(Debug)]
enum Number {
    U32(u32),
    I32(i32),
}
fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    };
    number
}

// *** Struct
#[derive(Debug)]
pub struct Student {
    pub name: String,
    pub age: u8,
    pub class: String,
}

// Mô tả hành vi cho Stuct Student
impl Student {
    // Constructor - hàm khởi tạo
    pub fn new() -> Self {
        Student {
            name: String::from("Jimmy"),
            age: 24,
            class: String::from("C"),
        }
    }

    // Sử dụng self (Ownership)
    pub fn get_class(self) -> String {
        // println!("Test {:?}", self); Muốn log "&self" phải thêm "#[derive(Debug)]" ở Struct
        self.class // hoặc dùng clone() or to_owned()
    }

    // Sử dụng &self (shared Reference)
    pub fn get_name(&self) -> String {
        // println!("Test {:?}", &self); Muốn log "&self" phải thêm "#[derive(Debug)]" ở Struct
        self.name.to_string() // hoặc dùng clone() or to_owned()
    }

    // Sử dụng &self (shared Reference)
    pub fn get_name2(&self) -> &String {
        // println!("Test {:?}", &self); Muốn log "&self" phải thêm "#[derive(Debug)]" ở Struct
        &self.name // hoặc dùng clone() or to_owned()
    }

    // Sử dụng "&mut self" (Mutable Reference)
    pub fn set_class(&mut self, class: String) {
        self.class = class;
    }
}
