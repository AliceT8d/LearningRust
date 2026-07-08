# Rust 字符串 (String 与 &str) 与所有权笔记

## 1. 字符串的创建

在 Rust 中，主要有两种字符串类型：字符串字面量 `&str` 和动态分配的 `String`。

```rust
// 1. 字符串字面量 (&str)
// 硬编码在二进制文件中，不可变，拥有静态生命周期
let greeting = "Hello, world!"; 
println!("{}", greeting);

// 2. 动态字符串 (String)
// 存储在堆上，可变，支持动态扩容

// 创建空字符串 (不支持 String::new("xxx") 这种语法)
let empty_str = String::new(); 

// 从字面量创建 (调用结构体的关联函数/静态方法)
let from_str = String::from("Hello"); 

// 通过 to_string() 方法创建 (实现了 Display trait 的类型都可以调用)
let to_str = "Hello".to_string(); 
```

---

## 2. 所有权：移动 (Move) 与 克隆 (Clone)

`String` 类型在堆上分配内存，因此它**没有实现 `Copy` trait**，赋值操作会触发所有权转移。

```rust
let s1 = String::from("s1");
let s3 = String::from("s3");

// 【移动 Move】
// 相当于 C++ 的 std::move(s1)，将 s1 的所有权移交给 s2
let s2 = s1; 
// 此时 s1 已经失效，不再可用
// println!("{}", s1); // ❌ 编译报错：borrow of moved value: `s1`

// 【克隆 Clone】
// 深拷贝，在堆上分配新的内存并复制数据
let s2_clone = s3.clone(); 
println!("s3: {}, s2_clone: {}", s3, s2_clone); // s3 依然可用
```

---

## 3. 字符串切片 (&str) 与 UTF-8 编码

### 核心概念
- `String` 内部以 **UTF-8** 格式存储字节。
- 由于 UTF-8 是变长编码（1~4个字节），每个字符没有固定的字节边界。
- 因此，**不能将字符串切片 `&str` 理解为 `&[char]`**，它实际上是 `&[u8]`（字节切片）。

### 切片操作
```rust
let greeting = "Hello, world!";

// 获取字节切片 [0..5) 
// 注意：这里使用的是【字节索引】而不是字符索引。
// 对于纯 ASCII 字符没问题，但如果包含中文等多字节字符，随意切片可能会导致 panic（不在字符边界上）。
let hello = &greeting[0..5]; 
println!("切片: {}", hello); // 输出: Hello

// 将 String 转换为 &str 切片
let s2 = String::from("Rust");
let h: &str = s2.as_str();       // 不可变切片
// let h_mut: &mut str = s2.as_mut_str(); // 可变切片 (需要 s2 是 mut 的)
```

---

## 4. 字符串拼接

### 使用 `+` 运算符
`String` 实现了 `Add` trait，其签名大致为 `fn add(self, other: &str) -> String`。

```rust
let s4 = "hello".to_string();
let s3 = String::from(" world");

// s4 + &s3 
// 1. 左侧 s4 会被【消耗】（转移所有权），Rust 会复用 s4 的底层缓冲区（必要时扩容）。
//    这样做是为了避免每次拼接都重新分配内存，防止 O(n²) 的性能问题。
// 2. 右侧需要 &str。这里传入 &s3，Rust 会通过 Deref 隐式转换（&String -> &str）。
let s5 = s4 + &s3; 

// println!("{}", s4); // ❌ 报错，s4 的所有权已经被消耗
println!("拼接结果: {}", s5);
```

### 使用 `format!` 宏 (不消耗所有权)
如果不想丢失原有字符串的所有权，推荐使用 `format!` 宏。

```rust
let s6 = "hello".to_string();
let s7 = String::from(" rust");

// format! 宏只是借用参数，不会消耗 s6 和 s7 的所有权
let s8 = format!("{}{}", s6, s7);

println!("s6: {}, s7: {}, s8: {}", s6, s7, s8); // s6 和 s7 依然可用
```

---

## 5. 完整可运行代码示例

```rust
fn main() {
    // 1. 创建
    let greeting = "Hello, world!";
    println!("greeting: {}", greeting);

    let _empty = String::new();
    let _from = String::from("Hello");
    let _to = "Hello".to_string();

    // 2. 所有权
    let s1 = String::from("s1");
    let s3 = String::from("s3");
  
    let s2 = s1; // s1 moved
    let s2_clone = s3.clone(); // s3 cloned
    println!("s2: {}, s2_clone: {}", s2, s2_clone);

    // 3. 切片
    let hello = &greeting[0..5];
    println!("slice: {}", hello);

    let h = s2.as_str();
    println!("as_str: {}", h);

    // 4. 拼接 (+ 运算符)
    let s4 = "hello".to_string();
    let s5 = s4 + &s3; // s4 is moved, s3 is borrowed
    println!("s5 (+): {}", s5);

    // 5. 拼接 (format! 宏)
    let s6 = "hello".to_string();
    let s7 = format!("{}{}", s6, s3); // s6 is NOT moved
    println!("s6: {}, s7 (format): {}", s6, s7);
}
```



这是一个非常深刻且直击 Rust 字符串设计核心的问题！

