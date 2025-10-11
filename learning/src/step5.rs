// C++やPythonのクラスのように、フィールドをまとめたデータ構造を定義
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Clone)] 
struct Friend {
    username: String,
    email: String,
}

// タプル構造体: フィールド名がなく、型だけを持つ
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


pub fn step5_structs() {
    println!("[Step5] --- Structs ---");
    
    // --- 構造体のインスタンス化 ---
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // ドット記法でフィールドにアクセス
    user1.email = String::from("anotheremail@example.com");
    println!("[Step5] User email: {}", user1.email);

    // --- 構造体更新記法 ---
    // `..` を使うと、残りのフィールドを他のインスタンスからコピーできる
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // username, active, sign_in_countをuser1からコピー
    };
    // この時、user1のString型フィールド(username)はuser2にムーブされる。
    // そのため、user1のusernameは使えなくなるが、activeやsign_in_countのような
    // Copyトレイトを持つ型はコピーされるだけなので、user1全体が無効になるわけではない。
    // println!("[Step5] user1.username is moved: {}", user1.username); // コンパイルエラー！
    println!("[Step5] user2.username: {}", user2.username);

    // 構造体自体のディープコピーは可能だが、コストが高い
    // その高いコストを自覚的にするため、アトリビュート `#[derive(Clone)]` を使用する必要がある
    let friend1 = Friend {
        username: String::from("taro"),
        email: String::from("friend1@example.com"),
    };
    let mut friend2 = friend1.clone(); // clone()メソッドを使ってディープコピー
    friend2.username = String::from("jiro");
    println!("[Step5] friend1: {}, friend2: {}", friend1.username, friend2.username);

    // --- タプル構造体のインスタンス化 ---
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("[Step5] Color: R={}, G={}, B={}", black.0, black.1, black.2);

    // --- メソッドの呼び出し ---
    println!("[Step5] --- Methods ---");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "[Step5] The area of the rectangle is {} square pixels.",
        rect1.area() // メソッド呼び出し
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("[Step5] Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    // --- 関連関数の呼び出し (C++の静的メソッドのようなもの) ---
    // `::` 構文で呼び出す
    let sq = Rectangle::square(25);
    println!("[Step5] Created a square: width={}, height={}", sq.width, sq.height);
}


// 構造体のためのメソッドを定義するには `impl` (implementation) ブロックを使う
#[derive(Debug)] // デバッグ出力のために必要なおまじない
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle構造体のための実装ブロック
impl Rectangle {
    // &self: 構造体インスタンスの不変の借用
    // C++/Pythonの `this` や `self` に相当
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 他のRectangleインスタンスを不変で借用するメソッド
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // &self を第一引数に取らない関数は「関連関数 (Associated Function)」と呼ばれる
    // これはインスタンスではなく、型自体に関連付けられる (静的メソッド)
    // コンストラクタとしてよく使われる
    fn square(size: u32) -> Self { // `Self` は `Rectangle` のエイリアス
        Self {
            width: size,
            height: size,
        }
    }
}