use rocket::http::RawStr;
use rocket_contrib::json::Json;
use crate::db;
use crate::services::mail;
use chrono::NaiveDateTime;

#[derive(Deserialize, Debug)]
pub struct ContactForm {
    name_surname: String,
    email: String,
    phone: Option<String>,
    message: String,
}

#[post("/contact", format = "json", data = "<form>")]
pub fn process_form(form: Json<ContactForm>) {
    let m = mail::Mail{
        to: "andrej@dundovic.com.hr",
        subject: &format!("FutureHub-web - Kontakt - Poruka od {}", &form.name_surname),
        body: format!("Od: {} <{}>\n\
                      Telefon: {}\n\
                      Poruka: {}",
                      &form.name_surname,
                      &form.email,
                      form.phone.as_ref().unwrap_or(&"Telefon nije unesen".to_string()),
                      &form.message).to_string(),
    };

    mail::send_mail(&m);
}
