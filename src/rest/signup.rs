use rocket_contrib::json::Json;
use rocket::{
    Route,
    http::Status
};
use super::response::{Response, ResponseWithStatus, Message};
use crate::services::mail;
use crate::consts::messages;

pub fn get_routes() -> Vec<Route> {
    routes![
        post,
        option,
    ]
}

#[derive(Deserialize, Debug)]
pub struct SignupForm {
    name: String,
    surname: String,
    email: String,
    phone: Option<String>,
    oib: String,
    address: String,
    message: Option<String>,
}

fn mail_to_owners(form: &SignupForm) -> () {
    let m = mail::Mail{
        to: "futurehub@udruga-point.hr",
        subject: &format!("FutureHub-web - Prijava - {}", &form.email),
        body: format!("Prijava na obrazovni program - Ljetna škola računarstva 2021\n
                Ime: {}\n
                Prezime: {}\n
                Email: {}\n
                Telefon: {}\n
                OIB: {}\n
                Adresa: {}\n
                Napomena: {}",
                      &form.name,
                      &form.surname,
                      &form.email,
                      form.phone.as_ref().unwrap_or(&"Telefon nije unesen".to_string()),
                      &form.oib,
                      &form.address,
                      &form.message.as_ref().unwrap_or(&"Nema napomene".to_string())
                ).to_string(),
    };
    
    mail::send_mail(&m);
}

fn mail_to_user(form: &SignupForm) -> () {
    let m = mail::Mail{
        to:  &form.email,
        subject: "Future Hub Križevci - Uspješna prijava",
        body: format!("Draga/dragi {},\n
hvala Vam na prijavi na obrazovni program \"Ljetna škola računarstva 2021\". Vaša je prijava zabilježena.\n
Iako je automatski generirana i poslana poruka, za sva pitanja vezana uz prijavu ili program, slobodno odgovorite na ovaj mail.\n
Projekt Future Hub Križevci",
                      &form.name,
                ).to_string(),
    };
    
    mail::send_mail(&m);
}

#[options("/signup")]
pub fn option<'a>() -> rocket::Response<'a> {
    let mut res = rocket::Response::new();
    res.set_status(Status::new(200, "No Content"));
    res
}

#[post("/signup", format = "json", data = "<form>")]
pub fn post(form: Json<SignupForm>) -> ResponseWithStatus {

    mail_to_owners(&form);
    mail_to_user(&form);

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(
                    String::from(messages::MESSAGE_SENT_SUCCESS)
                )
            )
    }
}
