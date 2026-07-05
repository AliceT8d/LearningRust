fn main() {
    // 引用
    // 默认为不可变的
    let value = 42; // i32
    let ref_of_value = &value; //&i32

    println!("Value is {}", *ref_of_value); // 手动解引用
    println!("Value is {}", ref_of_value); // 自动解引用
    println!("value's addr is {:p}", ref_of_value); // 打印被引用者的地址
    println!("ref_of_value's addr is {:p}", &ref_of_value); // 打印引用者的地址
                                                            // 如果要打印 地址 这种类型 需要使用 :p

    // 借用
    let mut num1 = 42; // 即便这里加入 mut 关键字，也不行
    let ref_num1 = &num1; // 这个操作就是 （不可变）借用操作。无法修改 num1
                          // 这点与C语言很不一样

    let mut num1 = 42; // 因为num1可变，所可以创建可变借用，无 mut 关键字则不可以
    let ref_num1 = &mut num1; // 这个操作就是 （可变）借用操作。可以修改

    // 不可变借用 可以创建多个
    // 不支持 对同一个对象创建多个可变借用
    // 跟指针的概念很不相同，不允许同一时期存在 同一个对象的可变借用和不可变借用

    let mut v1 = 1;
    let v2 = &v1; // v2: &i32
    let v3: &mut i32 = &mut v1; // v3: &mut i32

    // 切片 slice
    // 切片总是一种借用，可以是可变的 也可以是不可变的

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let s1 = &arr[1..3]; // 所以 1..3 表示 [1, 3)，这个是下标的1-3（下标从0开始），如果需要包含3，使用 [1..=3]
                         // 空的 [..] 表示所有的数组元素
    println!("{:?}", s1);
}

fn print_value(arg: &i32) {
    // 一个 i32 对象的不可变借用
    println!("{}", arg);
}
