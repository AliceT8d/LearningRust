fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut arr = [1, 23, 23, 45, 677, 83];
    let mut sum = 0;
    for item in &arr {
        sum = sum + item; // i32 + &i32 这个两个类型不一致，item 是 arr 每个元素的不可变借用
    }

    for item in arr {
        // arr 是 [i32; 6] 类型。
        // 在 Rust 中，数组直接实现了 IntoIterator，for item in arr 会消耗数组，按值迭代每一个元素。
        // 正常来说会消耗 这个数组的所有权，这里按下不表，后面会有介绍
        // 因此 item 的类型是 i32（元素的所有权直接移动出来），而不是 &i32。
        // sum 是 i32，item 也是 i32，sum + item 是 i32 + i32，类型完全匹配。
        sum = sum + item;
    }

    println!("{:?}", arr);
    println!("sum: {}", sum);

    let mut arr2 = [1, 2, 3, 4, 5, 6];
    // 反转数组 需要一个可变借用
    arr2.reverse();
    arr2.sort();
    println!("{:?}", arr2);

    // 需要两根相同类型的数组才能 concat
    println!("arr + arr2 = {:?}", [arr, arr2].concat());

    // 元组
    let (l, r) = arr2.split_at(3);
    println!("l: {:?}", l);
    println!("r: {:?}", r);

    let age = 20;
    if age < 18 {
        // 没有括号，这个和C语言很不一样
        println!("未成年");
    } else {
        println!("已经成年");
    }

    // 三元运算符？
    let message = if age < 12 {
        who();
        "child"
    } else {
        "young man"
    };

    let num = 1;
    // 类似 switch case
    match num {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!(),
    }

    let ab = match num {
        1 => "hello",
        2 => "world",
        _ => "",
    };

    // 元组
    let tp = (1, -1);
    match tp {
        (0, 0) => println!("zero point"),
        (_, y) if y < 0 => {
            print!("tuple y < 0");
        }
        (x, _) if x < 0 => {
            print!("tuple x < 0");
        }
        _ => println!("tuple x and y >= 0"),
    }

    let arr1 = [1, 2, 3, 4, 5];
    let is_head_gt_0 = match arr1 {
        [n, _, _, _, _] | [_, n, _, _, _] if n > 0 => true,
        _ => false,
    };

    // aka
    let is_head_gt_0_2 = matches!(arr1,[n, _, _, _, _] | [_, n, _, _, _] if n > 0);

    let t = (2, 4);
    if let (x @ 1..=2, y @ 1..4) = t {
        println!();
        println!("1<=x<=2 && 1<=y<4");
    } else {
        println!();
        println!("not valid");
    }
}

// 单元类型，类似 void 类型？
fn who() -> () {
    print!("123123");
}
