use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::extension::ClientId;
use lettre::{Message, SmtpTransport, Transport};

pub struct Mail<'a> {
    pub to: &'a str,
    pub subject: &'a str,
    pub body: String
}

pub fn send_mail(mail: &Mail) {
    let c = crate::active_config();
    let c_smtp = c.get_extra("smtp").unwrap();

    let email = Message::builder()
        .from(c_smtp["from"].as_str().unwrap().parse().unwrap())
        .reply_to(c_smtp["from"].as_str().unwrap().parse().unwrap())
        .to(mail.to.parse().unwrap())
        .subject(mail.subject)
        .body(mail.body.clone())
        .unwrap();

    let creds = Credentials::new(
        c_smtp["username"].as_str().unwrap().to_string(),
        c_smtp["password"].as_str().unwrap().to_string()
    );

    let mailer = SmtpTransport::starttls_relay(c_smtp["relay"].as_str().unwrap())
        .unwrap()
        .port(c_smtp["port"].as_integer().unwrap() as u16)
        .hello_name(ClientId::Domain("futurehub.krizevci.eu".to_string()))
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
