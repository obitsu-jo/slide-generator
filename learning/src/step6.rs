// --- Enumの定義 ---

// IPアドレスの種類を表現するEnum
// 各バリアントは独立した値
enum IpAddrKind {
    V4,
    V6,
}

// Enumのバリアントにデータを持たせる
// これがRustのEnumの強力な点
// 同じIpAddrという型でありながら、V4は4つのu8を、V6はStringを持つことができる
#[derive(Debug)] // デバッグ表示用
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// さまざまな型のデータをバリアントに持たせることも可能
#[derive(Debug)]
enum Message {
    Quit, // データなし
    Move { x: i32, y: i32 }, // 匿名の構造体を持つ
    Write(String), // Stringを持つ
    ChangeColor(i32, i32, i32), // 3つのi32を持つ
}

// Enumにもimplブロックでメソッドを定義できる
impl Message {
    fn call(&self) {
        println!("[Step6] Message received: {:?}", self);
    }
}

pub fn step6_enums_and_match() {
    println!("[Step6] --- Enums ---");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("[Step6] Home IP: {:?}", home);
    println!("[Step6] Loopback IP: {:?}", loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    // --- `match` 制御フロー演算子 ---
    println!("[Step6] --- match ---");
    // `match`は、ある値がEnumのどのバリアントに一致するかをチェックし、
    // それに応じてコードを実行する。C/C++のswitch文に似ているが、より強力。
    let coin = Coin::Penny;
    println!("[Step6] Value of Penny: {} cent(s)", value_in_cents(coin));
    
    let quarter = Coin::Quarter(UsState::Alaska);
    println!("[Step6] Value of Alaska Quarter: {} cent(s)", value_in_cents(quarter));


    // --- Option<T> Enum: Nullの代わり ---
    // Rustには他の多くの言語にある `null` や `None` がありません。
    // その代わりに、値が存在しない可能性を表現するために `Option<T>` Enumを使います。
    // enum Option<T> {
    //     None,       // 値がないことを示す
    //     Some(T),    // 値 T があることを示す
    // }
    println!("[Step6] --- Option<T> ---");
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; // `None`を使うには型を明示する必要がある

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // コンパイルエラー！
    // error[E0277]: cannot add `Option<i8>` to `i8`
    // Option<T>はTとは別の型なので、直接計算できない。
    // これにより、「nullかもしれない値」をうっかり使ってしまうミスを防ぐ。
    
    // Option<T>から値を取り出すには、`match`を使うのが安全
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("[Step6] five plus one is: {:?}", six); // Some(6)
    println!("[Step6] none plus one is: {:?}", none); // None

    // --- `if let` 構文 ---
    // `match`は全てのパターンを網羅する必要があるが、特定の1つのパターンにだけ
    // 注目したい場合は `if let` が便利
    let config_max = Some(3u8);
    // `config_max`が`Some(max)`というパターンにマッチする場合だけブロック内を実行
    if let Some(max) = config_max {
        println!("[Step6] The maximum is configured to be {}", max);
    }
    // `else`も付けられる
    let mut count = 0;
    let coin_to_check = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin_to_check {
        println!("[Step6] State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

// --- ヘルパー関数 ---

fn route(_ip_kind: IpAddrKind) {}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...など
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarterバリアントは州の情報も持つ
}

// `match`は網羅的(exhaustive)でなければならない。
// つまり、全ての可能性を考慮しないとコンパイルエラーになる。
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("[Step6] Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // バリアント内のデータを取り出して使うことができる
            println!("[Step6] State quarter from {:?}!", state);
            25
        }
        // `_`（アンダースコア）は、全てのその他の値にマッチするプレースホルダー
        // もしこれをコメントアウトすると、`match`が網羅的でなくなりエラーになる
        // _ => 0,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), // `Some`から値`i`を取り出して計算し、`Some`で包んで返す
    }
}