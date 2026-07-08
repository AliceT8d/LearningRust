fn main() {
    let greeting = "Hello, world!"; // 字符串字面量
    print!("{}", greeting);

    let my_str = String::new(); // 创建一个空的字符串 不支持 new("xxx") 这种语法
                                // 调用结构体的静态方法
    let my_str = String::from("Hello");
    let my_str = "Hello".to_string();

    let s1 = String::from("s1");
    let s3 = String::from("s1");
    let s2 = s1; // 这里相当于C++的 std::move(s1) 将 s1 的所有权移交给 s2
                 // s1 将在交付所有权之后不再可用
                 // print!("{}", s1); 报错！
    let s2 = s3.clone(); // 深拷贝
                         // String 没有实现 copy trait

    // str vs slice
    // 由于 String 内的字符串存储为 UTF8 类型，所以每个字符没有一个清晰的边界，至少对程序来说是这样的
    // 所以不能将 字符串切片&str 理解为 &[char] 因为内部根本不是char类型组成的
    // let hello = &greeting[0..5];
    let hello = &greeting[0..5];

    println!("{}", hello);

    let h = s2.as_str(); // 这个方法返回字符串切片类类型 &str 还有as_mut_str -> &mut str

    // 如果要实现字符串相加
    // String 的 add trait 需要一个&str
    let s4 = "hello".to_string();
    let s5 = s4 + s3.as_str();
    // println!("{}", s4);
    // 如果我不想丢失所有权，我可以这么做
    // 使用 format! 宏
    // let s5 = format!("{}{}", s4, s3);
    // 或者 .clone

    /*
       实现两个字符串拼接的 `+` 运算符。该操作会消耗左侧的 `String`，并复用其缓冲区（必要时会扩容）。
       这样做是为了避免每次操作都分配新的 `String` 并复制全部内容——若通过重复拼接构建 n 字节的字符串，
       这种低效方式会导致 O(n²) 的运行时间。
       右侧的字符串仅以借用形式参与运算，其内容会被复制到返回的 `String` 中。
    */
}
