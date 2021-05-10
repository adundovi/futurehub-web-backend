use crate::db;
use crate::tools::pdflatex::{render_pdf, TemplateRecipe};
use std::path::Path;
use serde_json::value::{Map as JsonMap};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use handlebars::{
    Handlebars,
    Helper,
    Context,
    RenderContext,
    Output,
    HelperResult,
    to_json,
};

pub fn f(args: &clap::ArgMatches) {
    match args.value_of("ID") {
            Some(i) => match i.parse::<i32>() {
                Ok(i) => generate_certificates(i),
                Err(_) => print!("ID should be a number"),
            },
            None => print!("No ID given"),
    };
}

fn generate_certificates(id: i32) {
    let conn = db::establish_connection();
    let course = db::models::course::Course::get(id, &conn).expect("Id not found");
    let participants = db::models::course::Course::list_participants(id, &conn);
    
    let template_path = Path::new("./templates/tex/").join(&course.cert_template.unwrap());
    let resources_path = Path::new("./templates/tex");
    
    /*let date = Utc.datetime_from_str(
                   &format!("{}-01 12:00:00", args.value_of("date").unwrap()),
                   "%Y-%m-%d %H:%M:%S").unwrap().with_timezone(&Utc);*/
   
    for p in participants {
        let mut data = JsonMap::new();

        let name = p.0.name.unwrap_or_default();
        let surname = p.0.surname.unwrap_or_default();
        let oib = p.0.oib.unwrap_or_default();
        let address = p.0.address.unwrap_or_default();
        let gender: bool = p.0.gender.unwrap_or_default() == "woman".to_string();
    
        data.insert("resources".to_string(),
                to_json(resources_path.canonicalize().unwrap().to_str().unwrap()));
        data.insert("name".to_string(), to_json(&name));
        data.insert("surname".to_string(), to_json(&surname));
        data.insert("oib".to_string(), to_json(&oib));
        data.insert("gender".to_string(), to_json(&gender));
        data.insert("address".to_string(), to_json(&address));

        let output_path_str = format!("./tmp/cert-{course}-{surname}-{name}.tex",
                                      course = course.code,
                                      surname = &surname,
                                      name = &name);
        let output_path = Path::new(&output_path_str);

        let t = TemplateRecipe {
            template_path: &template_path,
            output_path: &output_path,
            data: &data,
            helpers: Some(vec![
                      ("genitiv_mjesta".to_string(), genitiv_mjesta_helper)
            ]),
        };

        render_pdf(&t).unwrap();
    }

}

fn read_tezaurus<P>(filename: P) -> io::Result<HashMap<String,String>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut word_map = HashMap::new();
    for line in reader.lines() {
        if let Ok(l) = line {
            let words: Vec<&str> = l.split(">").collect();
            word_map.insert(
                words[0].trim().to_string(),
                words[1].trim().to_string()
            );
        }
    }
    Ok(word_map)
}

fn genitiv_mjesta_helper(h: &Helper,
                             _: &Handlebars,
                             _: &Context,
                             _: &mut RenderContext,
                             out: &mut dyn Output) -> HelperResult {
    let path = Path::new("./templates/tex/address_tezaurus.txt");
    let genitivi = read_tezaurus(path).unwrap();
    let address_value = h.param(0).unwrap().value();

    if address_value.is_string() {
        let address = address_value.as_str().unwrap();
        let default_result = "DODATI GENITIV MJESTA".to_string();
        let genitiv_mjesta =
            genitivi.get(address).unwrap_or(&default_result); 
        out.write(&genitiv_mjesta)?;
    }
    Ok(())
}

