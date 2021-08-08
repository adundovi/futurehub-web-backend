use rocket_contrib::json::Json;
use rocket::http::Status;
use super::response::{Response, ResponseWithStatus, Message};
use crate::services::mail;
use crate::consts::messages;

#[derive(Deserialize, Debug)]
pub struct ContactForm {
    full_name: String,
    email: String,
    phone: Option<String>,
    message: String,
}

#[post("/contact", format = "json", data = "<form>")]
pub fn post_form(form: Json<ContactForm>) -> ResponseWithStatus {
    let m = mail::Mail{
        to: "4ndY@krizevci.info",
        subject: &format!("FutureHub-web - Kontakt - Poruka od {}", &form.full_name),
        body: format!("Od: {} <{}>\n
                      Telefon: {}\n
                      Poruka: {}",
                      &form.full_name,
                      &form.email,
                      form.phone.as_ref().unwrap_or(&"Telefon nije unesen".to_string()),
                      &form.message).to_string(),
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
