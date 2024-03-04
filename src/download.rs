pub static MIRRORS: [&'static str; 3] = [
    "https://setup.rbxcdn.com",
    "https://setup-cfly.rbxcdn.com",
    "https://s3.amazonaws.com/setup.roblox.com",
];

// screw you

// fn downloader() {
// print!("Downloading roblox from selected/closest mirror...");
// let oworesponse = reqwest::blocking::get("https://setup.rbxcdn.com/version-bca459bcd1854ce4-RobloxPlayerInstaller.exe").expect("request nailed >~<", cdn, installer);
// let body = oworesponse.text().expect("body is heckin' invalid owo uwu");
// let mut out = File::create("/home/breaktrace/livelylive/RobloxPlayerInstaller.exe").expect("failed to create file");
// let bytes = body.as_bytes();
// io::copy(&mut bytes.as_ref(), &mut out).expect("failed to copy content");
//
// let output = Command::new("wine")
// .arg("/home/breaktrace/livelylive/RobloxPlayerInstaller.exe")
// .output()
// .expect("Failed to execute command");
//
// if output.status.success() {
// let stdout = String::from_utf8_lossy(&output.stdout);
// println!("Running installer...");
// } else {
// let stderr = String::from_utf8_lossy(&output.stderr);
// eprintln!("Error: {}", stderr);
// }
// }