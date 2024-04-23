use core::num;
use std::env;
use std::io::Read;

fn stl_to_gltf(path_to_stl: &String, path_to_output: &String, is_binary: bool) {
    println!("path_to_stl: {}", path_to_stl);
    println!("path_to_output: {}", path_to_output);
    println!("is_binary: {}", is_binary);

    // ヘッダ情報
    let header_bytes = 80;
    let unsigned_long_int_bytes = 4;
    let float_bytes = 4;
    let vec3_bytes = 4 * 3;
    let spacer_bytes = 2;
    let num_vertices_in_face = 3;

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<f32> = Vec::new();

    let mut path_to_output_bin = String::new();
    let mut path_to_output_gltf = String::new();

    // 出力ファイルパス
    if !is_binary {
        path_to_output_bin = format!("{}/{}", path_to_output, "out.bin");
        path_to_output_gltf = format!("{}/{}", path_to_output, "out.gltf");
    } else {
        path_to_output_bin = path_to_output.to_string();
    }
    println!("path_to_output_bin: {}", path_to_output_bin);
    println!("path_to_output_gltf: {}", path_to_output_gltf);

    // STLファイルをバイナリモードで開く
    let file = std::fs::File::open(path_to_stl).expect("Failed to open file");
    let mut reader = std::io::BufReader::new(file);

    // ファイルのバイナリを読み込む
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Failed to read file");

    // バイナリを16進数でprint
    for i in 0..buffer.len() {
        print!("{:02x} ", buffer[i]);
    }

    println!("");

    // 先頭からheader_bytesを消去して，16進数でprint
    buffer = buffer.split_off(header_bytes);
    for i in 0..buffer.len() {
        print!("{:02x} ", buffer[i]);
    }

    println!("");

    let num_faces_bytes = buffer[0..unsigned_long_int_bytes].to_vec();
    let number_faces = u32::from_le_bytes([
        num_faces_bytes[0],
        num_faces_bytes[1],
        num_faces_bytes[2],
        num_faces_bytes[3],
    ]) as usize;

    println!("number_faces: {}", number_faces);

    // STLファイルのバイト数を取得
    let stl_assume_bytes = header_bytes
        + unsigned_long_int_bytes
        + number_faces * (vec3_bytes * 3 + spacer_bytes + vec3_bytes);
    println!("stl_assume_bytes: {}", stl_assume_bytes);
}

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

    let path_to_stl: &String = &args[1];
    let path_to_output: &String = &args[2];

    // バイナリモードで出力するかどうか
    let mut is_binary: bool = false; // Make the variable mutable
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

    stl_to_gltf(path_to_stl, path_to_output, is_binary)
}
