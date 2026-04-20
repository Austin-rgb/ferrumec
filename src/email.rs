use reqwest::Client;
use reqwest::Response;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct EmailAddress {
    email: String,
    name: String,
}

#[derive(Serialize)]
struct Attachment {
    path: String,
    filename: String,
}

#[derive(Serialize)]
struct EmailPayload {
    sender: EmailAddress,
    to: Vec<EmailAddress>,
    subject: String,
    htmlContent: String,
    attachments: Vec<Attachment>,
}
type SendResult = Result<Response, Box<dyn std::error::Error>>;
trait Sender{
    fn send(&self,payload:&EmailPayload)->impl Future<Output=SendResult>;
}

struct Brevo(String);
impl Brevo{
    fn new()->Result<Self,env::VarError>{
        let api_key = env::var("BREVO_API_KEY")?;
        Ok(Brevo(api_key))
    }
}
impl Sender for Brevo{
    async fn send(&self,payload:&EmailPayload)->SendResult{
    let client = Client::new();
    let res = client
        .post("https://api.brevo.com/v3/smtp/email")
        .header("accept", "application/json")
        .header("api-key", self.0.clone())
        .json(payload)
        .send()
        .await?;
        Ok(res)
}
}

struct Resend(String);

impl Resend{
    fn new()->Result<Self,env::VarError>{
        let api_key = env::var("RESEND_API_KEY")?;
        Ok(Resend(api_key))
    }
}

impl Sender for Resend {

    async fn send(&self,payload:&EmailPayload)->SendResult {

        #[derive(Serialize)]
        struct ResendAttachment<'a>{
            path:&'a String,
            filename:&'a String,
        }

        #[derive(Serialize)]
        struct ResendPayload<'a>{
            from:String,
            to:Vec<&'a String>,
            subject:&'a String,
            html:&'a String,
            attachments:Vec<ResendAttachment<'a>>
        }

        let client = Client::new();
        let key = self.0.clone();

        let rp = ResendPayload{
            from:format!("{} <{}>",payload.sender.name,payload.sender.email),
            to:payload.to.iter().map(|t|&t.email).collect(),
            subject:&payload.subject,
            html:&payload.htmlContent,
            attachments:payload.attachments
                .iter()
                .map(|a|ResendAttachment{
                    path:&a.path,
                    filename:&a.filename
                })
                .collect()
        };

            let res = client
                .post("https://api.resend.com/emails")
                .header("Authorization",format!("Bearer {}",key))
                .header("Content-Type","application/json")
                .json(&rp)
                .send()
                .await?;

            Ok(res)
    }
}
