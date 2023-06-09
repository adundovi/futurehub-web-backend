use futurehub_web_backend::cli;
use futurehub_web_backend::cli::menu::Menu;

fn main() {
    // Get all submenus...
    let submenus: Vec<Menu> = vec![
        cli::category::menu(),
        cli::course::menu(),
        cli::event::menu(),
        cli::post::menu(),
        cli::repo::menu(),
        cli::user::menu(),
        cli::zns::menu(),
    ];
    
    // ...generate calp::App from them...
    let menu_apps: Vec<clap::App> =
        submenus.iter()
                .map(|m| m.generate())
                .collect();
    
    // ...build and parse...
    let cli_builder = 
        cli::menu::main_menu()
        .subcommands(menu_apps)
        .get_matches();

    // ...and finally, run the command.
    for m in submenus {
        m.process(&cli_builder);
    }
}
