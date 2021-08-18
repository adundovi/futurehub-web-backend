use rocket_contrib::json::Json;
use rocket::http::Status;
use super::response::{Response, ResponseWithStatus, Message};
use crate::services::mail;
use crate::consts::messages;

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

#[post("/signup", format = "json", data = "<form>")]
pub fn post(form: Json<SignupForm>) -> ResponseWithStatus {
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

    ResponseWithStatus {
            status: Status::Ok,
            response: Response::Message(
                Message::new(
                    String::from(messages::MESSAGE_SENT_SUCCESS)
                )
            )
    }
}
