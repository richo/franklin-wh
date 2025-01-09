use reqwest::{self, Url};

mod responses;
mod errors;

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

    pub fn iot_user_runtime_datalog(&self) -> Url {
        self.base.join("hes-gateway/terminal/selectIotUserRuntimeDataLog")
            .unwrap() // We know this is a valid suffix
    }

    pub fn login(&self) -> Url {
        unimplemented!()
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

    // pub fn login<'a>(username: &'a str, password: &'a str) -> Result<String, errors::RequestError> {
    //     // TODO(richo) what do we do if someone wants to use a different API base

    // }

    /// Return the current state of charge from 0 to 100
    pub async fn get_state_of_charge(&self) -> Result<f32, errors::RequestError> {
        let response = self.get(&self.api.charge_power_details())
            .await?
            .json::<responses::ChargePowerDetails>()
            .await?;

        response.inner().map(|x| x.batterySoc)
    }

    pub async fn get_iot_user_runtime_datalog(&self) -> Result<responses::IoTUserRuntimeDataLog, errors::RequestError> {
        let response = self.get(&self.api.iot_user_runtime_datalog())
            .await?;

        Ok(response.json::<responses::IoTUserRuntimeDataLog>()
            .await?)
    }


    /// Make a GET request with authentication etc handled for you.
    async fn get(&self, url: &Url) -> Result<reqwest::Response, reqwest::Error> {
        let res = self.client
            .get(url.clone())
            .query(&[("gatewayId", self.gateway.as_str()), ("lang", "en_US")])
            .header("loginToken", format!("APP_ACCOUNT:{}", self.token))
            .send().await?;

        if res.status() != 200 {
            // TODO(richo) Figure out how to deal with this more gracefully
            panic!("FAILED: {:?}", res.text().await);
        }

        Ok(res)
    }
}


