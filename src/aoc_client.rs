use anyhow::{anyhow, Context};
use bytes::Bytes;
use futures_core::Stream;
use reqwest::{
    self,
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder, Url,
};
use secrecy::{ExposeSecret, Secret};
use tokio_stream::StreamExt;

pub struct AocClient {
    client: Client,
    base_url: Url,
}

impl AocClient {
    pub fn new(mut base_url: Url, session_id: Secret<String>) -> anyhow::Result<Self> {
        if base_url.cannot_be_a_base() {
            return Err(anyhow!("base URL is not a valid base"));
        }
        if base_url.path_segments().unwrap().last() != Some("") {
            base_url.path_segments_mut().unwrap().push("");
        }
        let mut headers = HeaderMap::with_capacity(1);
        let mut session_id =
            HeaderValue::from_bytes(format!("session={}", session_id.expose_secret()).as_bytes())
                .map_err(|_| anyhow!("invalid bytes in session ID"))?;
        session_id.set_sensitive(true);
        headers.insert("Cookie", session_id);
        Ok(Self {
            client: ClientBuilder::new()
                .default_headers(headers)
                .build()
                .expect("couldn't initialize HTTP client"),
            base_url,
        })
    }

    pub async fn get_input(
        &self,
        year: i32,
        day: u32,
    ) -> anyhow::Result<impl Stream<Item = anyhow::Result<Bytes>>> {
        // path_segments_mut cannot error because pre-conditions are checked
        // on instantiation
        let mut url = self.base_url.clone();
        url.path_segments_mut().unwrap().extend(&[
            &year.to_string(),
            "day",
            &day.to_string(),
            "input",
        ]);
        Ok(self
            .client
            .get(url)
            .send()
            .await
            .context("HTTP GET")?
            .error_for_status()?
            .bytes_stream()
            .map(|bytes| bytes.context("reading HTTP response")))
    }
}
