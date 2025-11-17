use qrcode::{QrCode, render::unicode};
use std::fs;

fn main() {
    let text = "https://play.limitlesstcg.com/tournament/685192ba27d8bc24cf255b9d/player/numatinale/decklist";
    let code = QrCode::new(text).unwrap();

    let image = code.render::<unicode::Dense1x2>()
        .quiet_zone(false)
        .build();

    let output_path = "qrcode.txt";
    fs::write(output_path, image).expect("Unable to write file");

    println!("QR code written to {}", output_path);
}

