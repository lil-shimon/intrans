use std::io;
mod calc;

fn main() {
    println!("16進数を入力してください");

    // 標準入力をbindするための変数
    let mut hex = String::new();
    io::stdin().read_line(&mut hex).expect("failed");
    calc::hex_to_dec(&hex);
}
