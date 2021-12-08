use crate::db;
use tex_tmpl_rs::{render_pdf, TemplateRecipe};
use std::path::Path;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use std::error::Error;
use serde_json::value::{Map as JsonMap, Value as Json};

use handlebars::{
    Handlebars,
    Helper,
    Context,
    RenderContext,
    Output,
    HelperResult,
    to_json,
};

use std::str::FromStr;

#[derive(Deserialize,Serialize,Clone)]
#[serde(rename_all = "PascalCase")]
struct Events {
    date: String,
    items: Vec<db::models::event::Event>,
}

impl Events {
    fn new(date: &str, e: db::models::event::Event) -> Events {
        Events { date: date.to_string(), items: vec![e] }
    }
}

enum TexFontSize {
    Smaller,
    Small,
    Normal,
    Large,
    Larger,
}

impl FromStr for TexFontSize {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "smaller" => Ok(TexFontSize::Smaller),
            "small" => Ok(TexFontSize::Small),
            "normal" => Ok(TexFontSize::Normal),
            "large" => Ok(TexFontSize::Large),
            "larger" => Ok(TexFontSize::Larger),
            _ => Err("no matched fontsize"),
        }
    }
}

// list events
pub fn f(args: &clap::ArgMatches) {
        
    let date = Utc.datetime_from_str(
                   &format!("{}-01 12:00:00", args.value_of("date").unwrap()),
                   "%Y-%m-%d %H:%M:%S").unwrap().with_timezone(&Utc);
    
    let template_path = Path::new("./templates/tex/fhk-calendar-a3.hbs");
    let resources_path = Path::new("./templates/tex");

    let split_at: usize = args.value_of_t("split_at").unwrap_or(10);

    let mut data = make_data(date, split_at).unwrap();
    
    let fontsize = match args.value_of_t("size").unwrap() {
        TexFontSize::Smaller => "\\footnotesize",
        TexFontSize::Small => "\\small",
        TexFontSize::Normal => "\\normalsize",
        TexFontSize::Large => "\\large",
        TexFontSize::Larger => "\\Large",
    };
    data.insert("resources".to_string(),
                to_json(resources_path.canonicalize().unwrap().to_str().unwrap()));
    data.insert("fontsize".to_string(), to_json(fontsize));
    
    let output_path_str = format!("./tmp/calendar-{year}-{month}.pdf",
                           year = date.year(),
                           month = date.month());
    let output_path = Path::new(&output_path_str);

    let t = TemplateRecipe {
        template: &template_path,
        output: &output_path,
        data: &data,
        helpers: Some(vec![
                      ("datetime2shorttime".to_string(), datetime2shorttime_helper)
        ]),
    };

    render_pdf(&t).unwrap();
}

fn datetime2shorttime_helper(h: &Helper,
                             _: &Handlebars,
                             _: &Context,
                             _: &mut RenderContext,
                             out: &mut dyn Output) -> HelperResult {
    let dt = h.param(0).unwrap().value();

    if dt.is_string() {
        let short_t = NaiveDateTime::parse_from_str(
            dt.as_str().unwrap(),
            "%Y-%m-%dT%H:%M:%S").unwrap()
            .format("%H:%M").to_string();
        out.write(&short_t)?;
    }
    Ok(())
}

fn load_calendar(dt: DateTime<Utc>) -> Result<Vec<db::models::event::Event>, Box<dyn Error>> {
    let conn = db::establish_connection();
    let events = db::models::event::Event::query_by_month(&conn, &dt)
                    .into_iter().map(|i| i.0).collect();
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

fn sort_events(events: Vec<db::models::event::Event>) ->
    (Vec<Events>, Vec<Events>) {
    let mut events_in_hub: Vec<Events> = Vec::new();
    let mut events_outside_hub: Vec<Events> = Vec::new();

    fn weekday2hrweekday<'a>(wd: &'a Weekday) -> &'a str {
        match wd {
            Weekday::Mon => "ponedjeljak",
            Weekday::Tue => "utorak",
            Weekday::Wed => "srijeda",
            Weekday::Thu => "četvrtak",
            Weekday::Fri => "petak",
            Weekday::Sat => "subota",
            Weekday::Sun => "nedjelja",
        }
    }

    for e in events {
        let idx = String::from(format!("{} ({})",
            &e.datetime.format("%d. %m."), weekday2hrweekday(
                &e.datetime.weekday())
            ));
        if e.place == Some("Prostori FHK-a".to_string()) ||
           e.place == Some("Online".to_string()) {
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

pub fn make_data(dt: DateTime<Utc>, split_at: usize) -> Result<JsonMap<String, Json>, Box<dyn Error>> {
    let mut data = JsonMap::new();

    data.insert("month".to_string(), to_json(dt.month()));
    data.insert("year".to_string(), to_json(dt.year()));

    let events = load_calendar(dt)?;
    let (events_in_hub, events_outside_hub) = sort_events(events);

    if events_in_hub.len() > split_at {
        let events_in_hub_1: Vec<Events> = events_in_hub[0..(split_at+1)].to_vec();
        data.insert("events_in_hub_1".to_string(), to_json(events_in_hub_1));
        let events_in_hub_2: Vec<Events> = events_in_hub[(split_at+1)..].to_vec();
        data.insert("events_in_hub_2".to_string(), to_json(events_in_hub_2));
    } else {
        let events_in_hub_1: Vec<Events> = events_in_hub.to_vec();
        data.insert("events_in_hub_1".to_string(), to_json(events_in_hub_1));
    }

    data.insert("events_outside_hub".to_string(), to_json(events_outside_hub));

    Ok(data)
}
