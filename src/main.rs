use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("エラー: ファイルパスを指定してください。");
        eprintln!("使い方: cargo run -- <ファイルパス>");
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("読み込み中: {}", file_path);

    let content = fs::read_to_string(file_path)
        .expect("ファイルの読み込みに失敗しました。ファイルが存在するか確認してください。");

    let slides: Vec<&str> = content.split("---").map(|s| s.trim()).collect();

    println!("--- スライドの枚数: {} 枚 ---", slides.len());

    for (index, slide) in slides.iter().enumerate() {
        println!("\n=== スライド {} ===", index + 1);
        println!("{}", slide);
    }
}
