use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::io::Read;
use std::error::Error;
use crate::util::ContextManager;

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const OPENAI_MODEL: &str = "gpt-3.5-turbo";

type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Role{
    user,
    system,
    assistant,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message{
    role: Role,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prompt {
    model: String,
    messages: Vec<Message>,
}

pub struct GPTClient {
    api_key: String,
    url: String,
}

impl GPTClient {
    pub fn new(api_key: String) -> Self {
        GPTClient {
            api_key: api_key,
            url: String::from(OPENAI_API_URL),
        }
    }

    pub fn prompt(&self, prompt: String) -> BoxResult<String> {
        let behavior = String::from("You will not include any boilerplate in your answers");

        let manager = ContextManager::new();

        let mut messages = vec![
            Message{
                role: Role::system,
                content: behavior,
            },
        ];

        messages.extend(manager.read_context());

        let msg = Message{
            role: Role::user,
            content: prompt,
        };

        let question = msg.clone();
        messages.push(msg);


        let p = Prompt {
            model: String::from(OPENAI_MODEL),
            messages,
        };

        let mut auth = String::from("Bearer ");
        auth.push_str(&self.api_key);

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(auth.as_str())?);
        headers.insert("Content-Type", HeaderValue::from_str("application/json")?);

        let body = serde_json::to_string(&p)?;

        let client = Client::new();
        let mut res = client.post(&self.url)
            .body(body)
            .headers(headers)
            .send()?;

        let mut response_body = String::new();
        res.read_to_string(&mut response_body)?;
        let json_object: Value = from_str(&response_body)?;
        let answer = json_object["choices"][0]["message"]["content"].as_str();

        let content = match answer {
            Some(a) => String::from(a),
            None => return Err(format!("JSON parse error: {response_body}").into()),
        };

        let response = Message {
            role: Role::assistant,
            content: content.clone(),
        };

        manager.write_context(&[question, response]);

        Ok(content)
    }
}
