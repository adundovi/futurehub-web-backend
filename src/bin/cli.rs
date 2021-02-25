use futurehub_web_backend::cli;

fn main() {
    let cli_builder = 
        cli::menu_main()
        .subcommand(cli::event::menu())
        .get_matches();

    // process menus
    cli::event::process(cli_builder);
}
