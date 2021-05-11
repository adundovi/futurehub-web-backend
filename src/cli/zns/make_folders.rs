use crate::db;
use std::path::Path;
use std::fs::create_dir_all;
use chrono::prelude::*;
use chrono::NaiveDateTime;

// create post
pub fn f(args: &clap::ArgMatches) {
    let basepath = Path::new(
        args.value_of("PATH").unwrap_or_default());

    lvl1(basepath);
    lvl2_provedba(&basepath
                  .join("1_PROVEDBA"));
   
    let months = vec!["2021-02", "2021-3", "2021-4"];
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

fn lvl4_aktivnost(basepath: &Path) -> () {
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

fn lvl3_aktivnosti(basepath: &Path, dt: DateTime<Utc>) -> () {
    let conn = db::establish_connection();
    for (e, (_, c)) in db::models::event::query_with_course_by_month(&conn, &dt) {
        let activity = format!("{}-{}",
                               c.code,
                               e.datetime.date());
        let activityfolder = basepath.join(activity);
        lvl4_aktivnost(&activityfolder);
    }
}
