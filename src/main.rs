extern crate base64;
extern crate hyper;
extern crate hyper_rustls;
extern crate google_pubsub1 as pubsub;
extern crate yup_oauth2 as oauth;

use std::default::Default;
use std::env;

use hyper::net::HttpsConnector;

const SUBSCRIPTION_NAME: &'static str = "projects/YOUR_PROJECT_ID/subscriptions/YOUR_SUBSCRIPTION";
const TOPIC_NAME: &'static str = "projects/YOUR_PROJECT_ID/topics/YOUR_TOPIC";

type PubsubMethods<'a> = pubsub::ProjectMethods<'a,
                                                hyper::Client,
                                                oauth::ServiceAccountAccess<hyper::Client>>;

fn publish(methods: &PubsubMethods, message: &str) {

    let message = pubsub::PubsubMessage {
        data: Some(base64::encode(message.as_bytes())),
        ..Default::default()
    };
    let request = pubsub::PublishRequest { messages: Some(vec![message]) };
    let result = methods.topics_publish(request.clone(), TOPIC_NAME).doit();

    match result {
        Err(e) => {
            println!("Publish error: {}", e);
        }
        Ok((_response, response)) => {
            for msg in response.message_ids.unwrap_or(Vec::new()) {
                println!("Published message #{}", msg);
            }
        }
    }
}

fn subscribe(methods: &PubsubMethods) {
    let request = pubsub::PullRequest {
        return_immediately: Some(false),
        max_messages: Some(10),
    };
    loop {
        let result = methods.subscriptions_pull(request.clone(), SUBSCRIPTION_NAME).doit();
        match result {
            Err(e) => {
                println!("Pull error: {}", e);
            }
            Ok((_response, response)) => {
                for msg in response.received_messages.unwrap_or(Vec::new()) {
                    let ack_id = msg.ack_id.unwrap_or(String::new());
                    let message = msg.message.unwrap_or(Default::default());
                    println!("message <{}> '{}' at {}",
                        message.message_id.unwrap_or(String::new()),
                        String::from_utf8(base64::decode(&message.data.unwrap_or(String::new())).unwrap()).unwrap(),
                        message.publish_time.unwrap_or(String::new())
                    );

                    if ack_id != "" {
                        let request = pubsub::AcknowledgeRequest { ack_ids: Some(vec![ack_id]) };
                        let result = methods.subscriptions_acknowledge(request, SUBSCRIPTION_NAME).doit();

                        match result {
                            Err(e) => {
                                println!("Ack error: {:?}", e);
                            }
                            Ok(_) => ()
                        }
                    }
                }
            }
        }
    }
}

fn main() {

    let client_secret = oauth::service_account_key_from_file(&"auth.json".to_string()).unwrap();
    let client = hyper::Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));

    let mut access = oauth::ServiceAccountAccess::new(client_secret, client);

    use oauth::GetToken;
    println!("{:?}",
             access.token(&vec!["https://www.googleapis.com/auth/pubsub"]).unwrap());

    let client = hyper::Client::with_connector(HttpsConnector::new(hyper_rustls::TlsClient::new()));
    let hub = pubsub::Pubsub::new(client, access);
    let methods = hub.projects();

    let args: Vec<String> = env::args().skip(1).collect();

    if args[0] == "pub" {
        let message = "pubsub test by rust.".to_string();
        publish(&methods, &message);
    } else if args[0] == "sub" {
        subscribe(&methods);
    }


    println!("completed {:?}.", args[0]);
}
