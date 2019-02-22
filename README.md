# rust-learning-note

Rust语言学习笔记

### 先从示例开始
main.rs
fn main() {
    println!("Hello, world!");
}

编译方法 `rustc main.rs `
> ! 意味着调用的是宏而不是普通函数 

### Cargo 系统构建和包管理工具

1. 新建项目 **{cargo new hellocargo --bin}**

    目录结构如下
         ├── Cargo.toml
         └── src
                 └── main.rs

2.  编译  _**cargo build**_
3.  检查（编译但不生成可执行文件）**cargo check**
4.  构建并运行  **cargo run**
5.  发布 **cargo build --release**

### 基础编程概念

1. 变量和常量

    :: let::声明变量，变量声明后将不能改变，除非通过 ::mut::
    ```rust
    fn main() {
        let x = 5;
        println!("The value of x is: {}", x);
        x = 6; // wrong
        println!("The value of x is: {}", x);
        let mut x = 5;
        println!("The value of x is: {}", x);
        x = 6; // right
        println!("The value of x is: {}", x);
    }
    ```
    声明常量使用 ::const:: 关键字而不是:: let::，并且 必须 注明值的类型。
    const MAX_POINTS: u32 = 100_000;
2. 数据类型
    除了常见的整型，浮点型，布尔型之外，还有元组类型。
    Rust的元组和Python有类似之处。

    ```rust
    fn main() {
        let tup = (500, 6.4, 1);
    
        let (x, y, z) = tup;
    
        println!("The value of y is: {}", y);
    
        let x: (i32, f64, u8) = (500, 6.4, 1);
    
        let five_hundred = x.0;
    
        let six_point_four = x.1;
    
        let one = x.2;
    }
    ```

### 函数

先来看一个具体的示例

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

- 函数声明的关键字 `fn`
- 传参 `x: i32` x 为变量名， i32 为类型, 传参时**必须**声明参数类型
- 返回值 仅制定类型 `-> i32` 结尾处的语句 `x + 1` 如果加上分号将不能正确执行，这里有语句和表达式的区别
- Rust语句不返回值，表达式可以
- 使用 `return` 关键字可以提前返回

### 控制流

- **if/else**

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```
**if/else**关键字之后不需要添加括号，也不需要像Python那样加冒号
