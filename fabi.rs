fn main() {
    
    println!("result is {}", fabi(3))

} 

fn fabi(x: i32) -> i32 {
    let mut y = 3;
    let mut f1 = 1;
    let mut f2 = 1;
    let mut ret = 0;
    while y <= x {
        ret = f1 + f2;
        y += 1;
        f2 = f1;
        f1 = ret;
    }
    return ret
}

