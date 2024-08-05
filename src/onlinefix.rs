use std::{env, fs::{self, File}, io::Write};
use byteorder::{ByteOrder, LittleEndian};
use md5::compute;

pub fn check_shortcuts_vdf() -> File
{
    let path = "/home/amiroof/.steam/steam/userdata/1043553521/config/shortcuts.vdf";
    if !fs::metadata(path).is_ok() 
    {  
        let bytes: &[u8] = &[00u8, b's', b'h', b'o', b'r', b't', b'c', b'u', b't', 
        00u8, 00u8, b'0', 30u8, 00u8, 02u8];
        let mut file = fs::File::create(path).unwrap();
        file.write_all(bytes).unwrap();
        return file;
    } else
    {
        fs::File::open(path).unwrap()
    }
}

pub fn get_shortcuts_vdf_string(game_location: String) -> Vec<u8>
{  
    let mut path: Vec<&str> = game_location.split("/").collect();
    let app_name = path.pop().unwrap();
    let app_id = generate_appid(app_name);
    let start_dir = path.join("/");
    let launch_options = "WINEDLLOVERRIDES=\"OnlineFix64=n;SteamOverlay64=n;winmm=n,b;dnet=n;steam_api64=n\" %command%";
    let mut bytes: Vec<u8> = vec![]; 
    bytes.extend_from_slice("appid".as_bytes());
    bytes.push(00u8); // I guess this one byte is the space between a key and a value,
    bytes.extend_from_slice(app_id.as_bytes());
    bytes.push(01u8); // and this is between two key-value pairs.
    bytes.extend_from_slice("appname".as_bytes());
    bytes.push(00u8);
    bytes.extend_from_slice(app_name.as_bytes());
    bytes.push(00u8); // I'm not sure why this is here. maybe some optional option?
    bytes.push(01u8);
    bytes.extend_from_slice("Exe".as_bytes());
    bytes.push(00u8);
    bytes.extend_from_slice(game_location.as_bytes());
    bytes.push(00u8);
    bytes.push(01u8);
    bytes.extend_from_slice("StartDir".as_bytes());
    bytes.push(00u8);
    bytes.extend_from_slice(start_dir.as_bytes());
    bytes.push(00u8);
    bytes.push(01u8);
    bytes.extend_from_slice("icon".as_bytes());
    bytes.push(00u8);
    bytes.push(00u8);
    bytes.push(01u8);
    bytes.extend_from_slice("ShortcutPath".as_bytes());
    bytes.push(00u8);
    bytes.push(00u8);
    bytes.push(01u8);
    bytes.extend_from_slice("LaunchOptions".as_bytes());
    bytes.push(00u8);
    bytes.extend_from_slice(launch_options.as_bytes());
    bytes.push(00u8);
    bytes.push(02u8);
    bytes.extend_from_slice("IsHidden".as_bytes());
    bytes.extend_from_slice(&[00u8, 00u8, 00u8, 00u8, 00u8, 02u8]); // These are settings (probably).
    bytes.extend_from_slice("AllowDesktopConfig".as_bytes());
    bytes.extend_from_slice(&[00u8, 01u8, 00u8, 00u8, 00u8, 02u8]);
    bytes.extend_from_slice("AllowOverlay".as_bytes());
    bytes.extend_from_slice(&[00u8, 01u8, 00u8, 00u8, 00u8, 02u8]);
    bytes.extend_from_slice("OpenVR".as_bytes());
    bytes.extend_from_slice(&[00u8, 00u8, 00u8, 00u8, 00u8, 02u8]);
    bytes.extend_from_slice("Devkit".as_bytes());
    bytes.extend_from_slice(&[00u8, 00u8, 00u8, 00u8, 00u8, 02u8]);
    bytes.extend_from_slice("DevkitGameID".as_bytes());
    bytes.extend_from_slice(&[00u8, 00u8, 00u8, 00u8, 00u8, 02u8]);
    bytes.extend_from_slice("DevkitOverrideAppID".as_bytes());
    bytes.extend_from_slice(&[00u8, 00u8, 00u8, 00u8, 00u8, 02u8]);
    bytes.extend_from_slice("LastPlayTime".as_bytes());
    bytes.extend_from_slice(&[00u8, 00u8, 00u8, 00u8, 00u8, 01u8]);
    bytes.extend_from_slice("FlatpakAppID".as_bytes());
    bytes.extend_from_slice(&[00u8, 00u8, 00u8]);
    bytes.extend_from_slice("tags".as_bytes());
    bytes.extend_from_slice(&[00u8, 08u8, 08u8, 08u8, 08u8]);
    bytes
    
}

// I found this in the comments of steamtinkerlaunch project. 
fn generate_appid(app_name: &str) -> String { 
    let hasher = compute(app_name.as_bytes());

    let mut appid_array = [0; 4];
    appid_array.copy_from_slice(&hasher[..4]);

    let appid = LittleEndian::read_i16(&appid_array);
    format!("{:04x}", appid)
}