use crate::db;
use crate::tools::pdflatex::{render_pdf, TemplateRecipe};
use chrono::prelude::*;
use handlebars::to_json;
use serde::{Serialize, Deserialize};
use std::error::Error;
use serde_json::value::{Map as JsonMap, Value as Json};
use std::path::Path;

#[derive(Deserialize,Serialize,Clone)]
#[serde(rename_all = "PascalCase")]
struct Events {
    date: String,
    items: Vec<db::models::Event>,
}

impl Events {
    fn new(date: &str, e: db::models::Event) -> Events {
        Events { date: date.to_string(), items: vec![e] }
    }
}

// list events
pub fn f(args: &clap::ArgMatches) {

    let template_path = Path::new("./templates/tex/fhk-calendar-a3.hbs");
    let resources_path = Path::new("./templates/tex");
    let year = 2021;
    let month = 04;
    let mut data = make_data(year, month).unwrap();
    data.insert("resources".to_string(),
                to_json(resources_path.canonicalize().unwrap().to_str().unwrap()));

    let output_path_str = format!("./tmp/calendar-{year}-{month}.tex",
                           year = year,
                           month = month);
    let output_path = Path::new(&output_path_str);

    let t = TemplateRecipe {
        template_path: &template_path,
        output_path: &output_path,
        data: &data,
        helpers: None
    };

    render_pdf(&t).unwrap();
}

fn load_calendar() -> Result<Vec<db::models::Event>, Box<dyn Error>> {
    let mut events: Vec<db::models::Event> = Vec::new();
    
    let conn = db::establish_connection();
    let utc_now = Utc::now();
    for e in db::event::query_by_month(&conn, &utc_now) {
        events.push(e);
    };

    Ok(events)
}

fn search_in_vec(v: &Vec<Events>, needle: &str) -> Option<usize> {
    if v.len() == 0 {
        return None;
    }
    for (i, h) in v.iter().enumerate() {
        if h.date == needle {
            return Some(i);
        }
    }
    None
}

fn sort_events(events: Vec<db::models::Event>) ->
    (Vec<Events>, Vec<Events>) {
    let mut events_in_hub: Vec<Events> = Vec::new();
    let mut events_outside_hub: Vec<Events> = Vec::new();

    for e in events {
        let idx = String::from(format!("{} ({})",
            &e.datetime.format("%d. %m."), &e.datetime.weekday()));
        if e.place == Some("Prostori FHK-a".to_string()) {
            let i = search_in_vec(&events_in_hub, &idx);
            match i {
                Some(i) => events_in_hub[i].items.push(e),
                None => events_in_hub.push(
                        Events::new(&idx, e)
                    ),

            }
        } else {
            let i = search_in_vec(&events_outside_hub, &idx);
            match i {
                Some(i) => events_outside_hub[i].items.push(e),
                None => events_outside_hub.push(
                        Events::new(&idx, e)
                    ),

            }
        }
    }

    (events_in_hub, events_outside_hub)
}

pub fn make_data(year: u32, month: u32) -> Result<JsonMap<String, Json>, Box<dyn Error>> {
    let mut data = JsonMap::new();

    data.insert("month".to_string(), to_json(month));
    data.insert("year".to_string(), to_json(year));

    let events = load_calendar()?;
    let (events_in_hub, events_outside_hub) = sort_events(events);

    let limiter = 8;
    if events_in_hub.len() > limiter {
        let events_in_hub_1: Vec<Events> = events_in_hub[0..(limiter+1)].to_vec();
        data.insert("events_in_hub_1".to_string(), to_json(events_in_hub_1));
        let events_in_hub_2: Vec<Events> = events_in_hub[(limiter+1)..].to_vec();
        data.insert("events_in_hub_2".to_string(), to_json(events_in_hub_2));
    } else {
        let events_in_hub_1: Vec<Events> = events_in_hub.to_vec();
        data.insert("events_in_hub_1".to_string(), to_json(events_in_hub_1));
    }

    data.insert("events_outside_hub".to_string(), to_json(events_outside_hub));

    Ok(data)
}
