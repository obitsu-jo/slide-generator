pub fn step2_functions_and_control_flow() {
    println!("[Step2] --- Functions and Control Flow ---");
    // --- 関数呼び出し ---
    another_function(5, 'h');

    // --- 式 (Expression) ---
    // ブロック式
    let y = {
        let x = 3;
        x + 1 // セミコロンを付けない最後の行が、このブロックの戻り値になる
    };
    println!("The value of y is: {}", y); // yは4になる

    // --- 戻り値のある関数 ---
    let five = returns_five();
    println!("The value of five is: {}", five);

    let six = plus_one(five);
    println!("The value of six is: {}", six);

    // --- 制御フロー: if式 ---
    let number = 6;

    // C++/Pythonと似ているが、()は不要。
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }
    
    // ifも式なので、その結果を直接変数に入れることができます。
    // 全ての分岐で同じ型を返す必要があります。
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of num is: {}", num);

    // --- 制御フロー: ループ ---
    // Rustには3種類のループがあります: loop, while, for

    // 1. loop: 無限ループ
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // break文でループを抜け、値を返すこともできる
            break counter * 2;
        }
    };
    println!("The loop result is {}", result);

    // 2. while: 条件付きループ (C++/Pythonと同じ)
    let mut i = 3;
    while i != 0 {
        println!("{}!", i);
        i -= 1;
    }
    println!("LIFTOFF!!!");

    // 3. for: コレクションの要素を反復処理 (Pythonの for-in に非常に近い)
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() { // `.iter()` はコレクションの各要素への参照を順番に返す
        println!("the value is: {}", element);
    }
    
    // 特定の回数だけ繰り返す (Pythonの range)
    // 1..4 は 1, 2, 3 の範囲(Range)を生成
    for number in 1..4 {
        println!("{}!", number);
    }
}

// --- ヘルパー関数 ---

// 引数を取る関数
// C++と同様、引数の型注釈は必須
fn another_function(x: i32, unit_label: char) {
    println!("The measurement is: {}{}", x, unit_label);
}

// 戻り値のある関数
// `->` の後に戻り値の型を記述
fn returns_five() -> i32 {
    5 // セミコロンなし -> この値が返される
}

fn plus_one(x: i32) -> i32 {
    x + 1 // セミコロンを付けると「文」になり、何も返さない()型になってしまうので注意
}