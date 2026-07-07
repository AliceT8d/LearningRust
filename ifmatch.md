
# Rust 数组、所有权、match 与 if let 笔记

## 完整的可运行示例

```rust
fn main() {
    // ---- 数组与引用迭代 ----
    let arr = [1, 23, 23, 45, 677, 83];
    let mut sum = 0;

    // 方式一：for item in &arr → item 是 &i32，需要解引用
    for item in &arr {
        sum = sum + *item;        // *item 得到 i32
    }

    // 方式二：使用模式解构，直接获取 i32
    for &item in &arr {
        sum += item;
    }
    println!("arr: {:?}", arr);   // arr 仍然可用
    println!("sum: {}", sum);

    // ---- 所有权：消耗式迭代 ----
    {
        let mut owned_arr = [10, 20, 30, 40, 50];
        let mut total = 0;
        for item in owned_arr {   // 所有权被移动，item 类型为 i32
            total += item;
        }
        // println!("{:?}", owned_arr); // 错误！owned_arr 已被消耗
        println!("total: {}", total);
    } // 离开作用域，不再需要 owned_arr

    // ---- 数组方法：reverse、sort、split_at ----
    let mut arr2 = [1, 2, 3, 4, 5, 6];
    arr2.reverse();
    arr2.sort();
    println!("arr2: {:?}", arr2);   // [1, 2, 3, 4, 5, 6]

    let (l, r) = arr2.split_at(3);
    println!("left:  {:?}", l);     // [1, 2, 3]
    println!("right: {:?}", r);     // [4, 5, 6]

    // concat 需要先在外部将数组转换为切片引用
    let arr1 = [1, 23, 23, 45, 677, 83]; // 重新定义 arr1，避免已被移动
    let combined: Vec<i32> = [arr1.as_slice(), arr2.as_slice()].concat();
    println!("arr1 + arr2 = {:?}", combined);

    // ---- if 表达式（类似三元运算符） ----
    let age = 20;
    let message = if age < 12 {
        who();          // 执行一个返回单元的函数
        "child"
    } else {
        "young man"
    };
    println!("{}", message);

    // ---- match 控制流 ----
    let num = 1;
    match num {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }

    let word = match num {
        1 => "hello",
        2 => "world",
        _ => "",
    };
    println!("{}", word);

    // 元组匹配
    let tp = (1, -1);
    match tp {
        (0, 0) => println!("zero point"),
        (_, y) if y < 0 => println!("tuple y < 0"),
        (x, _) if x < 0 => println!("tuple x < 0"),
        _ => println!("tuple x and y >= 0"),
    }

    // 数组模式匹配
    let arr1 = [1, 2, 3, 4, 5];
    let is_head_gt_0 = match arr1 {
        [n, _, _, _, _] | [_, n, _, _, _] if n > 0 => true,
        _ => false,
    };
    // 使用 matches! 宏的简化写法
    let is_head_gt_0_2 = matches!(arr1, [n, _, _, _, _] | [_, n, _, _, _] if n > 0);
    println!("{} {}", is_head_gt_0, is_head_gt_0_2);

    // ---- if let 与范围模式 ----
    let t = (2, 4);
    if let (x @ 1..=2, y @ 1..4) = t {
        println!("1 <= x <= 2 且 1 <= y < 4");
    } else {
        println!("not valid");
    }
}

// 单元类型 (unit type) 函数，类似于 C 的 void
fn who() -> () {
    print!("called who() → ");
}
```

---

## 要点说明

### 1. 数组迭代与所有权
- `for item in &arr`：**不可变借用**，`item` 是 `&i32`，需要解引用 `*item` 才能与 `i32` 运算。也可以用 `for &item in &arr` 直接解构得到 `i32`。
- 在 Rust 中，`for item in arr` 会按值消费（Move） 这个数组。为什么现在没报错？因为数组元素是 i32，i32 实现了 Copy trait。所以编译器在迭代时隐式复制了一份数组，原来的 arr 依然有效，后面的 println!("{:?}", arr) 才能正常编译。
- 如果后续还需要用到原数组，务必使用引用迭代（`&arr`）或 `arr.iter()`。

### 2. 数组方法与拼接
- `reverse()`、`sort()` 需要 `&mut self`，因此数组必须为 `mut`。
- `split_at(mid)` 返回两个 **切片**（`&[T]`），不移动原数组。
- 使用 `concat` 拼接两个数组时，参数需要是 `[&[T]; N]`（切片引用的数组），可以用 `arr1.as_slice()` 或直接写 `&arr1[..]`。

### 3. `if` 表达式与单元类型
- Rust 中的 `if` 是表达式，可以返回值，每个分支的返回类型必须一致。
- `who()` 是一个返回单元类型 `()` 的函数，放在 `if` 分支中可以当作副作用执行，最终由字符串字面量提供分支值。

### 4. `match` 与 `matches!`
- `match` 用于模式匹配，必须穷尽所有可能。
- `if` 守卫（`if condition`）可以附加更多条件。
- `matches!` 宏是 `match` 的便捷形式，返回 `bool`。

### 5. `if let` 与范围模式
- `if let` 用于匹配单个模式，搭配 `@` 可以绑定变量并约束范围，如 `x @ 1..=2`。
- 范围模式 `1..=2` 表示闭区间，`1..4` 表示左闭右开。

---

## 注意（常见误区）
- 消费数组所有权的 `for` 循环之后，不能再以任何形式使用该数组，否则编译错误。
- 使用 `&arr` 迭代时，元素是引用，与值相加前必须解引用。
- 确保拼接数组时所有元素类型一致，且数组生命周期足够。
