extern crate handlebars;

use std::io::prelude::*;
use std::process::Command;
use std::error::Error;
use std::path::Path;

use serde_json::value::{Map as JsonMap, Value as Json};
use handlebars::{
    Handlebars,
    Helper,
    Context,
    RenderContext,
    Output,
    HelperResult
};

#[derive(Clone)]
pub struct TemplateRecipe<'a> {
    pub template_path: &'a Path,
    pub output_path: &'a Path,
    pub data: &'a JsonMap<String, Json>,
    pub helpers: Option<Vec<(String,
                         fn(h: &Helper,
                            _: &Handlebars,
                            _: &Context,
                            rc: &mut RenderContext,
                            out: &mut dyn Output) 
                         -> HelperResult)>>,
}

fn save2file(filepath: &Path, content: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(filepath.parent()
                            .unwrap().to_str().unwrap())?;
    let mut file = std::fs::File::create(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn run_pdflatex(filepath: &Path) -> () {
    let status = Command::new("/usr/bin/pdflatex")
            .arg(format!("-aux-directory={}",
                         filepath.parent()
                         .unwrap().to_str().unwrap()))
            .arg(format!("-output-directory={}",
                         filepath.parent()
                         .unwrap().to_str().unwrap()))
            .arg(filepath.to_str().unwrap())
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
  
    if let Some(helpers) = &recipe.helpers {
        for h in helpers {
            let (n, f) = h;
            handlebars.register_helper(&n, Box::new(f));
        }
    }
    handlebars.register_template_file(template_name,
                                      recipe.template_path.to_str().unwrap())?;

    save2file(recipe.output_path,
              &handlebars.render(template_name, recipe.data)?)?;
    run_pdflatex(recipe.output_path);

    Ok(())
}

