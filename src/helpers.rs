use isahc::{http::StatusCode, HttpClient};
use std::fs::File;
use std::io::Write;
use reqwest::{blocking::Client, Error, header};
use serde_json::json;
use tokio::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{prelude::*, TimeZone};
use chrono_tz::Asia::Bangkok;
use csv::Writer;

pub fn is_internet_connected() -> bool {
    let client = match HttpClient::new() {
        Ok(client) => client,
        _ => {
            return false;
        }
    };

    let response = match client.get("https://www.google.com") {
        Ok(response) => response,
        _ => {
            return false;
        }
    };

    response.status() == StatusCode::OK || response.status().is_redirection()
}

pub fn make_env(recipient_name: String, recipient_email: String) {
    // Add your Sendgrid's API key here
    let mut env = "SENDGRID_API_KEY=\"\"
SENDER_NAME=\"\"
SENDER_EMAIL=\"\"
".to_string();

    env.push_str(format!("RECIPIENT_NAME=\"{}\"\n", recipient_name).as_str());
    env.push_str(format!("RECIPIENT_EMAIL=\"{}\"\n", recipient_email).as_str());

    let mut data_file = File::create(".env").expect("Nothing");
    data_file.write(env.as_bytes()).expect("Nothing");
}

use std::env;

pub struct Time {
    real_time: String,
    response_time: i32,
}

pub struct User {
    name: String,
    email: String,
}

pub fn send_email(result: bool) -> Result<(), Error> {
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

    let graph = read_csv();
    let email_template_success = format!("<body>
    
    <h1>Email Report</h1>
    
    <img src=\"{}\">

    <p>This is your performance(response time) report for the website</p>
    <ul>
        <li>Website response times have a good status!</li>
    </ul>

    <p>from <strong><i>notifychecker</i></strong>, made with 💖 by <strong>@serayuta</strong></p>

</body>", graph);

    let email_template_unsuccess = format!("<body>

    <h1>Email Report</h1>

    <p>This is your performance(response time) report for the website</p>

    <ul>
        <li>Website response have a bad status!</li>
    </ul>

    <p>from <strong><i>notifychecker</i></strong>, made with 💖 by <strong>@serayuta</strong></p>

</body>");

    if result == true {
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
                        "value": email_template_success,
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
    }
    else {
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
                        "value": email_template_unsuccess,
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
    }

    Ok(())
}

pub fn read_csv() -> String {
    // Read data from the CSV file and format it
    let mut x_values = Vec::new();
    let mut y_values = Vec::new();

    let reader = csv::Reader::from_path("output/test.csv");
    for result in reader.expect("NOTHING").records() {
        let record = result.unwrap();
        let time = &record[0];
        let response_time: f64 = record[1].parse().unwrap();
        x_values.push(time.to_string());
        y_values.push(response_time);
    }

    // Construct the Google Charts API URL
    let chart_api_url = "https://chart.googleapis.com/chart?";

    // Define chart parameters and data
    let chart_type = "cht=lc"; // Line chart
    let chart_data = format!("chd=t:{}", y_values.iter().map(|&v| v.to_string()).collect::<Vec<String>>().join(","));
    let chart_size = "chs=400x200"; // Chart size
    let chart_labels = format!("chxt=y,x&chxr=0,0,1000&chds=0,1000&chxl=1:|{}", x_values.join("|"));

    // Combine all parameters into the URL
    let url = format!("{}{}&{}&{}&{}", chart_api_url, chart_type, chart_data, chart_size, chart_labels);

    return url;
}

#[tokio::main]
pub async fn check_res(url: String, hour: String, minuite: String) -> bool {
    let interval = Duration::from_secs(1); // Set the interval in seconds (e.g., 60 seconds)

    let mut list_time = Vec::new();

    loop {
        let start_time = Instant::now();

        // Get the current time as a `SystemTime` object.
        let current_time = SystemTime::now();

        // Calculate the duration since the UNIX epoch.
        let duration = current_time.duration_since(UNIX_EPOCH).expect("SystemTime::duration_since failed");

        // Convert the duration to a `DateTime<Utc>` object.
        let utc_time = Utc.timestamp_opt(duration.as_secs() as i64, duration.subsec_nanos());

        // Convert the UTC time to the desired time zone (GMT+7).
        let local_time = utc_time.unwrap().with_timezone(&Bangkok);

        // Format the date and time as a string.
        let formatted_time = local_time.format("%H:%M").to_string();

        if formatted_time == format!("{}:{}", hour, minuite) {
            let _ = write_csv(File::create("output/test.csv").unwrap(), list_time);
            break;
        }

        // Perform an HTTP GET request
        match reqwest::get(url.clone()).await.unwrap().status() {
            StatusCode::OK => {
                if formatted_time.chars().nth(4) == Some('0') || formatted_time.chars().nth(4) == Some('5') {
                    let response_time = start_time.elapsed().as_secs_f32();
                    let time = Time {
                        real_time: formatted_time,
                        response_time: (response_time * 1000.0) as i32,
                    };
                    let mut counter = 0;
                    for i in 0..list_time.len() {
                        if time.real_time == list_time[i].real_time {
                            counter += 1
                        }
                    }
                    if counter == 0 { list_time.push(time); }
                }
            }
            _ => {
                return false;
            }
        }

        // Sleep for the specified interval before the next request
        tokio::time::sleep(interval).await;
    }
    return true;
}

pub fn write_csv<W: Write>(writer: W, list_time: Vec<Time>) -> Result<(), csv::Error> {
    let mut wtr = Writer::from_writer(writer);
    for time in list_time {
        wtr.write_record(&[time.real_time, time.response_time.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}
