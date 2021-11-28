const EXAMPLE: &str = "AB8";

/// 16進数を10進数にする関数
pub fn hex_to_dec() {
    let org = &EXAMPLE;

    // 桁数
    let len = org.len();

    // 10進数表示結果
    let mut dec = 0;

    // カウント
    let mut count = 1;

    // 一文字ずつにする
    for i in org.chars() {
        let base: i32 = 16;

        // 何乗するか
        let mul: u32 = (len - count) as u32;

        // 桁数を増加
        count = count + 1;

        // charを文字列にする
        let i_str = i.to_string();

        // A B C D E Fの場合と切り分ける
        if &i_str == "A" || &i_str == "B" || &i_str == "C" || &i_str == "D" || &i_str == "E" || &i_str == "F" {
            let mut num = 0;

            //TODO: matchに書き換える
            if &i_str == "A" {
                num = 10;
            }

            if &i_str == "B" {
                num = 11;
            }

            if &i_str == "C" {
                num = 12;
            }

            if &i_str == "D" {
                num = 13;
            }

            if &i_str == "E" {
                num = 14;
            }

            if &i_str == "F" {
                num = 15;
            }

            // 計算
            dec = dec + num * base.pow(mul);
        } else {

            // 32bit integerに変換
            let num: i32 = i_str.parse().unwrap();

            // 計算
            dec = dec + num * base.pow(mul);
        }
    }

    println!("{}", dec);
}