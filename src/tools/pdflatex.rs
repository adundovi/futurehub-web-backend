extern crate handlebars;

use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::error::Error;

use serde_json::value::{Map as JsonMap, Value as Json};
use handlebars::Handlebars;

fn save2file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn run_pdflatex(filename: &str) -> () {
    let status = Command::new("/usr/bin/pdflatex")
            .arg(filename)
            .status()
            .expect("Failed to execute process");
    match status.code() {
        Some(code) => println!("Exited with status code: {}", code),
        None       => println!("Process terminated by signal")
    }
}

pub fn render_pdf(template_path: &str, data: &JsonMap<String, Json>, output: &str) -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    let template_name = "tex_template";
    handlebars.register_template_file(template_name, template_path)?;

    save2file(output, &handlebars.render(template_name, data)?)?;
    run_pdflatex(output);

    Ok(())
}

