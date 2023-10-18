use dotenv;
use http::StatusCode;
use reqwest::{blocking::Client, Error};
use reqwest::header;
use serde_json::json;
use std::env;

struct User {
    name: String,
    email: String,
}

pub fn send_email() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let api_key = env::var("SENDGRID_API_KEY").unwrap();

    let sender = User {
        name: String::from(env::var("SENDER_NAME").unwrap()),
        email: String::from(env::var("SENDER_EMAIL").unwrap()),
    };

    let recipient = User {
        name: String::from(env::var("RECIPIENT_NAME").unwrap()),
        email: String::from(env::var("RECIPIENT_EMAIL").unwrap()),
    };

    let email_template = String::from("<body>
    
    <h1>Email Report</h1>
    
    <img src=\"https://chart.googleapis.com/chart?cht=lc&chd=t:30,10,45,38,25|10,20,10,20,10&chls=2.0,0.0,0.0&chs=400x300&chg=0,20,3,3,10,20&chxt=x,y&chxl=0:|Week1|Week2|Week3|Week4|Week5|1:|0|20|40|60|80|100&chm=o,ff9900,0,-1,10.0|d,ff0000,1,-1,10.0&chco=FFC6A5,DEBDDE&chdl=Click|GRU\" alt=\"Line Graph\">

    <p>This is your performance(response time) report for the website:</p>
    <ul>
        <li>Website response times have </li>
    </ul>

    <p>from <strong><i>notifychecker</i></strong>, made with 💖 by <strong>@serayuta</strong></p>

</body>");

    let body = json!(
        {
            "personalizations": [{
                "to": [{
                    "email": recipient.email,
                    "name": recipient.name
                }],

                "subject": "Website Performance Report"
            }],
            "from": {
                "email": sender.email,
                "name": sender.name
            },
            "subject": "",
            "content": [
                {
                    "type": "text/html",
                    "value": email_template,
                },
            ]
        }
    );

    let client = Client::new()
        .post("https://api.sendgrid.com/v3/mail/send")
        .json(&body)
        .bearer_auth(api_key)
        .header(
            header::CONTENT_TYPE, 
            header::HeaderValue::from_static("application/json")
        );

    let response = client.send()?;

    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => println!("Email sent!"),
        _ => eprintln!(
            "Unable to send your email. Status code was: {}. Body content was: {:?}",
            response.status(),
            response.text()
        ),
    }

    Ok(())
}
