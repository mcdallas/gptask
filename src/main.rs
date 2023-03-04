pub mod gpt;

fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);

    if args.len() == 0 {
        println!("No prompt provided");
        std::process::exit(1);
    }

    let prompt = args.join(" ");
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set the OPENAI_API_KEY environment variable");

    let client = gpt::GPTClient::new(api_key);
    let mut response = client.prompt(prompt).expect("Could not make request to API");

    response.push_str("\n");
    if let Some(r) = response.strip_prefix("\n\n") {
        response = String::from(r);
    }
    println!("{}", response);
}
