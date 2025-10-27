use crate::Error;
use crate::purchase_list::PurchaseList;
use jiff::civil::Date;
use scraper::{Html, Selector};
use ureq::Agent;
use ureq::http::StatusCode;

const DOMAIN: &str = "http://192.168.41.123:90/";
const LOGIN_URL: &str = "http://192.168.41.123:90/login";
const PURCHASE_LIST_API_URL: &str = "http://192.168.41.123:90/backstage/purchase/list/search";

pub struct Client {
    agent: Agent,
    username: String,
    password: String,
    csrf_token: Option<String>,
}

impl Client {
    pub fn new(agent: Agent, username: String, password: String) -> Self {
        Self {
            agent,
            username,
            password,
            csrf_token: None,
        }
    }

    pub fn get_purchase_list(
        &mut self,
        start_date: &Date,
        end_date: &Date,
    ) -> Result<PurchaseList, Error> {
        self.refresh_login_status()?;

        Ok(self
            .agent
            .post(PURCHASE_LIST_API_URL)
            .send_form([
                ("_token", self.csrf_token.as_ref().unwrap().clone()),
                ("startday", start_date.strftime("%Y%m%d").to_string()),
                ("endday", end_date.strftime("%Y%m%d").to_string()),
            ])?
            .body_mut()
            .read_json()?)
    }

    fn refresh_login_status(&mut self) -> Result<bool, Error> {
        // Check whether "XSRF-TOKEN", and "laravel_session" cookies and csrf_token are set and not expired
        let is_cookie_expired;
        let cookie_jar = self.agent.cookie_jar_lock();
        {
            let xsrf_token = cookie_jar.get(DOMAIN, "/", "XSRF-TOKEN");
            let laravel_session = cookie_jar.get(DOMAIN, "/", "laravel_session");
            is_cookie_expired = xsrf_token.is_none() || laravel_session.is_none();
        }
        cookie_jar.release();
        if is_cookie_expired || self.csrf_token.is_none() {
            // Login expired, reset csrf_token and perform login again
            self.csrf_token = None;
            self.login()?;

            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn login(&mut self) -> Result<(), Error> {
        let csrf_token = self.get_login_form_with_csrf_token()?;

        // Submit login form
        let response = match self.agent.post(LOGIN_URL).send_form([
            ("_token", csrf_token.clone()),
            ("user_id", self.username.clone()),
            ("password", self.password.clone()),
        ]) {
            Ok(response) => response,
            Err(ureq::Error::StatusCode(status_code)) => {
                return Err(Error::LoginError(status_code));
            }
            Err(e) => return Err(Error::from(e)),
        };
        if response.status() != StatusCode::FOUND {
            return Err(Error::LoginError(response.status().as_u16()));
        }

        // Login success, set csrf_token
        self.csrf_token = Some(csrf_token);

        Ok(())
    }

    fn get_login_form_with_csrf_token(&self) -> Result<String, Error> {
        let mut response = self.agent.get(LOGIN_URL).call()?;
        let html = response.body_mut().read_to_string()?;
        let html = Html::parse_document(&html);
        let selector = Selector::parse(r#"meta[name="csrf-token"]"#).expect("Invalid selector");
        let csrf_token_meta = html
            .select(&selector)
            .next()
            .ok_or(Error::CSRFTokenNotFound)?;
        Ok(csrf_token_meta
            .attr("content")
            .ok_or(Error::CSRFTokenNotFound)?
            .to_string())
    }
}
