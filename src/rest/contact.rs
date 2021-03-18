use rocket_contrib::json::Json;
use crate::services::mail;

#[derive(Deserialize, Debug)]
pub struct ContactForm {
    full_name: String,
    email: String,
    phone: Option<String>,
    message: String,
}

#[post("/contact", format = "json", data = "<form>")]
pub fn process_form(form: Json<ContactForm>) {
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
}
