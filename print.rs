fn main() {
    println!("Hello, Rust!");
    let (name, age) = ("Alice", 12);
    let x = 10;
    println!(
        "My name is {my_name}, \
         I am {my_age} years old. \
         x = {x};",
        my_age = age,
        my_name = name
    );

    // rust clippy fmt fix

    let x = 3.14; // this is considered as a *double* float; aka `f64`;
    let real_value = std::f64::consts::PI;
    // `:.x` keep x decimal place(s) in output
    println!("{:.2}, {:.6}, {}", real_value, real_value, real_value as i32);
    // 3.14, 3.141593, 3

    let decimal = 23842134;
    // `:X(x)` indicate that print decimal number in hex
    // `#` indicate that print message will carry a prefix, like 0x or 0b
    println!("{:#X}, {:#x}, {:x}", decimal, decimal, decimal);
    // 0x16BCD56, 0x16bcd56, 16bcd56

    let binary = 1024;
    // `:b` indicate that print in binary
    println!("{:#b}, {:b} ", binary, binary);
    // 0b10000000000,10000000000

    // If you prefix a string with `r`,
    // the backslashes in string won't be escaped.
    println!(r"C:\Apps\mihoyo\Zenless Zone Zero");
    // `r` prefix ignore `\"`, if you don't want this,
    // use `r#`, you can write multiple `#` symbols,
    // remember that `#` should appear in pairs.
    println!(r######"Hi, this \ " ""  "" \" , so many `"` "######);
}
