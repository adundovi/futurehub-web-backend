use futurehub_web_backend::cli;

fn main() {

    let post_menu = cli::post::menu();

    let cli_builder = 
        cli::menu::main()
        .subcommand(cli::event::menu())
        .subcommand(post_menu.generate())
        .get_matches();

    post_menu.process(&cli_builder);
    cli::event::process(&cli_builder);
}
