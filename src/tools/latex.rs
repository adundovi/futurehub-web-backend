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

fn sort_events(events: Vec<Event>) -> 
    (Vec<Events>, Vec<Events>) {
    let mut events_in_hub: Vec<Events> = Vec::new();
    let mut events_outside_hub: Vec<Events> = Vec::new();

    for e in events {
        let idx = String::from(format!("{} ({})", &e.date, &e.day));
        if &e.location == "hub" {
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

fn save2file(filename: &String, content: &String) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn make_data(year: u32, month: u32) -> Result<JsonMap<String, Json>, Box<dyn Error>> {
    let mut data = JsonMap::new();

    data.insert("month".to_string(), to_json(month));
    data.insert("year".to_string(), to_json(year));

    let events = load_calendar()?;
    let (events_in_hub, events_outside_hub) = sort_events(events);

    let limiter = 6;
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

fn run_pdflatex(filename: &String) -> () {
    let status = Command::new("/usr/bin/pdflatex")
            .arg(filename.as_str())
            .status()
            .expect("Failed to execute process");
    match status.code() {
        Some(code) => println!("Exited with status code: {}", code),
        None       => println!("Process terminated by signal")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("calendar", "./templates/fhk-calendar-a3.hbs")?;

    let year = 2020;
    let month = 11;

    let resources = "templates/".to_string();
    let mut data = make_data(year, month)?;
    data.insert("resources".to_string(), to_json("templates"));

    let filename = format!("tmp/calendar-{year}-{month}.tex",
                           year = year,
                           month = month).to_string();
    save2file(&filename, &handlebars.render("calendar", &data)?);
    run_pdflatex(&filename);
    Ok(())
}

