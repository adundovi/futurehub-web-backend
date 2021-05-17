use crate::db;
use crate::db::models::event::EventAttendee;
use crate::tools::pdflatex::{render_pdf, TemplateRecipe};
use std::path::Path;
use serde_json::value::{Map as JsonMap};
use chrono::{
    NaiveDate,
    NaiveDateTime,
    NaiveTime
};
use crate::tools::cli_edit::{
    edit_option_line,
    edit_option_datetime,
};

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
struct Attendee {
    name: String,
    surname: String,
    email: String,
    role: Option<String>,
    origin: String,
    join_date: NaiveDate,
    join_time: NaiveTime,
    leave_datetime: Option<NaiveDateTime>,
    presence: Option<String>,
}

pub fn f(args: &clap::ArgMatches) {
    let conn = db::establish_connection();

    match args.subcommand() {
        Some(("add",  args)) => {
            let eid = match args.value_of("EID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let uid = match args.value_of("UID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if eid.is_some() && uid.is_some() {
                db::models::event::add_attendee(eid.unwrap(), uid.unwrap(), &conn);
            }
       },
       Some(("list",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => list_attendees(i),
                    Err(_) => print!("ID should be a number"),
                },
                None => print!("No ID given"),
            };
       },
       Some(("edit",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => edit_attendees(i),
                    Err(_) => print!("ID should be a number"),
                },
                None => print!("No ID given"),
            };
       },
       Some(("export_list",  args)) => {
            match args.value_of("ID") {
                Some(i) => match i.parse::<i32>() {
                    Ok(i) => export_list(i),
                    Err(_) => print!("ID should be a number"),
                },
                None => print!("No ID given"),
            };
       },
       Some(("remove",  args)) => {
            let eid = match args.value_of("EID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };
            let uid = match args.value_of("UID") {
                Some(t) => t.parse::<i32>().ok(),
                None => None,
            };

            if eid.is_some() && uid.is_some() {
                db::models::event::remove_attendee(eid.unwrap(), uid.unwrap(), &conn);
            }
       },
        Some((&_, _)) => print!("No subcommand selected"),
        None => print!("No subcommand selected"),
    }
}

fn list_attendees(id: i32) {
    let conn = db::establish_connection();
    for (i, p) in db::models::event::list_attendees(&conn, id).iter().enumerate() {
                let presence = match p.1.presence.as_ref() {
                    Some(p) => p,
                    None => "",
                };
                println!("{}.\t{}({})\t{}", i+1, p.0.username, p.0.id, presence);
    }
}

fn edit_attendees(id: i32) {
    let conn = db::establish_connection();
    let item = db::models::event::list_attendees(&conn, id);
    let mut new_item: Vec<EventAttendee> = vec![];

    for (u, i) in item {
        println!("Attendee: {}", u.username);
        let ea = EventAttendee{
            id: i.id,
            user_id: i.user_id,
            event_id: i.event_id,
            join_datetime: edit_option_datetime(&i.join_datetime, "Join date & time"),
            leave_datetime: edit_option_datetime(&i.leave_datetime, "Leave date & time"),
            presence: edit_option_line(&i.presence, "Presence"),
            note: edit_option_line(&i.note, "Note"),
        };
        new_item.push(ea);
    }
    
    db::models::event::update_attendees(&conn, &new_item);
}

fn export_list(id: i32) {
    let conn = db::establish_connection();
    let event = db::models::event::get(&conn, id).expect("Not found");
    let (course, _) = db::models::event::get_course(&conn, id);
    let first_date = db::models::course::Course::first_date(course.id, &conn);
    let last_date = db::models::course::Course::last_date(course.id, &conn);
    let users_event_attendees = db::models::event::list_attendees(&conn, id);
    let event_datetime = event.datetime;
    let event_id = event.id;

    let template_path = Path::new("./templates/tex/fhk-potpisna-lista.hbs");
    let resources_path = Path::new("./templates/tex");
    let output_path_str = format!("./tmp/potpisna-lista-{course}-{date}.tex",
                                    course = course.code, date = &event_datetime.date());
    let output_path = Path::new(&output_path_str);
        
    let mut data = JsonMap::new();
    data.insert("resources".to_string(),
                to_json(resources_path.canonicalize().unwrap().to_str().unwrap()));
    data.insert("event_id".to_string(), to_json(&event_id));
    data.insert("instance_id".to_string(), to_json(&"fhk.krizevci.eu-001".to_string()));
    data.insert("course_title".to_string(), to_json(&course.title));
    data.insert("course_code".to_string(), to_json(&course.code));
    data.insert("course_lecturer".to_string(), to_json(&course.lecturer));
    data.insert("course_organizer".to_string(), to_json(&course.organizer));
    data.insert("course_creation_date".to_string(), to_json(&course.creation_date.date()));
    data.insert("course_note".to_string(), to_json(&"Project Future Hub Križevci (UP.04.2.1.07.0129)
".to_string()));
    data.insert("course_started".to_string(), to_json(&(first_date.date())));
    data.insert("course_ended".to_string(), to_json(&(last_date.date())));
    data.insert("event_date".to_string(), to_json(&event_datetime.date()));
    data.insert("event_time".to_string(), to_json(&event_datetime.time()));
    let mut attendees: Vec<Attendee> = vec![];
    
    for a in users_event_attendees {

        let join_datetime = a.1.join_datetime.unwrap();

        let new_a = Attendee {
            name: a.0.name.unwrap_or_default(),
            surname: a.0.surname.unwrap_or_default(),
            email: a.0.email,
            role: a.1.note,
            origin: "web".to_string(),
            join_date: join_datetime.date(),
            join_time: join_datetime.time(),
            leave_datetime: a.1.leave_datetime,
            presence: Some("online".to_string()),
        };
        attendees.push(new_a);
    }
   
    attendees.sort_by(|a, b| a.join_time.cmp(&b.join_time));
    data.insert("attendees".to_string(), to_json(&attendees));

    let t = TemplateRecipe {
            template_path: &template_path,
            output_path: &output_path,
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
