use reqwest::{self, Url};

mod responses;

const DEFAULT_URL_BASE: &'static str  = "https://energy.franklinwh.com";

struct Api {
    base: Url,
}

impl Default for Api {
    fn default() -> Self {
        Self::new(DEFAULT_URL_BASE)
            .unwrap() // We know the DEFAULT_URL_BASE will always parse ok
    }
}

impl Api {
    pub fn new<'a>(base: &'a str) -> Result<Self, ()> {
        let base = Url::parse(base).map_err(|_| ())?;
        Ok(Api {
            base,
        })
    }

    pub fn charge_power_details(&self) -> Url {
        self.base.join("hes-gateway/terminal/chargePowerDetails")
            .unwrap() // We know this is a valid suffix
    }
}

pub struct Client {
    api: Api,
    token: String,
    client: reqwest::Client,
    gateway: String,
}

impl Client {
    pub fn new<'a>(token: &'a str, gateway: &'a str) -> Self {
        Client {
            api: Default::default(),
            token: token.into(),
            client: Default::default(),
            gateway: gateway.into(),
        }
    }

    /// Return the current state of charge from 0 to 100
    pub async fn get_state_of_charge(&self) -> Result<f32, ()> {
        let response = self.get(&self.api.charge_power_details())
            .await.map_err(|_| ())?
            .json::<responses::ChargePowerDetails>()
            .await.map_err(|_| ())?;

        return response.inner().map(|x| x.batterySoc);
    }

    /// Make a GET request with authentication etc handled for you.
    async fn get(&self, url: &Url) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .get(url.clone())
            .query(&[("gatewayId", &self.gateway)])
            .header("loginToken", format!("APP_ACCOUNT:{}", self.token))
            .send().await
    }
}

