slint::include_modules!();

use rfd::FileDialog;
use std::net::{Ipv4Addr};
use std::process::Command;
use std::path::Path;
use std::str;
use regex::Regex;

//'sshfs michael@192.168.1.197:/home/michael/ ~/Dev -o volname=DEV'

slint::slint!{
    export component HelloWorld {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_get_local_mount_point({
        let ui_handle = ui.as_weak();
        move || {
            let res = FileDialog::new()
                .set_directory("~")
                .pick_folder();
            let ui = ui_handle.unwrap();
            if res.is_some() {
                ui.set_local_mount_point(res.unwrap().display().to_string().into());
            }
        }
    });

    ui.on_requires_password_change({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_requires_password(!ui.get_requires_password());
            ui.set_password("".into());
        }
    });

    ui.on_mount_remote_machine({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let local_mount_point = ui.get_local_mount_point().to_string();
            let mount_point_path = Path::new(&local_mount_point);
            if mount_point_path.exists() {
                let output = Command::new("df")
                    .arg(&local_mount_point)
                    .output()
                    .expect("Failed to execute command");
                if output.status.success() {
                    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8");
                    let lines: Vec<&str> = output_str.lines().collect();
                    if lines.len() == 2 {
                        if let Some(second_line) = lines.get(1) {
                            let fields: Vec<&str> = second_line.split_whitespace().collect();
                            if let Some(filesys) = fields.get(0) {
                                let re = Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap();
                                if let Some(captured) = re.find(filesys){
                                    let ip_str = captured.as_str();
                                    if ip_str.parse::<Ipv4Addr>().is_ok(){
                                        // If there's an IP address in the string, and it's a valid IPv4 address and therefore is already a mount point for a sshfs volume.
                                        println!("{} is a valid IPv4 address", ip_str);
                                        notification().expect("");
                                    }
                                } else {
                                    // if it doesn't contain an IPv4 address, safe to continue.
                                    println!("{} is not a valid IPv4 address", filesys);
                                }
                            } else {
                                println!("Unable to extract file system information");
                            }
                        }
                    }
                }
            }
        }
    });

    ui.run()
}

fn notification(/*message: &str, on_ok: fn() -> bool, on_cancel: fn() -> bool*/) -> Result<(), slint::PlatformError> {
   let hello = HelloWorld::new()?;
    hello.run()
}

fn unmount(path: &Path) -> bool{
    return true;
}
