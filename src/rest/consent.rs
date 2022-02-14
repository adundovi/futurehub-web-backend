use rocket_contrib::json::Json;
use rocket::{
    uri,
    Route,
    http::Status,
    response::Redirect,
};
use chrono::Utc;
use sha2::{Sha256, Digest};
use rand::Rng;

use super::response::{Response, ResponseWithStatus, Message};
use crate::services::mail;
use crate::consts::messages;
use crate::db::{
    MainDbConn,
    models::consent
};

pub fn get_routes() -> Vec<Route> {
    routes![
        post,
        option,
        verify,
    ]
}

#[derive(Deserialize, Debug)]
pub struct ConsentForm {
    name: String,
    surname: String,
    email: String,
    phone: Option<String>,
    oib: String,
    child_name: String,
    child_surname: String,
    consent_on_off: String,
    consent_type: String,
}

fn mail_to_owners(c: &consent::Consent) -> () {
    let m = mail::Mail{
        to: "andrej.dundovic@udruga-point.hr",
        subject: &format!("[FHK] Nova privola za: {} {}", &c.child_surname, &c.child_name),
        body: format!("Potvrđen je obrazac za privolu - {} {}\n
                Ime i prezime roditelja/staratelja: {} {}\n
                Email: {}\n
                OIB: {}\n
                Ime i prezime djeteta: {} {}\n
                ",
                      &c.consent_type,
                      &c.consent_on_off,
                      &c.name,
                      &c.surname,
                      &c.email,
                      &c.oib,
                      &c.child_name,
                      &c.child_surname,
                ).to_string(),
    };
    
    mail::send_mail(&m);
}

fn mail_to_user(c: &consent::NewConsent) -> () {
        let m = mail::Mail{
            to:  &c.email,
            subject: "Future Hub Križevci - Provjera privole",
            body: format!("Poštovani,\n
obrazac privole uspješno je poslan, ali je samu privolu potrebno potvrditi na sljedećoj poveznici:\n\n
https://futurehub.krizevci.eu{}\n\n

Iako je ova poruka automatski generirana i odaslana, za sva pitanja vezana uz davanje privole ili sam program, slobodno pošaljite upit na ovu mail adresu.\n
Projekt Future Hub Križevci",
                      uri!("/api", verify: hash = c.verify_hash.as_ref().unwrap()),
                ).to_string(),
    };
    
    mail::send_mail(&m);
}

#[options("/consent")]
pub fn option<'a>() -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[get("/consent/verify?<hash>")]
pub fn verify(hash: String, conn: MainDbConn) -> Redirect {

    match consent::Consent::verify(&conn, &hash) {
        Ok(c) => {
            mail_to_owners(&c);
            Redirect::to("https://futurehub.krizevci.eu/consent/accepted")
        },
        Err(_) => {
            Redirect::to("https://futurehub.krizevci.eu/consent/error")
        }
    }
}

#[post("/consent", format = "json", data = "<form>")]
pub fn post(form: Json<ConsentForm>, conn: MainDbConn) -> ResponseWithStatus {

    let mut hasher = Sha256::new();
    let mut rng = rand::thread_rng();

    let r: u8 = rng.gen();
    hasher.update(format!("{}{}", &form.email, r));
    let hash = hasher.finalize();
    let encoded_hash = base16ct::lower::encode_string(&hash);

    let c = consent::NewConsent{
        name: form.name.clone(),
        surname: form.surname.clone(),
        email: form.email.clone(),
        phone: form.phone.clone(),
        oib: form.oib.clone(),
        child_name: form.child_name.clone(),
        child_surname: form.child_surname.clone(),
        consent_on_off: form.consent_on_off.clone(),
        consent_type: form.consent_type.clone(),
        entry_date:  Utc::now().naive_utc(),
        verified: false,
        verify_hash: Some(encoded_hash.clone())
    };
    
    consent::Consent::insert(&conn, &c);

    mail_to_user(&c);

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(
                    String::from(messages::MESSAGE_SENT_SUCCESS)
                )
            )
    }
}
