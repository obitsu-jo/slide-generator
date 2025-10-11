pub fn step4_references_and_borrowing() {
    println!("[Step4] --- References and Borrowing ---");

    let s1 = String::from("hello");

    // `&s1` はs1への「参照」を作成します。s1の所有権はムーブしません。
    // この操作を「借用(borrowing)」と呼びます。
    let len = calculate_length(&s1);

    println!("[Step4] The length of '{}' is {}.", s1, len);
    // s1は所有権を渡していないので、このスコープでもまだ有効です！

    // --- 可変な参照 (Mutable References) ---
    println!("[Step4] --- Mutable References ---");
    let mut s2 = String::from("hello");
    println!("[Step4] Before change: {}", s2);
    
    // `&mut s2` で可変な参照を作成します。
    // これにより、関数内で値を変更できます。
    change(&mut s2);

    println!("[Step4] After change: {}", s2);

    // --- 借用のルール ---
    // ここがRustの安全性の核となる部分です。
    // 1. あるスコープ内で、特定のデータに対しては「1つの可変参照」か「複数の不変参照」のどちらか一方しか存在できない。
    // 2. 参照は常に有効でなければならない (ダングリングポインタの防止)。

    // --- ルール1の実例 ---
    let mut s3 = String::from("hello");

    let r1 = &s3; // OK: 不変参照
    let r2 = &s3; // OK: 複数の不変参照は問題ない
    // let r3 = &mut s3; スコープを抜ける前に可変な参照を作成するとエラーになる
    println!("[Step4] r1 = {}, r2 = {}", r1, r2);

    // r1, r2がスコープを抜ければ、可変な参照を作成できる
    let r3 = &mut s3;
    println!("[Step4] r3 = {}", r3);

    // --- ダングリングポインタの防止 ---
    // 以下のdangle()関数のコメントアウトされたコードはコンパイルエラーになります。
    // let reference_to_nothing = dangle();
    // Rustコンパイラは、無効なメモリを指す参照が作られることを未然に防ぎます。
}

// 引数にStringへの参照(`&String`)を取る
// この関数は値の所有権を得ない
fn calculate_length(s: &String) -> usize {
    s.len()
} // sはスコープを抜けるが、所有権を持っていないので何も破棄されない

// 可変な参照(`&mut String`)を取る
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
// ダングリング参照を生成しようとする関数（コンパイルエラーになる）
fn dangle() -> &String { // dangleはStringへの参照を返す
    let s = String::from("hello"); // sは新しいString

    &s // sへの参照を返す
} // ここでsはスコープを抜け、メモリは解放される。返された参照は無効なメモリを指すことになる！
*/