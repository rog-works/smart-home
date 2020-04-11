fn main() {
    println!("Hello, world!");

    syntax_and_semantics();
}

// # 4. シンタックスとセマンティクス
// ## 変数束縛
fn syntax_and_semantics() {
    let _x1 = 5; // XXX 接頭辞に「_」を付けると未使用でも警告されない

    // ### パターン
    let (_x2, _y2) = (1, 2);

    // ### 型アノテーション
    let _x3: i32 = 5;
    let _x4 = 5; // x4: i32 型推論が効くからあえて書かなくて良い

    // ### 可変性
    let _x5 = 5;
    // x = 10; デフォルトでimutableなのでcompile error
    let mut _x6 = 5;
    _x6 = 10; // ok

    // ### 束縛を初期化する
    let _x7: i32; // 宣言だけなら警告止まり

    // println!("The value of x7 is: {}", x7); // 初期化しないまま使用するとcompile error

    // ### スコープとシャドーイング
    let x8: i32 = 17;
    {
        let y8: i32 = 3;
        println!("The value of x8 is {} and value of y8 is {}", x8, y8);
    }
    // println!("The value of x8 is {} and value of y8 is {}", x8, y8); // yがブロックスコープ外なのでcompile error

    let x9: i32 = 8;
    {
        println!("{}", x9); // 8
        let x9 = 12; // 同じ名前で束縛を上書きできる
        println!("{}", x9); // 12
    }
    println!("{}", x9); // 8
    let x9 = 42;
    println!("{}", x9); // 42

    let mut _x10: i32 = 1;
    _x10 = 7;
    let _x10 = _x10; // imutableになり、7に束縛

    let _y10 = 4;
    let _y10 = "I can also be bound to text!"; // 違う型に変わる
}
