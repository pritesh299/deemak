mod keys;
mod screen;
mod server;
use deemak::menu;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // We have 2 modes, the web and the raylib gui. The web argument runs it on the web, else
    // raylib gui is set by default.
    if args.len() > 1 && args[1] == "web" {
        server::launch_web();
        return;
    }

    // Initialize Raylib window
    let (mut rl, thread) = raylib::init().size(800, 600).title("DEEMAK Shell").build();

    // Main menu loop
    loop {
        match menu::show_menu(&mut rl, &thread) {
            Some(0) => {
                // Shell mode
                let mut shell = screen::ShellScreen::new_world(rl, thread);
                shell.run();
                break; // Exit after shell closes
            }
            Some(1) => {
                // About screen
                menu::about::show_about(&mut rl, &thread);
            }
            Some(2) | None => {
                // Exit
                break;
            }
            _ => unreachable!(),
        }
    }
}
