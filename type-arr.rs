fn main() {
    // rust 不会像C++那样做隐式转换 需要使用 as 关键字进行强转
    // 可以使用 `_Type` 作为后缀，表面这个数据是 Type 类型的
    // 比如 100_u8, 0xffff_u32, 0.12_f32 仅限基本数据类型
    // rust 也可以这样分表示一个数 100_000_000，2.23_34_f64
    // 如果想在代码在 存储 ASCII 字符，使用 `b'+'`这样的， `'+'_u8`是不合法的
    let a = b'a';
    let aa = b'A';
    let x = 43_u8;
    let y = 43;
    println!("'a' - 'A'={}", a - aa);
    // 也可以使用as关键字将 ASCII 转为 char 可以多次转换
    print!("{}", x as char);
    print!("{}", y as u8 as char);
    // 在rust中，char 占用 4个字节，并不像C++ 和 C语言那样占用一个字节！

    // 展示文字
    let chinese = '\u{4f60}'; // ‘你’ 这里必须要加花括号
    println!("{}", chinese);

    // rust 不支持 u32 as char 因为 不是所有的 u32 值都有 对应的字符
    // 如果要这么做，需要使用 char::from_u32 函数，这回返回 Option<char>
    let u32v = 0xffff_u32;
    // let ch = u32v as char; //ERROR
    let ch = char::from_u32(u32v);
    print!("ch:{:?}", ch);

    // 创建一个数组
    // let variable : [type; size] = [v1, v2, v3 ...]; // type 和 size 必须一起存在
    let ints = [1, 2, 3, 4]; // size 必须和实际的数量对应，不可以像C++那样声明一部分
    // 数组的大小不支持在运行时确定大小，就像C++一样，不同于C语言的VLA

    // println!("{}", ints); ERROR
    println!("{:?}", ints);
    println!("{:#?}", ints);
    // 重复表达式
    // 如果要创建一个数组，全部都为 0
    let arr = [0; 10];
    println!("{:?}", arr);
    // 也可以在初始化时候指定类型
    // 数组也是默认不可以更改的， 要改需要加 mut 关键字
    let arr = [10_u8; 20];

    // 数组的索引从 0 开始
    let x = arr[2];
    // 无法 arr[2] = 2;

    // + - * / % 语法和C++一样 但注意类型不匹配不能进行运算
}
