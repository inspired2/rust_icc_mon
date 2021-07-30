use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};
use crate::custom_err;

use super::{EMAIL_TO, EMAIL_FROM, CustomErr};

pub fn send_email(m: String) -> Result<(), CustomErr> {
    let email = Message::builder()
        .from(EMAIL_FROM.parse().unwrap())
        .to(EMAIL_TO.parse().unwrap())
        .subject("icc err report")
        .date_now()
        .body(m)?;

    let mailer = SmtpTransport::relay("smtp.yandex.ru")
        .expect("could not connect to smtp")
        .credentials(Credentials::new("inspired2@yandex.ru".into(), "gxxwwnmetadyjtfh".into()))
        .port(465)
        .build();
        
    match mailer.send(&email) {
        Err(_e) => Err(custom_err::from("error sending email")),
        Ok(_r) => Ok(())
    }
}