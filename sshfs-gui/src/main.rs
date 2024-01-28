slint::include_modules!();

use rfd::FileDialog;

//'sshfs michael@192.168.1.197:/home/michael/ ~/Dev -o volname=DEV'

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
        println!("Clicked checkbox!");
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_requires_password(!ui.get_requires_password());
            ui.set_password("".into());
        }
    });

    ui.run()
}
