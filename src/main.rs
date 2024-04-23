use std::env;

fn main() {
    println!("Start!");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a STL file name");
        return;
    } else if args.len() < 3 {
        println!("Please provide output file name");
        return;
    }

    let path_to_stl = &args[1];
    let path_to_output = &args[2];

    // バイナリモードで出力するかどうか
    let mut is_binary = false; // Make the variable mutable
                               // is_binary変数を定義
    if args.len() > 3 {
        is_binary = true;
    }

    // 出力ファイルの拡張子がglbの場合はバイナリモードで出力する
    if path_to_output.ends_with(".glb") {
        is_binary = true;
    }

    // STLファイルが存在しない場合，エラーを出力して終了
    if !std::path::Path::new(path_to_stl).exists() {
        println!("STL File not found: {}", path_to_stl);
        return;
    }

    // 現在のディレクトリを取得
    let current_dir = std::env::current_dir().unwrap();

    // 出力フォルダが存在しない場合，今のディレクトリに作成
    if !is_binary {
        if !std::path::Path::new(path_to_output).exists() {
            std::fs::create_dir_all(format!("{}/{}", current_dir.display(), path_to_output))
                .expect("Failed to create output folder");
        }
    }

    println!("path_to_stl: {}", path_to_stl);
    println!("path_to_output: {}", path_to_output);
    println!("is_binary: {}", is_binary);
}
