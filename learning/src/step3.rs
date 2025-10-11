// 所有権のルール
// 1. Rustの各値は、所有者(owner)と呼ばれる変数を持つ。
// 2. いかなる時も、所有者は一人だけ。
// 3. 所有者がスコープから外れたら、値は破棄(drop)される。

pub fn step3_ownership() {
    println!("[Step3] --- Ownership Basics ---");
    
    // --- スコープ ---
    {
        // sはまだ有効ではない
        let s = "hello"; // この行からsは有効になる
        // sを使って処理
        println!("[Step3] s in inner scope: {}", s);
    } // このスコープの終わりでsは無効になる
    // println!("{}", s); // ここでsを使おうとするとコンパイルエラー

    // --- String型 ---
    // これまでのリテラル文字列(&'static str)と違い、String型はヒープに確保され、可変であり、コンパイル時にサイズが不明なテキストを扱うのに使います。
    let mut s = String::from("hello");
    s.push_str(", world!"); // 文字列を追加
    println!("[Step3] {}", s);

    // --- 所有権のムーブ (Move) ---
    // ここが最も重要なポイントです！
    println!("[Step3] --- Move ---");
    let s1 = String::from("hello");
    let s2 = s1; // s1の所有権がs2に「ムーブ」する

    // println!("[Step3] s1 is: {}", s1); 
    // 上の行のコメントを外すとコンパイルエラー！
    // error[E0382]: borrow of moved value: `s1`
    // s1はs2に所有権を渡したため、もはや無効な変数になっています。
    // これにより、同じメモリ領域を2つの変数が解放しようとする「二重解放」エラーを未然に防ぎます。
    println!("[Step3] s2 is: {}", s2); // s2は有効なのでOK

    // --- データのコピー (Copy) ---
    // i32のようなスタック上にのみ存在するデータは挙動が異なります。
    println!("[Step3] --- Copy ---");
    let x = 5;
    let y = x; // i32はCopyトレイトを持つため、値がコピーされる

    println!("[Step3] x = {}, y = {}", x, y); // xもyも両方有効！
    // このような単純な型では、所有権のムーブは起きません。

    // --- データのクローン (Clone) ---
    // ヒープ上のデータ(Stringなど)を本当にコピーしたい場合は、clone()メソッドを使います。
    println!("[Step3] --- Clone ---");
    let s3 = String::from("hello");
    let s4 = s3.clone(); // s3のデータをディープコピーしてs4が所有する

    println!("[Step3] s3 = {}, s4 = {}", s3, s4); // s3もs4も両方有効！

    // --- 所有権と関数 ---
    println!("[Step3] --- Ownership and Functions ---");
    let s5 = String::from("takes ownership");
    
    // s5はtakes_ownership関数にムーブされ、このスコープでは無効になる
    takes_ownership(s5);
    // println!("{}", s5); // ここで使うとエラー

    let x5 = 5;
    // x5はmakes_copy関数にコピーされ、このスコープでも引き続き有効
    makes_copy(x5);
    println!("[Step3] x5 is still valid: {}", x5);

    // --- 戻り値とスコープ ---
    // 関数は値を返すことで、所有権を呼び出し元に返す(ムーブする)ことができる
    let s6 = gives_ownership();
    println!("[Step3] Got ownership of '{}' back", s6);

    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7);
    println!("[Step3] Took and gave back: '{}'", s8);
    // println!("{}", s7); // s7はムーブされたので無効
}

// この関数はStringの所有権を得て、スコープの終わりに破棄(drop)する
fn takes_ownership(some_string: String) {
    println!("[Step3] (in function) {}", some_string);
} // ここでsome_stringはスコープを抜け、dropが呼ばれる。メモリが解放される。

// この関数はi32のコピーを受け取る
fn makes_copy(some_integer: i32) {
    println!("[Step3] (in function) {}", some_integer);
} // ここでsome_integerはスコープを抜けるが、何も特別なことは起こらない。

// この関数は戻り値の所有権を呼び出し元にムーブする
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

// この関数はStringの所有権を奪い、それをまた呼び出し元に返す
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}