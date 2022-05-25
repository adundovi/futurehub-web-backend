use crate::db;
use std::path::Path;
use std::fs::create_dir_all;
use chrono::prelude::*;

//TODO: kad se nije održalo, ne napravi folder

// create post
pub fn f(args: &clap::ArgMatches) {
    let basepath = Path::new(
        args.value_of("PATH").unwrap_or_default());

    lvl1(basepath);
    lvl2_provedba(&basepath
                  .join("1_PROVEDBA"));
   
    let months = vec!["2022-05", "2022-06", "2022-07"];
    for m in months {
        let date = Utc.datetime_from_str(
                   &format!("{}-01 12:00:00", m),
                   "%Y-%m-%d %H:%M:%S").unwrap().with_timezone(&Utc);
        lvl3_aktivnosti(&basepath
                    .join("1_PROVEDBA")
                    .join("PE_1"),
                    date);
    }
}

fn mkdir_from_vec(basepath: &Path, v: Vec<String>) -> () {
    for f in v {
        let newfolder = basepath.join(f);
        create_dir_all(&newfolder).unwrap();
    }
}

fn lvl1(basepath: &Path) -> () {
    let rootfolders = vec![
        "1_PROVEDBA".to_string(),
        "2_POKAZATELJI".to_string(),
        "3_FINANCIJE".to_string(),
    ];
    mkdir_from_vec(basepath, rootfolders);
}

fn lvl2_provedba(basepath: &Path) -> () {
    let folders = vec![
        "PE_1".to_string(),
        "PE_2".to_string(),
        "PE_PM".to_string(),
        "PE_V".to_string(),
    ];
    mkdir_from_vec(basepath, folders);
}

fn prepare_string(s: String) -> String {
    s.replace(" ", "_")
     .replace("\"", "")
     .replace("'", "")
     .replace("(", "")
     .replace(")", "")
     .replace(":", "")
     .replace("?", "")
     .replace("č", "c")
     .replace("ć", "c")
     .replace("đ", "d")
     .replace("š", "s")
     .replace("ž", "z")
     .replace("Č", "C")
     .replace("Ć", "C")
     .replace("Đ", "D")
     .replace("Š", "S")
     .replace("Ž", "Z")
}

fn lvl3_aktivnosti(basepath: &Path, dt: DateTime<Utc>) -> () {
    let conn = db::establish_connection();
    for (e, c) in db::models::event::Event::query_with_course_by_month(&conn, &dt) {
        let activity = format!("{}-{}",
                               c.code,
                               prepare_string(c.title));
        let activityfolder = basepath.join(activity);
        lvl4_aktivnosti(&activityfolder, e);
    }
}

fn lvl4_aktivnosti(basepath: &Path, e: db::models::event::Event) -> () {
        let activity = format!("{}",
                               e.datetime.date());
        let activityfolder = basepath.join(activity);
        lvl5_aktivnost(&activityfolder);
}

fn lvl5_aktivnost(basepath: &Path) -> () {
    let folders = vec![
        "1_POTPISNE_LISTE".to_string(),
        "2_FOTOGRAFIJE".to_string(),
        "3_MATERIJALI_PROVEDBE".to_string(),
        "4_PREZENTACIJA".to_string(),
        "4_EVALUACIJA".to_string(),
        "5_CERTIFIKATI".to_string(),
        "6_POZIV".to_string(),
    ];
    mkdir_from_vec(basepath, folders);
}
