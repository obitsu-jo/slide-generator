pub fn step1_variables() {
    println!("[Step1] --- Variables ---");
    // デフォルトで不変
    let x = 5;
    println!("x = {}", x);
    // x = 6; // これはコンパイルエラー

    // `mut`で可変にできる
    let mut y = 10;
    println!("y = {}", y);
    y = 20;
    println!("y is now {}", y);
}