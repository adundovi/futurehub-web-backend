use rocket_contrib::json::Json;
use rocket::{
    Route,
    http::Status
};
use chrono::Utc;
use sha2::{Sha256, Digest};
use rand::Rng;
use base64ct::{Base64, Encoding};

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

fn mail_to_owners(form: &consent::NewConsent) -> () {
    /*let m = mail::Mail{
        to: "futurehub@udruga-point.hr",
        subject: &format!("[FHK] Prijava na {} - {}", &form.course, &form.email),
        body: format!("Prijava na obrazovni program - {}\n
                Ime: {}\n
                Prezime: {}\n
                Email: {}\n
                Telefon: {}\n
                OIB: {}\n
                Adresa: {}\n
                Napomena: {}",
                      &form.course,
                      &form.name,
                      &form.surname,
                      &form.email,
                      form.phone.as_ref().unwrap_or(&"Telefon nije unesen".to_string()),
                      &form.oib,
                      &form.address,
                      &form.message.as_ref().unwrap_or(&"Nema napomene".to_string())
                ).to_string(),
    };
    
    mail::send_mail(&m);*/
}

fn mail_to_user(c: &consent::NewConsent) -> () {
        let m = mail::Mail{
            to:  &c.email,
            subject: "Future Hub Križevci - Provjera privole",
            body: format!("Poštovani,\n
obrazac privole uspješno je poslan, ali je samu privolu potrebno potvrditi na sljedećoj poveznici:\n
{}\n

Iako je ova poruka automatski generirana i odaslana, za sva pitanja vezana uz davanje privole ili sam program, slobodno pošaljite upit na ovu mail adresu.\n
Projekt Future Hub Križevci",
                      &c.verify_hash.as_ref().unwrap(),
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

#[post("/consent", format = "json", data = "<form>")]
pub fn post(form: Json<ConsentForm>, conn: MainDbConn) -> ResponseWithStatus {

    let mut hasher = Sha256::new();
    let mut rng = rand::thread_rng();

    let r: u8 = rng.gen();
    hasher.update(format!("{}{}", &form.email, r));
    let hash = hasher.finalize();
    let encoded_hash = Base64::encode_string(&hash);

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

    //mail_to_owners(&form);
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