简单来说：**`mut` 关键字对于 `String` 的意义，不是让你“就地替换某个索引位置的字符”，而是允许你“改变字符串的整体结构、长度或重新分配内存”（例如追加、插入、删除、清空）。**

要彻底理解这个问题，我们需要从 Rust 字符串的底层设计说起。

---

### 1. 为什么不能通过索引修改字符？（UTF-8 的“锅”）

在 Rust 中，字符串（`String` 和 `&str`）底层是 **UTF-8 编码的字节序列**。UTF-8 是一种**变长编码**：
* 英文字母占 1 个字节。
* 带音标的拉丁字母占 2 个字节。
* 中文字符通常占 3 个字节。
* Emoji 表情占 4 个字节。

因为字符长度不固定，Rust 的**索引指的是“字节索引”，而不是“字符索引”**。
如果你允许通过索引直接修改字符（比如 `s[0] = '中'`），会引发严重的内存和逻辑问题：
1. **长度变化**：如果你把一个 1 字节的 `'a'` 替换成 3 字节的 `'中'`，字符串的总长度就变了。但字符串在内存中是连续分配的，就地变长会覆盖后面的内存。
2. **破坏编码**：如果你只修改了多字节字符的其中几个字节，会导致整个字符串变成无效的 UTF-8，这违反了 Rust “内存安全”的核心原则。

因此，Rust 编译器直接**禁止了 `s[index] = 'x'` 这种语法**。

---

### 2. 那 `mut` 对于 `String` 的意义是什么？

虽然不能“就地替换单个字符”，但一个可变的字符串（`mut String`）允许你调用那些**会改变字符串内部缓冲区（长度、容量、内容）的方法**。

以下是 `mut` 赋予 `String` 的核心能力：

#### A. 追加内容 (改变长度)
```rust
let mut s = String::from("Hello");
s.push('!');           // 追加单个字符
s.push_str(", Rust!"); // 追加字符串切片
// s 现在是 "Hello, Rust!"
```

#### B. 插入与删除 (改变结构)
```rust
let mut s = String::from("Hello World");
s.insert(5, ',');      // 在字节索引 5 处插入字符 -> "Hello, World"
s.remove(5);           // 删除字节索引 5 处的字符 -> "Hello World"
s.pop();               // 删除最后一个字符 -> "Hello Worl"
```

#### C. 清空与截断
```rust
let mut s = String::from("Hello");
s.truncate(3);         // 截断到 3 个字节 -> "Hel"
s.clear();             // 清空字符串 -> ""
```

**总结：** `mut` 的意义在于允许字符串**动态增长、缩小或重新排列其内部的字节数据**。

---

### 3. 如果我非要修改“某个位置的字符”怎么办？

如果你确实需要替换字符串中间的某一部分，Rust 提供了安全的方法，这些方法同样需要 `mut`：

#### 方法一：使用 `replace_range` (推荐)
你可以指定一个**字节范围**，用新的字符串替换它。这个方法会自动处理长度变化和内存重新分配。
```rust
let mut s = String::from("Hello World");
// 将字节索引 0..5 ("Hello") 替换为 "Hi"
s.replace_range(0..5, "Hi"); 
println!("{}", s); // 输出: "Hi World"
```

#### 方法二：转为字符向量（适用于复杂字符处理）
如果你需要按“字符”（而不是字节）来精确修改，可以先转成 `Vec<char>`，修改后再拼回来（注意：性能较差，会重新分配内存）。
```rust
let s = String::from("你好世界");
let mut chars: Vec<char> = s.chars().collect();
chars[2] = 'R'; // 修改第三个字符
let new_s: String = chars.into_iter().collect();
println!("{}", new_s); // 输出: "你好R界"
```

---

### 4. ⚠️ 新手极易混淆的陷阱：`mut &str` vs `&mut String`

在讨论字符串的 `mut` 时，必须区分**变量绑定的可变性**和**数据本身的可变性**。

#### 情况 A：`let mut s = "hello";` （可变变量，不可变数据）
这里的 `"hello"` 是字符串切片 `&str`（指向只读内存）。
`mut` 修饰的是**指针 `s` 本身**，意味着你可以让 `s` 指向另一个字符串，但**不能修改它指向的数据**。
```rust
let mut s = "hello";
s = "world"; // ✅ 合法：改变了 s 的指向
// s.push('!'); // ❌ 报错：&str 没有 push 方法，数据不可变
```

#### 情况 B：`let s: &mut String` （不可变变量，可变数据）
这里的 `s` 是一个指向 `String` 的可变引用。
你**不能**让 `s` 指向别的字符串，但**可以**通过 `s` 修改原字符串的内容。
```rust
let mut original_string = String::from("hello");
let s = &mut original_string; // s 是不可变变量，但它是可变引用

s.push_str(" world"); // ✅ 合法：修改了 original_string 的内容
// s = &mut another_string; // ❌ 报错：s 变量本身不是 mut，不能改变指向
```

### 总结
在 Rust 中，字符串不能通过索引修改，是为了**保证 UTF-8 编码的绝对安全和内存的连续性**。
`mut` 关键字对于字符串的意义，是**解锁字符串的“动态变更能力”**（增、删、整体替换、扩容），而不是提供不安全的“就地单字符修改”权限。
