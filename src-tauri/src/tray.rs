use tauri::{
    App, Error, Manager, Runtime, menu::{
        Menu,
        MenuItem
    }, tray::TrayIconBuilder
};


pub fn generate_tray<R: Runtime>(app: &App) -> Result<TrayIconBuilder<R>, Error>{

    //menu stuff
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let open_item = MenuItem::with_id(app, "open", "Show Cinny", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&quit_item, &open_item])?;
    let window = app.get_webview_window("main").unwrap();


    Ok(TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }

            "open" => {
                window.show().unwrap();
                window.unminimize().unwrap();
                window.set_focus().unwrap();
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
    )
}
