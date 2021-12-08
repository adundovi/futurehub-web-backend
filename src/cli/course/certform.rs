use crate::db;
use crate::db::model_traits::Queries;
use tex_tmpl_rs::{render_pdf, TemplateRecipe};
use std::path::Path;
use serde_json::value::{Map as JsonMap};

use handlebars::{
    Handlebars,
    Helper,
    Context,
    RenderContext,
    Output,
    HelperResult,
    to_json,
};

#[derive(Deserialize,Serialize,Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Participant {
    name: String,
    surname: String,
    oib: String
}

pub fn f(args: &clap::ArgMatches) {
    match args.value_of("ID") {
            Some(i) => match i.parse::<i32>() {
                Ok(i) => generate_form(i),
                Err(_) => print!("ID should be a number"),
            },
            None => print!("No ID given"),
    };
}

fn generate_form(id: i32) {
    let conn = db::establish_connection();
    let course = db::models::course::Course::get(&conn, id).expect("Id not found");
    let first_date = db::models::course::Course::first_date(id, &conn);
    let last_date = db::models::course::Course::last_date(id, &conn);
    let db_participants = db::models::course::Course::list_participants(&conn, id);
    
    let template_path = Path::new("./templates/tex/zapisnik-o-primopredaji.hbs");
    let resources_path = Path::new("./templates/tex");
    let output_path_str = format!("./tmp/certform-{course}.pdf",
                                    course = course.code);
    let output_path = Path::new(&output_path_str);
        
    let mut data = JsonMap::new();
    data.insert("resources".to_string(),
                to_json(resources_path.canonicalize().unwrap().to_str().unwrap()));
    data.insert("course_title".to_string(), to_json(&course.title));
    data.insert("course_code".to_string(), to_json(&course.code));
    data.insert("course_lecturer".to_string(), to_json(&course.lecturer));
    data.insert("course_started".to_string(), to_json(&(first_date.date())));
    data.insert("course_ended".to_string(), to_json(&(last_date.date())));
    let mut participants: Vec<Participant> = vec![];
    
    for p in db_participants {

        let participant = Participant {
            name: p.0.name.unwrap_or_default(),
            surname: p.0.surname.unwrap_or_default(),
            oib: p.0.oib.unwrap_or_default()
        };
        participants.push(participant);
    }
   
    participants.sort_by(|a, b| a.surname.cmp(&b.surname));
    data.insert("participants".to_string(), to_json(&participants));

    let t = TemplateRecipe {
            template: &template_path,
            output: &output_path,
            data: &data,
            helpers: Some(vec![
                ("inc".to_string(), inc_helper)
            ]),
    };
    render_pdf(&t).unwrap();
}

fn inc_helper(h: &Helper,
              _: &Handlebars,
              _: &Context,
              _: &mut RenderContext,
              out: &mut dyn Output) -> HelperResult {
    let dt = h.param(0).unwrap().value();
    out.write(&(dt.as_i64().unwrap() + 1).to_string())?;
    Ok(())
}
