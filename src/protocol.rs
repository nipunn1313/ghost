use anyhow::Result;
use futures::TryFutureExt;
use reqwest::{Client, Response};
use url::Url;

mod parser;

pub struct GitProtocol {
    url: Url,
    client: Client,
}

impl GitProtocol {
    pub fn new(mut url: String) -> GitProtocol {
        if !url.ends_with("/") {
            url = url + "/";
        }
        let url = Url::parse(&url).unwrap_or_else(|e| panic!("Could not parse {}: {}", url, e));
        GitProtocol {
            url,
            client: Client::new(),
        }
    }

    async fn call(&self, path: &str) -> Result<Response> {
        let url = self.url.join(path).unwrap();
        println!("Calling {:?}", url.as_str());
        Ok(self
            .client
            .get(url.as_str())
            .send()
            .inspect_ok(move |r| {
                println!("{:?} returned {:?}", url.as_str(), r.status());
            })
            .await?)
    }

    pub async fn refs(&self) -> Result<String> {
        let response = self.call("info/refs?service=git-upload-pack").await.unwrap();
        assert!(response.status() == 200); // is there a better way?

        let content = response.bytes().await?;
        println!("{:?}", content);

        //let first_line = hexadecimal_value(content).unwrap();
        //println!("first_line = {:?}", first_line);
        //Ok(first_line)

        Ok("hi".into())
    }
}
