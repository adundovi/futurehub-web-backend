use crate::db;
use crate::db::model_traits::Queries;
use std::collections::BTreeMap;
use chrono::NaiveDate;

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

pub fn f(args: &clap::ArgMatches) {
    let conn = db::establish_connection();

    match args.subcommand() {
        Some(("add",  args)) => {
            let cid = match args.value_of("CID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let uid = match args.value_of("UID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if cid.is_some() && uid.is_some() {
                db::models::course::Course::add_participant(cid.unwrap(), uid.unwrap(), &conn);
            }
       },
       Some(("list",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => list_participants(i),
                    Err(_) => print!("ID should be a number"),
                },
                None => print!("No ID given"),
            };
       },
       Some(("remove",  args)) => {
            let cid = match args.value_of("CID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let uid = match args.value_of("UID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if cid.is_some() && uid.is_some() {
                db::models::course::Course::remove_participant(cid.unwrap(), uid.unwrap(), &conn);
            }
       },
       Some(("recalculate", _)) => {
            for c in db::models::course::Course::get_all(&conn).expect("Not found") {
                let n_participants = db::models::course::Course::list_participants(&conn, c.id).len() as i32;
                println!("id = {}, N = {}", c.title, n_participants);
                db::models::course::Course::set_students(&conn, c.id, n_participants); 
            }
       },
       Some(("export", _)) => {
           export_participants();
       },
       Some((&_, _)) => print!("No subcommand selected"),
       None => print!("No subcommand selected"),
    }
}

fn list_participants(id: i32) {
    let conn = db::establish_connection();
    for (i, p) in db::models::course::Course::list_participants(&conn, id).iter().enumerate() {
                println!("{}.\t{}({})\t{}", i+1, p.0.username, p.0.id, p.1.join_date);
    }
}

#[derive(Deserialize,Serialize,Clone, Eq, Ord, PartialEq, PartialOrd)]
struct Participant {
    idx: usize,
    name: String,
    surname: String,
    gender: bool,
    courses: Vec<bool>
}

fn export_participants() {
    let conn = db::establish_connection();
    
    let template_path = Path::new("./templates/tex/popis-polaznika.hbs");
    let resources_path = Path::new("./templates/tex");
    let output_path_str = format!("./tmp/popis-polaznika.pdf");
    let output_path = Path::new(&output_path_str);
            
    let mut data = JsonMap::new();
    data.insert("resources".to_string(),
                to_json(resources_path.canonicalize().unwrap().to_str().unwrap()));

    let mut courses: Vec<String> = vec![];
    let mut courses_n: Vec<i32> = vec![];
    let mut total_women: i32= 0;
    let mut table = BTreeMap::<i32, (db::models::user::User, Vec<String>)>::new();
    
    for c in db::models::course::Course::get_all(&conn).expect("Not found") {
        if c.students == Some(0) || c.students == None {
            continue;
        }
        // courses created before given date
        /*
        if c.creation_date > NaiveDate::from_ymd(2022, 1, 31).and_hms(0, 0, 0) {
            continue;
        }
        if c.code == "FHK.1.1.4" || c.code  == "FHK.1.2.3" {
            continue;
        }*/
        courses.push(c.code.clone());
        let participants = db::models::course::Course::list_participants(&conn, c.id);
        courses_n.push(participants.len() as i32);
        for p in participants {
            if (p.0.gender) == Some("woman".to_string()) {
                total_women += 1;
            }
            let row = table.entry(p.0.id).or_insert((p.0, vec![]));
            row.1.push(c.code.clone());
        }
    }

    let mut participants: Vec<Participant> = vec![];
    
    for (i, (_, item)) in (&table).into_iter().enumerate() {
        let p = &item.0;
        let c: Vec<bool> = courses.iter().map(|i| item.1.contains(i)).collect();
        let name: String = p.name.as_ref().unwrap().to_string();
        let surname: String = p.surname.as_ref().unwrap().to_string();
        let gender: bool = p.gender.as_ref().unwrap() == "woman";
        let participant = Participant {
            idx: i + 1,
            name: name,
            surname: surname,
            gender: gender,
            courses: c
        };
        participants.push(participant);
    }
        
    data.insert("courses".to_string(), to_json(&courses));
    data.insert("courses_n".to_string(), to_json(&courses_n));
    data.insert("total".to_string(), to_json(&courses_n.iter().sum::<i32>()));
    //data.insert("total_women".to_string(), to_json(&participants.iter().map(|p| p.gender  as i32).sum::<i32>()));
    data.insert("total_women".to_string(), to_json(&total_women));
    data.insert("participants".to_string(), to_json(&participants));
   
    let t = TemplateRecipe {
            template: &template_path,
            output: &output_path,
            data: &data,
            helpers: None
    };
    render_pdf(&t).unwrap();
}
