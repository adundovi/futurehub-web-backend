extern crate handlebars;

use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::error::Error;
use std::path::Path;

use serde_json::value::{Map as JsonMap, Value as Json};
use handlebars::Handlebars;

#[derive(Clone)]
pub struct TemplateRecipe<'a> {
    template_path: &'a Path,
    output_path: &'a Path,
    data: &'a JsonMap<String, Json>,
    helpers: Option<Vec<(String,
                         fn(h: &handlebars::Helper,
                            _: &Handlebars,
                            _: &handlebars::Context,
                            rc: &mut handlebars::RenderContext,
                            out: &mut dyn handlebars::Output) 
                         -> handlebars::HelperResult)>>,
}

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

pub fn render_pdf(recipe: &TemplateRecipe) -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();
    
    let template_name = "tex_template";
    let template_path = recipe.template_path.to_str().unwrap();
    let output_path = recipe.output_path.to_str().unwrap();
   
    if recipe.helpers.is_some() {
        for h in recipe.helpers.unwrap() {
            let (n, f) = h;
            handlebars.register_helper(&n, Box::new(f));
        }
    }
    handlebars.register_template_file(template_name, template_path)?;

    save2file(output_path, &handlebars.render(template_name, recipe.data)?)?;
    run_pdflatex(output_path);

    Ok(())
}

